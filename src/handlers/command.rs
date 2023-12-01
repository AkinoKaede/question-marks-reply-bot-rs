use teloxide::prelude::*;
use teloxide::RequestError;
use teloxide::utils::command::BotCommands;

use crate::entities::prelude::*;

#[derive(teloxide::macros::BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub(crate) enum Command {
    #[command(description = "Display this text.")]
    Help,
    #[command(description = "Enable reply for you.")]
    EnableForMe,
    #[command(description = "Disable reply for you.")]
    DisableForMe,
}

pub(crate) async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> Result<(), RequestError> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .reply_to_message_id(msg.id).await?;
        }
        Command::EnableForMe => {   
        }
        Command::DisableForMe => {}
    }

    Ok(())
}