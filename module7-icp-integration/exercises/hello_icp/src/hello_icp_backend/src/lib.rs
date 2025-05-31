use candid::{CandidType, Deserialize};
use ic_cdk::{init, query, update};
use std::cell::RefCell;

#[derive(CandidType, Deserialize, Clone)]
struct GreetingState {
    message: String,
    history: Vec<String>,
}

thread_local! {
    static STATE: RefCell<GreetingState> = RefCell::new(GreetingState {
        message: String::from("Hello, Internet Computer!"),
        history: Vec::new(),
    });
}

#[init]
fn init() {
    let initial_message = String::from("Hello, Internet Computer!");
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.message = initial_message.clone();
        state.history.push(initial_message);
    });
}

#[update]
fn update_greeting(new_greeting: String) -> String {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.message = new_greeting.clone();
        state.history.push(new_greeting);
        state.message.clone()
    })
}

#[query]
fn get_greeting() -> String {
    STATE.with(|state| state.borrow().message.clone())
}

#[query]
fn get_greeting_history() -> Vec<String> {
    STATE.with(|state| state.borrow().history.clone())
}

// Additional challenge: Add a method to get the greeting history