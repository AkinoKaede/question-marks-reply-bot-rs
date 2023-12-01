use teloxide::prelude::*;
use teloxide::RequestError;
use teloxide::utils::command::BotCommands;

use crate::database::chat::Chat;
use crate::database::DATABASE;
use crate::database::user::User;

#[derive(teloxide::macros::BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub(crate) enum Command {
    #[command(description = "Display this text.")]
    Help,
    #[command(description = "Enable reply for you.")]
    EnableForMe,
    #[command(description = "Disable reply for you.")]
    DisableForMe,
    #[command(description = "Set probability of reply to your text messages.")]
    SetProbabilityForText(f64),
    #[command(description = "Set probability of reply to your sticker messages.")]
    SetProbabilityForSticker(f64),
}

pub(crate) async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> Result<(), RequestError> {
    let db = DATABASE.get().unwrap();

    match cmd {
        Command::Help => {
            let help_text = Command::descriptions().to_string();
            bot.send_message(msg.chat.id, help_text).await?;
        }
        Command::EnableForMe => {
            let user_id = msg.from().unwrap().id.0;
            let user = User::new(db);
            user.set_enabled(user_id, true);

            reply_to_message(&bot, &msg, "Enabled for you.").await?;
        }
        Command::DisableForMe => {
            let user_id = msg.from().unwrap().id.0;
            let user = User::new(db);
            user.set_enabled(user_id, false);

            reply_to_message(&bot, &msg, "Disabled for you.").await?;
        }
        Command::SetProbabilityForText(probability) => {
            let chat = Chat::new(db);
            set_probability(&bot, &msg, probability,
                            |p| chat.set_probability_for_text(msg.chat.id.0, p)).await?;
        }
        Command::SetProbabilityForSticker(probability) => {
            let chat = Chat::new(db);
            set_probability(&bot, &msg, probability,
                            |p| chat.set_probability_for_sticker(msg.chat.id.0, p)).await?;
        }
    }

    Ok(())
}

async fn reply_to_message<T>(bot: &Bot, msg: &Message, text: T) -> Result<(), RequestError>
    where T: Into<String>,
{
    bot.send_message(msg.chat.id, text)
        .reply_to_message_id(msg.id).await?;
    Ok(())
}


async fn is_user_administrator_for_chat(bot: &Bot, chat_id: &ChatId, user_id: &UserId) -> Result<bool, RequestError> {
    let admins = bot.get_chat_administrators(*chat_id).await?;
    let ids = admins.iter().map(|m| m.user.id).collect::<Vec<UserId>>();

    Ok(ids.iter().any(|i| i == user_id))
}

async fn set_probability<F>(bot: &Bot, msg: &Message, probability: f64, function: F) -> Result<(), RequestError>
    where F: Fn(f64) -> ()
{
    Ok(if msg.chat.is_group() || msg.chat.is_supergroup() {
        if probability <= 1f64 {
            match is_user_administrator_for_chat(bot, &msg.chat.id, &msg.from().unwrap().id).await {
                Ok(true) => {
                    function(probability);
                    reply_to_message(bot, msg, format!("Succeed to set probability to {}.", probability)).await?;
                }
                Ok(false) => {
                    reply_to_message(bot, msg, "You are not administrator.").await?;
                }
                Err(err) => {
                    reply_to_message(bot, msg, "Failed to set probability.").await?;
                    log::error!("Failed to get administrators: {:?}", err);
                }
            }
        } else {
            reply_to_message(&bot, msg, "Probability should between 0 and 1.").await?;
        }
    } else {
        reply_to_message(&bot, msg, "Cannot set probability in Private Chat.").await?;
    })
}