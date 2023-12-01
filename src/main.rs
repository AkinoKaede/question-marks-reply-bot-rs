use clap::Parser;
use teloxide::{prelude::*, RequestError};
use teloxide::types::{MediaKind, MediaSticker, MediaText, MessageCommon, MessageKind};

mod question_mark_reply;

#[derive(Parser)]
#[command(name = "question-marks-reply-bot-rs")]
#[command(version = "0.1.0")]
#[command(about = "A Telegram bot for reply question marks.", long_about = None)]
struct Cli {
    #[arg(short, long)]
    token: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    pretty_env_logger::init();
    log::info!("Starting question marks reply bot...");

    let bot = match cli.token {
        Some(token) => Bot::new(token),
        None => Bot::from_env(),
    };


    teloxide::repl(bot, handler).await;
}

async fn handler(bot: Bot, msg: Message) -> Result<(), RequestError> {
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
