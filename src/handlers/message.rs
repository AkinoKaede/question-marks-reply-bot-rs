use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use rand;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use teloxide::prelude::Message;
use teloxide::types::{MediaKind, MediaPhoto, MediaSticker, MediaText, MessageCommon, MessageKind};
use teloxide::{Bot, RequestError};

use crate::database::user::User;
use crate::database::DATABASE;
use crate::question_marks_reply;

pub(crate) async fn message_handler(bot: Bot, msg: Message) -> Result<(), RequestError> {
    let db = DATABASE.get().unwrap();

    if let Some(user_id) = msg.from().map(|u| u.id.0) {
        let user = User::new(db);
        if !user.get_enabled(user_id).unwrap_or(true) {
            return Ok(());
        }


        return match msg.kind {
            MessageKind::Common(
                MessageCommon {
                    media_kind: MediaKind::Text(MediaText { .. }),
                    ..
                }) |
            MessageKind::Common(
                MessageCommon {
                    media_kind: MediaKind::Photo(
                        MediaPhoto {
                            caption: Some(_),
                            ..
                        }),
                    ..
                })
            => {
                let chat = crate::database::chat::Chat::new(db);
                let probability = chat.get_probability_for_texts(msg.chat.id.0).unwrap_or(1f64);

                if random(msg.id.0, probability).await {
                    return Ok(question_marks_reply::on_text(bot, msg).await?);
                }

                Ok(())
            }
            MessageKind::Common(
                MessageCommon {
                    media_kind: MediaKind::Sticker(MediaSticker { .. }),
                    ..
                }
            ) => {
                let chat = crate::database::chat::Chat::new(db);
                let probability = chat.get_probability_for_stickers(msg.chat.id.0).unwrap_or(1f64);

                if random(msg.id.0, probability).await {
                    return Ok(question_marks_reply::on_sticker(bot, msg).await?);
                }
                Ok(())
            }
            _ => Ok(()),
        };
    }

    Ok(())
}

async fn random(message_id: i32, probability: f64) -> bool {
    let mut hasher = DefaultHasher::new();
    hasher.write_i32(message_id);
    let mut rng = StdRng::seed_from_u64(hasher.finish());
    let rand_num = rng.gen::<f64>();

    rand_num < probability
}