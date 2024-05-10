use serde::{Deserialize, Serialize};
use serde_json;
use std::{error::Error, fs};

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Search,
    Schedule,
    Help,
}

impl Action {
    fn new(string: String) -> Action {
        match string.as_str() {
            "search" => Self::Search,
            "schedule" => Self::Schedule,
            "help" => Self::Help,
            _ => Self::Help,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub action: Option<Action>,
    pub args: Option<Vec<String>>,
    pub calendar_id: String,
    pub event_name: String,
    pub address: String,
    pub timezone: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Config {
        args.next();

        let action = Some(Action::new(
            args.next().unwrap_or_else(|| String::from("help")),
        ));

        let args = args.collect::<Vec<String>>();

        let args = if args.is_empty() { None } else { Some(args) };

        let config = Config::read_config().expect("Failed to read configuration");

        Config {
            action,
            args,
            calendar_id: config.calendar_id,
            event_name: config.event_name,
            address: config.address,
            timezone: config.timezone,
        }
    }

    fn read_config() -> Result<Config, Box<dyn Error>> {
        let file_contents = fs::read_to_string("config.json")?;
        let config: Config = serde_json::from_str(&file_contents)?;
        Ok(config)
    }
}
