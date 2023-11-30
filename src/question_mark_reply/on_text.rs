use teloxide::{Bot, RequestError};
use teloxide::prelude::*;

use crate::question_mark_reply::{IsComposedOfSameChar, QuestionMarks};

pub(crate) async fn on_text(bot: Bot, msg: Message) -> Result<(), RequestError> {
    if let Some(text) = msg.text() {
        let text = text.to_string();

        if let Some(reply_msg) = msg.reply_to_message() {
            if let Some(reply_text) = reply_msg.text() {
                let reply_text = reply_text.to_string();

                if text.is_composed_of_same_char().unwrap_or(false)
                    && reply_text.is_composed_of_same_char().unwrap_or(false) {
                    let chr = text.chars().next().unwrap();

                    if text.chars().filter(|c| *c == chr).count()
                        == reply_text.chars().filter(|c| *c == chr).count() + 1 {
                        bot.send_message(msg.chat.id, format!("{}{}", text, chr))
                            .reply_to_message_id(msg.id).await?;

                        return Ok(());
                    }
                }
            }
        }

        if text.is_composed_of_question_marks() {
            if let Some(rev) = text.rev() {
                bot.send_message(msg.chat.id, rev).reply_to_message_id(msg.id).await?;
                return Ok(());
            }

            bot.send_message(msg.chat.id, text).reply_to_message_id(msg.id).await?;
        }
    }

    Ok(())
}