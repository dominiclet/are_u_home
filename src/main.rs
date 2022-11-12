use teloxide::dispatching::{UpdateHandler, dialogue};
use teloxide::prelude::*;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::utils::command::BotCommands;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    GoingHome,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "This bot keeps track of who is home. These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "start going home.")]
    Start,
    #[command(description = "update that you are home.")]
    Homed,
}

type BotDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env();


    Dispatcher::builder(
        bot,
        schema()
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(
            case![State::Start]
                .branch(case![Command::Help].endpoint(help))
                .branch(case![Command::Start].endpoint(start))
        )
        .branch(
            case![State::GoingHome]
                .branch(case![Command::Help].endpoint(help))
                .branch(case![Command::Start].endpoint(going_home_start))
                .branch(case![Command::Homed].endpoint(update_homed))
        );

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(
            Update::filter_message()
                .branch(command_handler)
        )
}

async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

async fn start(bot: Bot, dialogue: BotDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Starting to annoy everyone until everyone gets home.").await?;
    dialogue.update(State::GoingHome).await?;
    Ok(())
}

async fn update_homed(bot: Bot, _dialogue: BotDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Updated that you are home.").await?;
    Ok(())
}

async fn going_home_start(bot: Bot, _dialogue: BotDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Can't start a going home session. Previous going home session is not complete??").await?;
    Ok(())
}
