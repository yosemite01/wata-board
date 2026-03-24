#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{symbol_short, testutils::{Address as TestAddress, Ledger as TestLedger}, Env};

    fn create_test_env() -> Env {
        let env = Env::default();
        env.mock_all_auths();
        env
    }

    #[test]
    fn test_initialize_contract() {
        let env = create_test_env();
        let admin = TestAddress::generate(&env);
        
        // Test successful initialization
        NepaBillingContract::initialize(env.clone(), admin.clone());
        
        let retrieved_admin = NepaBillingContract::get_admin(env.clone());
        assert_eq!(retrieved_admin, admin);
        
        // Test double initialization fails
        let result = std::panic::catch_unwind(|| {
            NepaBillingContract::initialize(env.clone(), admin.clone());
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_payment_id_generation() {
        let env = create_test_env();
        let admin = TestAddress::generate(&env);
        let user = TestAddress::generate(&env);
        let token_address = TestAddress::generate(&env);
        
        // Initialize contract
        NepaBillingContract::initialize(env.clone(), admin.clone());
        
        let meter_id = String::from_str(&env, "METER-001");
        
        // Make multiple payments
        let payment1 = NepaBillingContract::pay_bill(
            env.clone(), 
            user.clone(), 
            token_address.clone(), 
            meter_id.clone(), 
            1000
        );
        
        let payment2 = NepaBillingContract::pay_bill(
            env.clone(), 
            user.clone(), 
            token_address.clone(), 
            meter_id.clone(), 
            2000
        );
        
        // Verify payment IDs are sequential and unique
        assert_eq!(payment1, 1);
        assert_eq!(payment2, 2);
        assert_ne!(payment1, payment2);
    }

    #[test]
    fn test_pay_bill_creates_payment_record() {
        let env = create_test_env();
        let admin = TestAddress::generate(&env);
        let user = TestAddress::generate(&env);
        let token_address = TestAddress::generate(&env);
        
        // Initialize contract
        NepaBillingContract::initialize(env.clone(), admin.clone());
        
        let meter_id = String::from_str(&env, "METER-001");
        let amount = 1000i128;
        
        // Make a payment
        let payment_id = NepaBillingContract::pay_bill(
            env.clone(), 
            user.clone(), 
            token_address.clone(), 
            meter_id.clone(), 
            amount
        );
        
        // Verify payment record was created
        let record = NepaBillingContract::get_payment_record(env.clone(), payment_id);
        assert_eq!(record.payer, user);
        assert_eq!(record.amount, amount);
        assert_eq!(record.meter_id, meter_id);
        assert!(!record.refunded);
        
        // Verify meter total was updated
        let total = NepaBillingContract::get_total_paid(env.clone(), meter_id);
        assert_eq!(total, amount);
    }

    #[test]
    fn test_admin_refund_happy_path() {
        let env = create_test_env();
        let admin = TestAddress::generate(&env);
        let user = TestAddress::generate(&env);
        let token_address = TestAddress::generate(&env);
        
        // Initialize contract
        NepaBillingContract::initialize(env.clone(), admin.clone());
        
        let meter_id = String::from_str(&env, "METER-001");
        let amount = 1000i128;
        
        // Make a payment
        let payment_id = NepaBillingContract::pay_bill(
            env.clone(), 
            user.clone(), 
            token_address.clone(), 
            meter_id.clone(), 
            amount
        );
        
        // Verify payment exists and is not refunded
        let record_before = NepaBillingContract::get_payment_record(env.clone(), payment_id);
        assert!(!record_before.refunded);
        
        // Admin refunds the payment
        NepaBillingContract::admin_refund(
            env.clone(), 
            admin.clone(), 
            token_address.clone(), 
            payment_id
        );
        
        // Verify payment is marked as refunded
        let record_after = NepaBillingContract::get_payment_record(env.clone(), payment_id);
        assert!(record_after.refunded);
        
        // Verify meter total was updated (refunded amount subtracted)
        let total = NepaBillingContract::get_total_paid(env.clone(), meter_id);
        assert_eq!(total, 0);
    }

    #[test]
    fn test_unauthorized_refund_fails() {
        let env = create_test_env();
        let admin = TestAddress::generate(&env);
        let user = TestAddress::generate(&env);
        let token_address = TestAddress::generate(&env);
        
        // Initialize contract
        NepaBillingContract::initialize(env.clone(), admin.clone());
        
        let meter_id = String::from_str(&env, "METER-001");
        let amount = 1000i128;
        
        // Make a payment
        let payment_id = NepaBillingContract::pay_bill(
            env.clone(), 
            user.clone(), 
            token_address.clone(), 
            meter_id.clone(), 
            amount
        );
        
        // Try to refund with unauthorized user (should fail)
        let unauthorized_user = TestAddress::generate(&env);
        let result = std::panic::catch_unwind(|| {
            NepaBillingContract::admin_refund(
                env.clone(), 
                unauthorized_user, 
                token_address.clone(), 
                payment_id
            );
        });
        assert!(result.is_err());
        
        // Verify payment is still not refunded
        let record = NepaBillingContract::get_payment_record(env.clone(), payment_id);
        assert!(!record.refunded);
    }

    #[test]
    fn test_double_refund_fails() {
        let env = create_test_env();
        let admin = TestAddress::generate(&env);
        let user = TestAddress::generate(&env);
        let token_address = TestAddress::generate(&env);
        
        // Initialize contract
        NepaBillingContract::initialize(env.clone(), admin.clone());
        
        let meter_id = String::from_str(&env, "METER-001");
        let amount = 1000i128;
        
        // Make a payment
        let payment_id = NepaBillingContract::pay_bill(
            env.clone(), 
            user.clone(), 
            token_address.clone(), 
            meter_id.clone(), 
            amount
        );
        
        // First refund (should succeed)
        NepaBillingContract::admin_refund(
            env.clone(), 
            admin.clone(), 
            token_address.clone(), 
            payment_id
        );
        
        // Second refund (should fail)
        let result = std::panic::catch_unwind(|| {
            NepaBillingContract::admin_refund(
                env.clone(), 
                admin.clone(), 
                token_address.clone(), 
                payment_id
            );
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_refund_nonexistent_payment_fails() {
        let env = create_test_env();
        let admin = TestAddress::generate(&env);
        let token_address = TestAddress::generate(&env);
        
        // Initialize contract
        NepaBillingContract::initialize(env.clone(), admin.clone());
        
        // Try to refund non-existent payment
        let result = std::panic::catch_unwind(|| {
            NepaBillingContract::admin_refund(
                env.clone(), 
                admin.clone(), 
                token_address.clone(), 
                999 // Non-existent payment ID
            );
        });
        assert!(result.is_err());
    }
}
