use std::time::Duration;

pub const UPDATE_INTERVAL: Duration = Duration::from_secs(300);

pub const HELP_COMMAND: &str = "Hi! My purpose is spam the group with messages until everyone updates that they're home. Available commands:\n
/help - Show available commands (this message)
/start [No. of people] - Start a 'going home' session. Optionally include the number of people expected to update as an argument. For example if 2 persons are expected to update that they are home, then send '/start 2'. If number of people expected is not given, then the value defaults to the number of people in this group.
/homed - Update that you are home";
