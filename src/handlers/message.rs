use std::fmt::Debug;
use teloxide::{Bot, RequestError};
use teloxide::prelude::Message;
use teloxide::types::{MediaKind, MediaSticker, MediaText, MessageCommon, MessageKind};
use crate::question_mark_reply;

pub(crate) async fn message_handler(bot: Bot, msg: Message) -> Result<(), RequestError> {
    match msg.kind {
        MessageKind::Common(
            MessageCommon {
                media_kind: MediaKind::Text(MediaText { .. }),
                ..
            }) => Ok(question_mark_reply::on_text::on_text(bot, msg).await?),
        MessageKind::Common(
            MessageCommon {
                media_kind: MediaKind::Sticker(MediaSticker { .. }),
                ..
            }
        ) => Ok(question_mark_reply::on_sticker::on_sticker(bot, msg).await?),
        _ => Ok(()),
    }
}
