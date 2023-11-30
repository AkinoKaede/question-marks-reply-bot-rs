use teloxide::{prelude::*};
use teloxide::types::{MediaKind, MediaSticker, MediaText, MessageCommon, MessageKind};

mod question_mark_reply;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting question marks reply bot...");
    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
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
    }).await;
}


