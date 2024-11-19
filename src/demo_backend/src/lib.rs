use candid::Nat;
use ic_cdk::api::management_canister::main::raw_rand;
use std::time::Duration;

thread_local! {
    static CURRENT_RANDOM_NUMBER: std::cell::RefCell<Nat> = std::cell::RefCell::new(Nat::from(0u8));
}

#[ic_cdk::init]
async fn init() {
    ic_cdk_timers::set_timer_interval(Duration::from_secs(5), generate_new_number);
}

 fn generate_new_number() {
    let _ = ic_cdk::spawn(async {
        let random_bytes = raw_rand().await.unwrap().0;
        if !random_bytes.is_empty() {
            CURRENT_RANDOM_NUMBER.with_borrow_mut(|n| {
                *n = Nat::from(random_bytes[0]);
            });
            ic_cdk::println!(
                "New random number generated: {}",
                CURRENT_RANDOM_NUMBER.with_borrow(|n| n.clone())
            );
        }
    });
}

#[ic_cdk::query]
fn get_current_number() -> Nat {
    CURRENT_RANDOM_NUMBER.with_borrow(|n| n.clone())
}
