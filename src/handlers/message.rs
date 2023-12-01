use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use rand;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use teloxide::{Bot, RequestError};
use teloxide::prelude::Message;
use teloxide::types::{MediaKind, MediaSticker, MediaText, MessageCommon, MessageKind};

use crate::database::DATABASE;
use crate::database::user::User;
use crate::question_mark_reply;

pub(crate) async fn message_handler(bot: Bot, msg: Message) -> Result<(), RequestError> {
    let db = DATABASE.get().unwrap();

    {
        let user_id = msg.from().unwrap().id.0;
        let user = User::new(db);
        if !user.get_enabled(user_id).unwrap_or(true) {
            return Ok(());
        }
    }


    match msg.kind {
        MessageKind::Common(
            MessageCommon {
                media_kind: MediaKind::Text(MediaText { .. }),
                ..
            }) => {
            if msg.chat.is_group() || msg.chat.is_supergroup() {
                let chat = crate::database::chat::Chat::new(db);
                let probability = chat.get_probability_for_text(msg.chat.id.0).unwrap_or(1f64);

                if random(msg.id.0, probability).await {
                    return Ok(question_mark_reply::on_text::on_text(bot, msg).await?);
                }
            }
            Ok(())
        }
        MessageKind::Common(
            MessageCommon {
                media_kind: MediaKind::Sticker(MediaSticker { .. }),
                ..
            }
        ) => {
            if msg.chat.is_group() || msg.chat.is_supergroup() {
                let chat = crate::database::chat::Chat::new(db);
                let probability = chat.get_probability_for_sticker(msg.chat.id.0).unwrap_or(1f64);

                if random(msg.id.0, probability).await {
                    return Ok(question_mark_reply::on_sticker::on_sticker(bot, msg).await?);
                }
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

async fn random(message_id: i32, probability: f64) -> bool {
    let mut hasher = DefaultHasher::new();
    hasher.write_i32(message_id);
    let mut rng = StdRng::seed_from_u64(hasher.finish());
    let rand_num = rng.gen::<f64>();

    rand_num < probability
}