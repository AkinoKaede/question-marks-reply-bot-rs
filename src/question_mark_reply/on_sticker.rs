use teloxide::{Bot, RequestError};
use teloxide::prelude::*;
use teloxide::types::InputFile;

use crate::question_mark_reply::QUESTION_MARK_EMOJIS;

pub(crate) async fn on_sticker(bot: Bot, msg: Message) -> Result<(), RequestError> {
    if let Some(sticker) = msg.sticker() {
        if let Some(emoji) = &sticker.emoji {
            if QUESTION_MARK_EMOJIS.contains(emoji) {
                bot.send_sticker(msg.chat.id, InputFile::file_id(&sticker.file.id))
                    .reply_to_message_id(msg.id.0).await?;
            }
        }
    }

    Ok(())
}
