use std::time::Duration;

pub const UPDATE_INTERVAL: Duration = Duration::from_secs(300);

pub const HELP_COMMAND: &str = "Hi! My purpose is spam the group with messages until everyone updates that they're home. Available commands:\n
/help - Show available commands (this message)
/start - Start a 'going home' session
/homed - Update that you are home";
