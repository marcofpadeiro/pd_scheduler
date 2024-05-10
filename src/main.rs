pub mod code;

use pd_scheduler::config::{Action, Config};
use pd_scheduler::{help, schedule, search};
use std::env;

#[tokio::main]
async fn main() {
    let config = Config::build(env::args());

    match config.action.as_ref().unwrap() {
        Action::Schedule => schedule(&config).await,
        Action::Search => search(config.args),
        _ => help(),
    }
}
