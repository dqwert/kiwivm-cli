use std::env;
use std::process;
use std::collections::HashMap;

use kiwivm_cli::{KiwivmCLI, Operation};

#[allow(unused_must_use)]
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || args[1].eq("-h") || args[1].eq("help") {
        println!("{}", KiwivmCLI::hint());
        process::exit(0);
    }

    let operation = Operation::new(args).unwrap();

    let vps = KiwivmCLI::new();
    vps.call_api(operation);
}
