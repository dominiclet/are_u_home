use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule="lowercase", description = "This bot helps your group keep track of who is home by annoying you \
        with messages until everyone is home. The following commands are supported:")]
pub enum Command {
    #[command(description = "Display this text.")]
    Help,
    #[command(description = "Start going home.")]
    Start, 
    #[command(description = "Update the group that you are home.")]
    Home,
}

pub async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Start => bot.send_message(msg.chat.id, "Starting going home session...").await?,
        Command::Home => bot.send_message(msg.chat.id, "Updated the group that you are home...").await?,
    };

    Ok(())
}

pub mod sub_routines {
    use std::collections::HashMap;
    use teloxide::types::ChatId;

    pub struct Task<'a> {
        grp_id: ChatId,
        no_ppl: i32,
        accounted: i32,
        not_home: Vec<&'a str>,
        last_bump: i64, 
    }

    pub struct AllTasks<'a> {
        tasks: HashMap<ChatId, Task<'a>>,
    }

    trait TaskMaster {
        fn new() -> AllTasks<'static>;
    }

    impl TaskMaster for AllTasks<'static> {
        fn new() -> AllTasks<'static> {
            let tasks: HashMap<ChatId, Task> = HashMap::new();
            AllTasks {
                tasks,
            }
        }
    }
}
