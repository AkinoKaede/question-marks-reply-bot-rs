use sled::{Db, Tree};

pub(crate) struct User {
    enabled: Tree,
}

impl User {
    pub(crate) fn new(db: &Db) -> Self {
        let enabled = db.open_tree(b"users:enabled").unwrap();
        Self { enabled }
    }
    pub(crate) fn set_enabled(&self, id: u64, enabled: bool) {
        self.enabled.insert(id.to_le_bytes(), if enabled { vec![1] } else { vec![0] }).expect("Failed to set User.enabled");
    }

    pub(crate) fn get_enabled(&self, id: u64) -> bool {
        if let Some(res) = self.enabled.get(id.to_le_bytes()).unwrap() {
            return res.iter().next() == Some(&1u8);
        }

        true
    }
}