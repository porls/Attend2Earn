#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Env, Symbol, Vec,
};

#[contract]
pub struct Attend2Earn;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Parent,
    Student,
    DailyAmount,
    Token,
    Attendance,
    Claimed,
}

#[contractimpl]
impl Attend2Earn {

    // Initialize contract
    pub fn init(
        env: Env,
        parent: Address,
        student: Address,
        daily_amount: i128,
        token: Address,
    ) {
        parent.require_auth();

        env.storage().instance().set(&DataKey::Parent, &parent);
        env.storage().instance().set(&DataKey::Student, &student);
        env.storage().instance().set(&DataKey::DailyAmount, &daily_amount);
        env.storage().instance().set(&DataKey::Token, &token);

        let empty: Vec<u32> = Vec::new(&env);
        env.storage().instance().set(&DataKey::Attendance, &empty);
        env.storage().instance().set(&DataKey::Claimed, &empty);
    }

    // Mark attendance
    pub fn mark_attendance(env: Env, day: u32) {
        let mut attendance: Vec<u32> = env
            .storage()
            .instance()
            .get(&DataKey::Attendance)
            .unwrap();

        for d in attendance.iter() {
            if d == day {
                panic!("Already marked");
            }
        }

        attendance.push_back(day);
        env.storage().instance().set(&DataKey::Attendance, &attendance);

        env.events().publish((Symbol::new(&env, "attendance"), day), true);
    }

    // Claim allowance
    pub fn claim(env: Env, student: Address, day: u32) {
        student.require_auth();

        let stored_student: Address = env
            .storage()
            .instance()
            .get(&DataKey::Student)
            .unwrap();

        if student != stored_student {
            panic!("Not authorized");
        }

        // Check attendance
        let attendance: Vec<u32> = env
            .storage()
            .instance()
            .get(&DataKey::Attendance)
            .unwrap();

        let mut found = false;
        for d in attendance.iter() {
            if d == day {
                found = true;
            }
        }

        if !found {
            panic!("No attendance");
        }

        // Prevent double claim
        let mut claimed: Vec<u32> = env
            .storage()
            .instance()
            .get(&DataKey::Claimed)
            .unwrap();

        for d in claimed.iter() {
            if d == day {
                panic!("Already claimed");
            }
        }

        // Get values
        let amount: i128 = env
            .storage()
            .instance()
            .get(&DataKey::DailyAmount)
            .unwrap();

        let parent: Address = env
            .storage()
            .instance()
            .get(&DataKey::Parent)
            .unwrap();

        let token_addr: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .unwrap();

        // Transfer using token contract
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&parent, &student, &amount);

        // Mark claimed
        claimed.push_back(day);
        env.storage().instance().set(&DataKey::Claimed, &claimed);

        env.events().publish((Symbol::new(&env, "paid"), day), amount);
    }
}