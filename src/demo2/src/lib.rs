use candid::Nat;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod, HttpResponse, TransformArgs,
    TransformContext, TransformFunc,
};
use ic_cdk::api::management_canister::main::raw_rand;
use std::time::Duration;

// NOTE: These thread_local! variables are not stored in the stable memory and are not persisted across canister upgrades.
thread_local! {
    static CURRENT_RANDOM_NUMBER: std::cell::RefCell<Nat> = std::cell::RefCell::new(Nat::from(0u8));
    static PROPOSAL_ID: std::cell::RefCell<String> = std::cell::RefCell::new("1".to_string());
}

#[ic_cdk::init]
async fn init() {
    // Initialize timer to generate new number every 30 seconds
    ic_cdk_timers::set_timer_interval(Duration::from_secs(30), generate_new_number);
    // Initialize timer to send an HTTPS outcall and print the results every 32 seconds
    ic_cdk_timers::set_timer_interval(Duration::from_secs(32), print_results);
}

// Create a function to generate a new random number.
// This function calls the raw_rand method of the management canister.
// Then it uses the random bytes returned to generate a random number.
fn generate_new_number() {
    ic_cdk::spawn(async {
        let random_bytes = raw_rand().await.unwrap().0;
        if !random_bytes.is_empty() {
            // Use the first byte to generate a number between 133000 and 133255 (recent proposal numbers)
            let current_random_number = random_bytes[0] as u32 + 133000u32;
            CURRENT_RANDOM_NUMBER.with_borrow_mut(|n| {
                *n = Nat::from(current_random_number).clone();
            });
            PROPOSAL_ID.with_borrow_mut(|p| {
                *p = current_random_number.to_string();
            });
        }
    });
}

// Define a public function to get proposal information.
#[ic_cdk::update]
async fn get_icp_info() -> String {
    // Define the API endpoints to obtain data from.
    // This example uses the IC API, but any API endpoint can be used.
    let url = format!(
        "https://ic-api.internetcomputer.org/api/v3/proposals/{}",
        PROPOSAL_ID.with_borrow(|p| p.clone())
    );
    let arg = CanisterHttpRequestArgument {
        url,
        max_response_bytes: None,
        method: HttpMethod::GET,
        headers: vec![],
        body: None,
        transform: Some(TransformContext {
            function: TransformFunc(candid::Func {
                method: "transform".to_string(),
                principal: ic_cdk::id(),
            }),
            context: vec![],
        }),
    };
    // HTTP outcalls require cycles are attached to the call.
    let http_response = http_request(arg, 20_949_972_000).await.unwrap().0;

    String::from_utf8(http_response.body).unwrap()
}

#[ic_cdk::query]
fn transform(raw: TransformArgs) -> HttpResponse {
    HttpResponse {
        status: raw.response.status,
        headers: vec![],
        body: raw.response.body,
    }
}

fn print_results() {
    ic_cdk::println!(
        "Generated new random proposal number: {}",
        CURRENT_RANDOM_NUMBER.with_borrow(|n| n.clone())
    );
    ic_cdk::spawn(async {
        let result = get_icp_info().await;
        ic_cdk::println!("Proposal info obtained through HTTPS outcall: {}", result);
    });
}
