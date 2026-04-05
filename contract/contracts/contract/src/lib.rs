#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env, Symbol, Address, Map, Vec,
};

#[contracttype]
#[derive(Clone)]
pub struct Subscription {
    pub subscriber: Address,
    pub merchant: Address,
    pub amount: i128,
    pub interval: u64, // seconds
    pub last_payment: u64,
    pub active: bool,
}

#[contract]
pub struct SubscriptionContract;

#[contractimpl]
impl SubscriptionContract {

    // Create a new subscription
    pub fn create_subscription(
        env: Env,
        subscriber: Address,
        merchant: Address,
        amount: i128,
        interval: u64,
    ) {
        subscriber.require_auth();

        let timestamp = env.ledger().timestamp();

        let sub = Subscription {
            subscriber: subscriber.clone(),
            merchant,
            amount,
            interval,
            last_payment: timestamp,
            active: true,
        };

        env.storage().instance().set(&subscriber, &sub);
    }

    // Pay subscription (can be triggered manually or via cron-like service)
    pub fn pay(env: Env, subscriber: Address) {
        let mut sub: Subscription = env
            .storage()
            .instance()
            .get(&subscriber)
            .expect("No subscription found");

        if !sub.active {
            panic!("Subscription inactive");
        }

        let current_time = env.ledger().timestamp();

        if current_time < sub.last_payment + sub.interval {
            panic!("Too early to pay");
        }

        sub.subscriber.require_auth();

        // Token transfer (native XLM or token contract assumed)
        let token_client = soroban_sdk::token::Client::new(
            &env,
            &env.current_contract_address(),
        );

        token_client.transfer(
            &sub.subscriber,
            &sub.merchant,
            &sub.amount,
        );

        sub.last_payment = current_time;

        env.storage().instance().set(&subscriber, &sub);
    }

    // Cancel subscription
    pub fn cancel(env: Env, subscriber: Address) {
        let mut sub: Subscription = env
            .storage()
            .instance()
            .get(&subscriber)
            .expect("No subscription");

        sub.subscriber.require_auth();
        sub.active = false;

        env.storage().instance().set(&subscriber, &sub);
    }

    // Get subscription details
    pub fn get_subscription(env: Env, subscriber: Address) -> Subscription {
        env.storage()
            .instance()
            .get(&subscriber)
            .expect("No subscription")
    }
}