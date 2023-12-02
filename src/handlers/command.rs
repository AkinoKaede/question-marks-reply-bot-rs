use log::info;
use teloxide::prelude::*;
use teloxide::RequestError;
use teloxide::utils::command::BotCommands;

use crate::database::chat::Chat;
use crate::database::DATABASE;
use crate::database::user::User;

#[derive(teloxide::macros::BotCommands, Clone)]
#[command(rename_rule = "snake_case", description = "These commands are supported:")]
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
    #[command(description = "Get probabilities of reply to your messages.")]
    GetProbabilities,
}

pub(crate) async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> Result<(), RequestError> {
    let db = DATABASE.get().unwrap();

    match cmd {
        Command::Help => {
            info!("{} executed /help in {}", msg.from().unwrap().id.0, msg.chat.id.0);

            let help_text = Command::descriptions().to_string();
            bot.send_message(msg.chat.id, help_text).await?;
        }
        Command::EnableForMe => {
            info!("{} executed /enable_for_me in {}", msg.from().unwrap().id.0, msg.chat.id.0);

            let user_id = msg.from().unwrap().id.0;
            let user = User::new(db);
            user.set_enabled(user_id, true);

            reply_to_message(&bot, &msg, "Enabled for you.").await?;
        }
        Command::DisableForMe => {
            info!("{} executed /disable_for_me in {}", msg.from().unwrap().id.0, msg.chat.id.0);

            let user_id = msg.from().unwrap().id.0;
            let user = User::new(db);
            user.set_enabled(user_id, false);

            reply_to_message(&bot, &msg, "Disabled for you.").await?;
        }
        Command::SetProbabilityForText(probability) => {
            info!("{} executed /set_probability_for_text {} in {}", msg.from().unwrap().id,
                probability, msg.chat.id.0);

            let chat = Chat::new(db);
            set_probability(&bot, &msg, probability,
                            |p| chat.set_probability_for_text(msg.chat.id.0, p)).await?;
        }
        Command::SetProbabilityForSticker(probability) => {
            info!("{} executed /set_probability_for_sticker {} in {}", msg.from().unwrap().id,
                probability, msg.chat.id.0);

            let chat = Chat::new(db);
            set_probability(&bot, &msg, probability,
                            |p| chat.set_probability_for_sticker(msg.chat.id.0, p)).await?;
        }
        Command::GetProbabilities => {
            info!("{} executed /get_probabilities in {}", msg.from().unwrap().id.0, msg.chat.id.0);

            let chat = Chat::new(db);
            let probability_for_text = chat.get_probability_for_text(msg.chat.id.0);
            let probability_for_sticker = chat.get_probability_for_sticker(msg.chat.id.0);

            let probability_for_text = match probability_for_text {
                Some(probability) => probability.to_string(),
                None => "none (default: 1)".to_string(),
            };

            let probability_for_sticker = match probability_for_sticker {
                Some(probability) => probability.to_string(),
                None => "none (default: 1)".to_string(),
            };

            let text = format!("Probability for text: {}\nProbability for sticker: {}", probability_for_text, probability_for_sticker);
            reply_to_message(&bot, &msg, text).await?;
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