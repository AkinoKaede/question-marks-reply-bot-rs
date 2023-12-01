use std::path::PathBuf;
use std::sync::OnceLock;

use sled::Db;

pub(crate) mod user;
pub(crate) mod chat;

pub(crate) static DATABASE: OnceLock<Db> = OnceLock::new();

pub(crate) async fn init(path: PathBuf) {
    let db = sled::open(path).unwrap();
    DATABASE.set(db).expect("Failed to set OnceLock");
}