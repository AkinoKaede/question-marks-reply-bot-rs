use sled::{Db, Tree};

pub(crate) struct Chat {
    text: Tree,
    sticker: Tree,
}

impl Chat {
    pub(crate) fn new(db: &Db) -> Self {
        let text = db.open_tree(b"chats:probability_for_text").unwrap();
        let sticker = db.open_tree(b"chats:probability_for_sticker").unwrap();
        Self { text, sticker }
    }

    pub(crate) fn set_probability_for_text(&self, id: i64, probability: f64) {
        assert!(probability <= 1f64);
        self.text.insert(id.to_le_bytes(), &probability.to_le_bytes()).expect("Failed to set probability for text.");
    }

    pub(crate) fn get_probability_for_text(&self, id: i64) -> Option<f64> {
        if let Some(res) = self.text.get(id.to_le_bytes()).unwrap() {
            let probability = res.to_vec();
            let probability: [u8; 8] = (&probability[..]).try_into().unwrap();
            return Some(f64::from_le_bytes(probability));
        }

        None
    }

    pub(crate) fn set_probability_for_sticker(&self, id: i64, probability: f64) {
        assert!(probability <= 1f64);
        self.sticker.insert(id.to_le_bytes(), &probability.to_le_bytes()).expect("Failed to set probability for sticker.");
    }

    pub(crate) fn get_probability_for_sticker(&self, id: i64) -> Option<f64> {
        if let Some(res) = self.sticker.get(id.to_le_bytes()).unwrap() {
            let probability = res.to_vec();
            let probability: [u8; 8] = (&probability[..]).try_into().unwrap();
            return Some(f64::from_le_bytes(probability));
        }

        None
    }
}