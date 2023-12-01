use std::path::PathBuf;

use clap::Parser;
use sea_orm::Database;
use teloxide::{prelude::*, update_listeners::webhooks};

use crate::handlers::{command, message};

mod question_mark_reply;
mod entities;
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

    #[arg(short, long)]
    db_file: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    pretty_env_logger::init();
    log::info!("Starting question marks reply bot...");


    let db_path = match cli.db_file {
        Some(db_path) => db_path,
        None => {
            let xdg_dirs = xdg::BaseDirectories::with_prefix("question-marks-reply-bot-rs")
                .expect("Failed to get XDG directories");

            xdg_dirs
                .place_data_file("database.sqlite")
                .expect("Failed to create configuration directory")
        }
    };

    let db = Database::connect(format!("sqlite://{}?mode=rwc",
                                       db_path.to_string_lossy())).await.expect("Database cannot connect");


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


    let handler = dptree::entry()
        .branch(Update::filter_message()
                .filter_command::<command::Command>()
                .endpoint(command::command_handler))
        .branch(Update::filter_message().endpoint(message::message_handler));


    let mut dispatcher = Dispatcher::builder(bot, handler).enable_ctrlc_handler().build();

    match listener {
        Some(listener) => {
            dispatcher.dispatch_with_listener(listener,
                                              LoggingErrorHandler::with_custom_text("An error from the update listener")).await;
        }
        None => {
            dispatcher.dispatch().await;
        }
    }
}

