use std::env;

use reqwest::blocking;

use kiwivm_cli::{KiwivmCLI, Operation};


fn main() {
    let mut vps = KiwivmCLI {
        veid: env::var("KIWIVM_VEID").unwrap(),
        api_key: env::var("KIWIVM_API_KEY").unwrap(),
        info: None,
    };

    vps.call_api(Operation::Start);
    vps.call_api(Operation::GetRateLimitStatus);
}
