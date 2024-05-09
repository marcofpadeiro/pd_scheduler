mod code;
mod config;

use config::Action;
use config::Config;
use pd_scheduler::{help, schedule, search, setup};
use std::env;

fn main() {
    let config = Config::build(env::args());

    match config.action {
        Action::Schedule => schedule(),
        Action::Search => search(config.args),
        Action::Setup => setup(),
        _ => help(),
    }
}
