#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String, token, Symbol, Vec, Map};

#[contract]
pub struct NepaBillingContract;

// Payment record structure to track all payment details
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct PaymentRecord {
    pub payer: Address,
    pub amount: i128,
    pub meter_id: String,
    pub timestamp: u64,
    pub refunded: bool,
}

// Storage keys
const ADMIN_KEY: Symbol = Symbol::short("ADMIN");
const PAYMENT_COUNTER: Symbol = Symbol::short("PAY_CNT");
const PAYMENT_RECORDS: Symbol = Symbol::short("PAY_REC");

#[contractimpl]
impl NepaBillingContract {
    
    /// Initialize the contract with an admin address
    pub fn initialize(env: Env, admin: Address) {
        // Only allow initialization once
        if env.storage().persistent().has(&ADMIN_KEY) {
            panic!("Contract already initialized");
        }
        
        env.storage().persistent().set(&ADMIN_KEY, &admin);
        env.storage().persistent().set(&PAYMENT_COUNTER, &0u64);
    }
    
    /// Get the contract admin
    pub fn get_admin(env: Env) -> Address {
        env.storage().persistent().get(&ADMIN_KEY)
            .unwrap_or_else(|| panic!("Contract not initialized"))
    }
    
    pub fn pay_bill(env: Env, from: Address, token_address: Address, meter_id: String, amount: i128) -> u64 {
        // 1. Verify the user authorized this payment
        from.require_auth();

        // 2. Initialize the Token client
        let token_client = token::Client::new(&env, &token_address);

        // 3. Move the tokens from the User to the Contract
        token_client.transfer(&from, &env.current_contract_address(), &amount);

        // 4. Create payment record
        let payment_id = Self::_generate_payment_id(&env);
        let timestamp = env.ledger().timestamp();
        
        let payment_record = PaymentRecord {
            payer: from.clone(),
            amount,
            meter_id: meter_id.clone(),
            timestamp,
            refunded: false,
        };

        // 5. Store payment record
        env.storage().persistent().set(&payment_id, &payment_record);

        // 6. Update the meter total (backward compatibility)
        let current_total: i128 = env.storage().persistent().get(&meter_id).unwrap_or(0);
        env.storage().persistent().set(&meter_id, &(current_total + amount));

        payment_id
    }

    pub fn get_total_paid(env: Env, meter_id: String) -> i128 {
        env.storage().persistent().get(&meter_id).unwrap_or(0)
    }
    
    /// Get payment record by ID
    pub fn get_payment_record(env: Env, payment_id: u64) -> PaymentRecord {
        env.storage().persistent().get(&payment_id)
            .unwrap_or_else(|| panic!("Payment record not found"))
    }
    
    /// Get all payment records for a specific payer
    pub fn get_payer_payments(env: Env, payer: Address) -> Vec<u64> {
        let counter: u64 = env.storage().persistent().get(&PAYMENT_COUNTER).unwrap_or(0);
        let mut payment_ids = Vec::new(&env);
        
        for i in 0..counter {
            let payment_id = i + 1; // Payment IDs start from 1
            if let Some(record) = env.storage().persistent().get::<u64, PaymentRecord>(&payment_id) {
                if record.payer == payer {
                    payment_ids.push_back(payment_id);
                }
            }
        }
        
        payment_ids
    }
    
    /// Admin-only refund function with security protections
    pub fn admin_refund(env: Env, admin: Address, token_address: Address, payment_id: u64) {
        // 1. Verify admin authorization
        admin.require_auth();
        
        // 2. Verify caller is the contract admin
        let contract_admin = Self::get_admin(env.clone());
        if admin != contract_admin {
            panic!("Only admin can initiate refunds");
        }
        
        // 3. Get payment record
        let mut payment_record = Self::get_payment_record(env.clone(), payment_id);
        
        // 4. Check if already refunded (prevent double-spending)
        if payment_record.refunded {
            panic!("Payment already refunded");
        }
        
        // 5. Mark as refunded BEFORE transferring (re-entrancy protection)
        payment_record.refunded = true;
        env.storage().persistent().set(&payment_id, &payment_record);
        
        // 6. Update meter total (subtract refunded amount)
        let current_total: i128 = env.storage().persistent().get(&payment_record.meter_id).unwrap_or(0);
        let new_total = current_total - payment_record.amount;
        env.storage().persistent().set(&payment_record.meter_id, &new_total);
        
        // 7. Transfer tokens back to payer
        let token_client = token::Client::new(&env, &token_address);
        token_client.transfer(&env.current_contract_address(), &payment_record.payer, &payment_record.amount);
    }
    
    /// Helper function to generate unique payment IDs
    fn _generate_payment_id(env: &Env) -> u64 {
        let counter: u64 = env.storage().persistent().get(&PAYMENT_COUNTER).unwrap_or(0);
        let new_id = counter + 1;
        env.storage().persistent().set(&PAYMENT_COUNTER, &new_id);
        new_id
    }
}

// Mock token contract for testing
#[cfg(test)]
pub mod token {
    use super::*;
    use soroban_sdk::contractimpl;

    pub struct MockTokenContract;

    #[contractimpl]
    impl MockTokenContract {
        pub fn mint(env: Env, to: Address, amount: i128) {
            // Simple mock implementation - in real tests this would be handled by testutils
        }
        
        pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
            // Simple mock implementation
        }
    }
}

#[cfg(test)]
mod tests;