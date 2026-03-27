#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::{Address as TestAddress}, Env, String};

    fn setup_test() -> (Env, NepaBillingContractClient, Address) {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, NepaBillingContract);
        let client = NepaBillingContractClient::new(&env, &contract_id);
        
        let admin = TestAddress::generate(&env);
        (env, client, admin)
    }

    #[test]
    fn test_initialize_contract() {
        let (env, client, admin) = setup_test();
        
        // Test successful initialization
        client.initialize(&admin);
        
        let retrieved_admin = client.get_admin();
        assert_eq!(retrieved_admin, admin);
        
        // Test double initialization fails
        let result = std::panic::catch_unwind(|| {
            client.initialize(&admin);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_pay_bill_and_records() {
        let (env, client, admin) = setup_test();
        client.initialize(&admin);
        
        let user = TestAddress::generate(&env);
        let token_address = TestAddress::generate(&env);
        let meter_id = String::from_str(&env, "METER-001");
        let amount = 1000i128;
        
        // Payment 1
        let payment1 = client.pay_bill(&user, &token_address, &meter_id, &amount);
        assert_eq!(payment1, 1);
        
        // Verify record
        let record = client.get_payment_record(&payment1);
        assert_eq!(record.payer, user);
        assert_eq!(record.amount, amount);
        
        // Verify total
        assert_eq!(client.get_total_paid(&meter_id), amount);
    }

    #[test]
    fn test_admin_refund_and_security() {
        let (env, client, admin) = setup_test();
        client.initialize(&admin);
        
        let user = TestAddress::generate(&env);
        let token_address = TestAddress::generate(&env);
        let meter_id = String::from_str(&env, "METER-001");
        
        // Payment
        let payment_id = client.pay_bill(&user, &token_address, &meter_id, &1000);
        
        // 1. Unauthorized refund fails
        let non_admin = TestAddress::generate(&env);
        let result = std::panic::catch_unwind(|| {
            client.admin_refund(&non_admin, &token_address, &payment_id);
        });
        assert!(result.is_err());
        
        // 2. Happy path refund
        client.admin_refund(&admin, &token_address, &payment_id);
        
        let record = client.get_payment_record(&payment_id);
        assert!(record.refunded);
        assert_eq!(client.get_total_paid(&meter_id), 0);
        
        // 3. Double refund fails
        let result2 = std::panic::catch_unwind(|| {
            client.admin_refund(&admin, &token_address, &payment_id);
        });
        assert!(result2.is_err());
    }

    #[test]
    fn test_review_system_happy_path() {
        let (env, client, admin) = setup_test();
        client.initialize(&admin);
        
        let user = TestAddress::generate(&env);
        let rating = 5;
        let comment = String::from_str(&env, "Great!");
        let tx_hash = String::from_str(&env, "h-123");
        
        client.submit_review(&user, &rating, &comment, &tx_hash);
        
        let review = client.get_user_review(&user).unwrap();
        assert_eq!(review.rating, rating);
        assert_eq!(review.comment, comment);
        
        let stats = client.get_rating_stats();
        assert_eq!(stats.total_reviews, 1);
        assert_eq!(stats.average_rating, 50);
    }

    #[test]
    fn test_rating_stats_calculation() {
        let (env, client, admin) = setup_test();
        client.initialize(&admin);
        
        // Submit 5-star and 3-star
        client.submit_review(&TestAddress::generate(&env), &5, &String::from_str(&env, "A"), &String::from_str(&env, "h1"));
        client.submit_review(&TestAddress::generate(&env), &3, &String::from_str(&env, "B"), &String::from_str(&env, "h2"));
        
        // Avg = (5+3)/2 = 4.0 -> 40
        let stats = client.get_rating_stats();
        assert_eq!(stats.total_reviews, 2);
        assert_eq!(stats.average_rating, 40);
        assert_eq!(stats.rating_counts.get(2).unwrap(), 1); // 3-star index 2
        assert_eq!(stats.rating_counts.get(4).unwrap(), 1); // 5-star index 4
    }

    #[test]
    fn test_invalid_review_inputs() {
        let (env, client, admin) = setup_test();
        client.initialize(&admin);
        let user = TestAddress::generate(&env);
        
        // 1. Rating too high
        let res1 = std::panic::catch_unwind(|| {
            client.submit_review(&user, &6, &String::from_str(&env, "X"), &String::from_str(&env, "H"));
        });
        assert!(res1.is_err());
        
        // 2. Rating too low
        let res2 = std::panic::catch_unwind(|| {
            client.submit_review(&user, &0, &String::from_str(&env, "X"), &String::from_str(&env, "H"));
        });
        assert!(res2.is_err());
        
        // 3. Comment too long (limit is 500 characters)
        let long_comment = String::from_str(&env, "A").repeat(501);
        let res3 = std::panic::catch_unwind(|| {
             client.submit_review(&user, &5, &long_comment, &String::from_str(&env, "H"));
        });
        assert!(res3.is_err());
    }

    #[test]
    fn test_duplicate_review_prevention() {
        let (env, client, admin) = setup_test();
        client.initialize(&admin);
        let user = TestAddress::generate(&env);
        
        client.submit_review(&user, &5, &String::from_str(&env, "First"), &String::from_str(&env, "H1"));
        
        let res = std::panic::catch_unwind(|| {
            client.submit_review(&user, &4, &String::from_str(&env, "Second"), &String::from_str(&env, "H2"));
        });
        assert!(res.is_err());
    }

    #[test]
    fn test_verify_review_logic() {
        let (env, client, admin) = setup_test();
        client.initialize(&admin);
        let user = TestAddress::generate(&env);
        let hash = String::from_str(&env, "hash-001");
        
        client.submit_review(&user, &5, &String::from_str(&env, "Test"), &hash);
        
        assert!(client.verify_review(&user, &hash));
        assert!(!client.verify_review(&user, &String::from_str(&env, "wrong-hash")));
    }

    // ==================== REFUND MECHANISM TESTS ====================

    #[test]
    fn test_refund_request_creation() {
        let (env, client, admin) = setup_test();
        client.initialize(&admin);
        
        let user = TestAddress::generate(&env);
        let token_address = TestAddress::generate(&env);
        let meter_id = String::from_str(&env, "METER-001");
        let amount = 1000i128;
        
        // Make a payment
        let payment_id = client.pay_bill(&user, &token_address, &meter_id, &amount);
        
        // Request refund
        let reason = String::from_str(&env, "wrong_meter_id");
        let request_id = client.request_refund(&user, &payment_id, &reason);
        
        assert_eq!(request_id, 1);
        
        // Verify request details
        let request = client.get_refund_request(&request_id);
        assert_eq!(request.payment_id, payment_id);
        assert_eq!(request.payer, user);
        assert_eq!(request.amount, amount);
        assert_eq!(request.reason, reason);
        assert_eq!(request.status, String::from_str(&env, "pending"));
    }

    #[test]
    fn test_refund_request_invalid_reason() {
        let (env, client, admin) = setup_test();
        client.initialize(&admin);
        
        let user = TestAddress::generate(&env);
        let token_address = TestAddress::generate(&env);
        let meter_id = String::from_str(&env, "METER-001");
        
        let payment_id = client.pay_bill(&user, &token_address, &meter_id, &1000);
        
        // Try invalid reason
        let invalid_reason = String::from_str(&env, "invalid_reason");
        let result = std::panic::catch_unwind(|| {
            client.request_refund(&user, &payment_id, &invalid_reason);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_refund_request_wrong_payer() {
        let (env, client, admin) = setup_test();
        client.initialize(&admin);
        
        let user = TestAddress::generate(&env);
        let token_address = TestAddress::generate(&env);
        let meter_id = String::from_str(&env, "METER-001");
        
        let payment_id = client.pay_bill(&user, &token_address, &meter_id, &1000);
        
        // Different user tries to request refund
        let other_user = TestAddress::generate(&env);
        let reason = String::from_str(&env, "wrong_meter_id");
        let result = std::panic::catch_unwind(|| {
            client.request_refund(&other_user, &payment_id, &reason);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_single_approval_refund() {
        let (env, client, admin) = setup_test();
        client.initialize(&admin);
        
        let user = TestAddress::generate(&env);
        let token_address = TestAddress::generate(&env);
        let meter_id = String::from_str(&env, "METER-001");
        let amount = 1000i128;
        
        // Make payment and request refund
        let payment_id = client.pay_bill(&user, &token_address, &meter_id, &amount);
        let reason = String::from_str(&env, "duplicate_payment");
        let request_id = client.request_refund(&user, &payment_id, &reason);
        
        // Admin approves
        client.approve_refund(&admin, &request_id);
        
        // Check request is approved
        let request = client.get_refund_request(&request_id);
        assert_eq!(request.status, String::from_str(&env, "approved"));
        assert_eq!(request.approvers.len(), 1);
        
        // Complete refund
        client.complete_refund(&admin, &token_address, &request_id);
        
        // Verify completion
        let completed_request = client.get_refund_request(&request_id);
        assert_eq!(completed_request.status, String::from_str(&env, "completed"));
        
        let payment_record = client.get_payment_record(&payment_id);
        assert!(payment_record.refunded);
        assert_eq!(client.get_total_paid(&meter_id), 0);
    }

    #[test]
    fn test_multi_signature_approval() {
        let (env, client, admin) = setup_test();
        client.initialize(&admin);
        
        let approver1 = TestAddress::generate(&env);
        let approver2 = TestAddress::generate(&env);
        let mut approvers = Vec::new(&env);
        approvers.push_back(approver1.clone());
        approvers.push_back(approver2.clone());
        
        // Set 2 approvals required
        client.set_refund_approvers(&admin, &approvers, &2);
        
        let user = TestAddress::generate(&env);
        let token_address = TestAddress::generate(&env);
        let meter_id = String::from_str(&env, "METER-001");
        let amount = 2000i128;
        
        let payment_id = client.pay_bill(&user, &token_address, &meter_id, &amount);
        let reason = String::from_str(&env, "user_error");
        let request_id = client.request_refund(&user, &payment_id, &reason);
        
        // First approval - should still be pending
        client.approve_refund(&approver1, &request_id);
        let request = client.get_refund_request(&request_id);
        assert_eq!(request.status, String::from_str(&env, "pending"));
        assert_eq!(request.approvers.len(), 1);
        
        // Second approval - should be approved
        client.approve_refund(&approver2, &request_id);
        let request = client.get_refund_request(&request_id);
        assert_eq!(request.status, String::from_str(&env, "approved"));
        assert_eq!(request.approvers.len(), 2);
    }

    #[test]
    fn test_double_approval_prevention() {
        let (env, client, admin) = setup_test();
        client.initialize(&admin);
        
        let user = TestAddress::generate(&env);
        let token_address = TestAddress::generate(&env);
        let meter_id = String::from_str(&env, "METER-001");
        
        let payment_id = client.pay_bill(&user, &token_address, &meter_id, &1000);
        let reason = String::from_str(&env, "incorrect_amount");
        let request_id = client.request_refund(&user, &payment_id, &reason);
        
        // Admin approves
        client.approve_refund(&admin, &request_id);
        
        // Try to approve again
        let result = std::panic::catch_unwind(|| {
            client.approve_refund(&admin, &request_id);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_refund_rejection() {
        let (env, client, admin) = setup_test();
        client.initialize(&admin);
        
        let user = TestAddress::generate(&env);
        let token_address = TestAddress::generate(&env);
        let meter_id = String::from_str(&env, "METER-001");
        
        let payment_id = client.pay_bill(&user, &token_address, &meter_id, &1000);
        let reason = String::from_str(&env, "wrong_meter_id");
        let request_id = client.request_refund(&user, &payment_id, &reason);
        
        // Reject refund
        let rejection_reason = String::from_str(&env, "Meter ID was actually correct");
        client.reject_refund(&admin, &request_id, &rejection_reason);
        
        // Verify rejection
        let request = client.get_refund_request(&request_id);
        assert_eq!(request.status, String::from_str(&env, "rejected"));
        assert_eq!(request.rejection_reason, rejection_reason);
        
        // Payment should NOT be refunded
        let payment_record = client.get_payment_record(&payment_id);
        assert!(!payment_record.refunded);
    }

    #[test]
    fn test_unauthorized_refund_approval() {
        let (env, client, admin) = setup_test();
        client.initialize(&admin);
        
        let user = TestAddress::generate(&env);
        let token_address = TestAddress::generate(&env);
        let meter_id = String::from_str(&env, "METER-001");
        
        let payment_id = client.pay_bill(&user, &token_address, &meter_id, &1000);
        let reason = String::from_str(&env, "duplicate_payment");
        let request_id = client.request_refund(&user, &payment_id, &reason);
        
        // Non-approver tries to approve
        let unauthorized = TestAddress::generate(&env);
        let result = std::panic::catch_unwind(|| {
            client.approve_refund(&unauthorized, &request_id);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_payer_refund_requests_list() {
        let (env, client, admin) = setup_test();
        client.initialize(&admin);
        
        let user = TestAddress::generate(&env);
        let token_address = TestAddress::generate(&env);
        let meter_id1 = String::from_str(&env, "METER-001");
        let meter_id2 = String::from_str(&env, "METER-002");
        
        // Make two payments
        let payment_id1 = client.pay_bill(&user, &token_address, &meter_id1, &1000);
        let payment_id2 = client.pay_bill(&user, &token_address, &meter_id2, &2000);
        
        // Request refunds for both
        client.request_refund(&user, &payment_id1, &String::from_str(&env, "wrong_meter_id"));
        client.request_refund(&user, &payment_id2, &String::from_str(&env, "incorrect_amount"));
        
        // Get all requests for this payer
        let requests = client.get_payer_refund_requests(&user);
        assert_eq!(requests.len(), 2);
    }

    #[test]
    fn test_refund_prevents_already_refunded() {
        let (env, client, admin) = setup_test();
        client.initialize(&admin);
        
        let user = TestAddress::generate(&env);
        let token_address = TestAddress::generate(&env);
        let meter_id = String::from_str(&env, "METER-001");
        
        let payment_id = client.pay_bill(&user, &token_address, &meter_id, &1000);
        let reason = String::from_str(&env, "wrong_meter_id");
        let request_id = client.request_refund(&user, &payment_id, &reason);
        
        // Approve and complete
        client.approve_refund(&admin, &request_id);
        client.complete_refund(&admin, &token_address, &request_id);
        
        // Try to request refund for already refunded payment
        let reason2 = String::from_str(&env, "another_reason");
        let result = std::panic::catch_unwind(|| {
            client.request_refund(&user, &payment_id, &reason2);
        });
        assert!(result.is_err());
    }
}
