use std::path::PathBuf;

use clap::Parser;
use teloxide::{prelude::*, update_listeners::webhooks};

use crate::database::init;
use crate::handlers::{command, message};

mod question_marks_reply;
mod handlers;
mod database;

#[derive(Parser)]
#[command(name = "question-marks-reply-bot-rs")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A Telegram bot for reply question marks.", long_about = None)]
struct Cli {
    #[arg(short, long, env = "QUESTION_MARKS_REPLY_BOT_RS_TOKEN")]
    token: Option<String>,

    #[arg(short, long, env = "QUESTION_MARKS_REPLY_BOT_RS_ADDRESS")]
    address: Option<String>,

    #[arg(short, long, env = "QUESTION_MARKS_REPLY_BOT_RS_URL")]
    url: Option<String>,

    #[arg(short, long, env = "QUESTION_MARKS_REPLY_BOT_RS_DATA_DIR")]
    data_dir: Option<PathBuf>,
}


#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    pretty_env_logger::init();
    log::info!("Starting question marks reply bot...");


    let data_dir = match cli.data_dir {
        Some(db_path) => PathBuf::from(db_path).join("database.sled/"),
        None => {
            let xdg_dirs = xdg::BaseDirectories::with_prefix("question-marks-reply-bot-rs")
                .expect("Failed to get XDG directories");

            match xdg_dirs.find_data_file("database.sled/") {
                Some(path) => path,
                None => {
                    xdg_dirs
                        .create_data_directory("database.sled/")
                        .expect("Failed to create configuration directory")
                }
            }
        }
    };

    init(data_dir).await;

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

