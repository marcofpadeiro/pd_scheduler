pub enum Action {
    Setup,
    Search,
    Schedule,
    Help,
}

impl Action {
    fn new(string: String) -> Action {
        match string.as_str() {
            "setup" => Self::Setup,
            "search" => Self::Search,
            "sche" => Self::Schedule,
            "help" => Self::Help,
            _ => Self::Help,
        }
    }
}

pub struct Config {
    pub action: Action,
    pub args: Option<Vec<String>>,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Config {
        args.next();
        let action = Action::new(args.next().unwrap_or_else(|| String::from("help")));
        let args = if args.by_ref().peekable().peek().is_none() {
            None
        } else {
            Some(args.collect())
        };

        Config { action, args }
    }
}
