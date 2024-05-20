use teloxide::{Bot, RequestError};
use teloxide::prelude::*;
use teloxide::types::InputFile;

use crate::question_marks_reply::QuestionMarks;

pub(crate) async fn on_sticker(bot: Bot, msg: Message) -> Result<(), RequestError> {
    if let Some(sticker) = msg.sticker() {
        if let Some(emoji) = &sticker.emoji {
            // Reply when the emoji of sticker contains a question mark.
            // Because it’s all emojis, there’s no need to consider whether it contains text.
            if emoji.is_contains_question_marks() {
                bot.send_sticker(msg.chat.id, InputFile::file_id(&sticker.file.id))
                    .reply_to_message_id(msg.id.0).await?;
            }
        }
    }

    Ok(())
}
