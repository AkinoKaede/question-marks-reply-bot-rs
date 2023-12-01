use clap::Parser;
use teloxide::{prelude::*, update_listeners::webhooks};
use teloxide::types::MessageCommon;

mod question_mark_reply;
mod handlers;

#[derive(Parser)]
#[command(name = "question-marks-reply-bot-rs")]
#[command(version = "0.1.0")]
#[command(about = "A Telegram bot for reply question marks.", long_about = None)]
struct Cli {
    #[arg(short, long)]
    token: Option<String>,

    #[arg(short, long)]
    address: Option<String>,

    #[arg(short, long)]
    url: Option<String>,
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

    let listener = match cli.address {
        Some(address) => {
            let address = address.parse().expect("Unable to parse socket address.");
            let url = cli.url.expect("The Url must be set.");
            let url = url.parse().unwrap();

            let axum = webhooks::axum(bot.clone(), webhooks::Options::new(address, url))
                .await
                .expect("Couldn't setup webhook");

            Some(axum)
        }

        _ => None,
    };


    match listener {
        Some(listener) => {
            teloxide::repl_with_listener(bot, handlers::message::message_handler, listener).await
        }
        None => {
            teloxide::repl(bot, handlers::message::message_handler).await
        }
    }
}

