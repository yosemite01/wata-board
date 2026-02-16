#![no_std]
// We added 'Address' and 'token' to the imports
use soroban_sdk::{contract, contractimpl, Address, Env, String, token};

#[contract]
pub struct NepaBillingContract;

#[contractimpl]
impl NepaBillingContract {
    
    pub fn pay_bill(env: Env, from: Address, token_address: Address, meter_id: String, amount: i128) {
        // 1. Verify the user authorized this payment
        from.require_auth();

        // 2. Initialize the Token client (for XLM or USDC)
        let token_client = token::Client::new(&env, &token_address);

        // 3. Move the tokens from the User to the Contract
        token_client.transfer(&from, &env.current_contract_address(), &amount);

        // 4. Update the meter record (using i128 for larger money values)
        let current_total: i128 = env.storage().persistent().get(&meter_id).unwrap_or(0);
        env.storage().persistent().set(&meter_id, &(current_total + amount));
    }

    pub fn get_total_paid(env: Env, meter_id: String) -> i128 {
        env.storage().persistent().get(&meter_id).unwrap_or(0)
    }
}