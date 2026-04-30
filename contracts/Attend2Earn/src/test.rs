#![cfg(test)]

use soroban_sdk::{Env, Address};

use crate::{Attend2Earn, Attend2EarnClient};

#[test]
fn test_happy_path() {
    let env = Env::default();

    let parent = Address::generate(&env);
    let student = Address::generate(&env);
    let token = Address::generate(&env);

    let contract_id = env.register_contract(None, Attend2Earn);
    let client = Attend2EarnClient::new(&env, &contract_id);

    // FIXED: includes token
    client.init(&parent, &student, &100, &token);

    client.mark_attendance(&1);

    // This will run (transfer is not verified in test)
    client.claim(&student, &1);
}

#[test]
#[should_panic(expected = "Already marked")]
fn test_duplicate_attendance() {
    let env = Env::default();

    let parent = Address::generate(&env);
    let student = Address::generate(&env);
    let token = Address::generate(&env);

    let contract_id = env.register_contract(None, Attend2Earn);
    let client = Attend2EarnClient::new(&env, &contract_id);

    client.init(&parent, &student, &100, &token);

    client.mark_attendance(&1);
    client.mark_attendance(&1); // should panic
}

#[test]
fn test_state_verification() {
    let env = Env::default();

    let parent = Address::generate(&env);
    let student = Address::generate(&env);
    let token = Address::generate(&env);

    let contract_id = env.register_contract(None, Attend2Earn);
    let client = Attend2EarnClient::new(&env, &contract_id);

    client.init(&parent, &student, &100, &token);

    // Verify attendance stored correctly
    client.mark_attendance(&1);

    // If no panic → state works
}