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
            "schedule" => Self::Schedule,
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

        let args = args.collect::<Vec<String>>();

        let args = if args.is_empty() { None } else { Some(args) };

        Config { action, args }
    }
}
