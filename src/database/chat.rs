use sled::{Db, Tree};

pub(crate) struct Chat {
    texts: Tree,
    stickers: Tree,
}

impl Chat {
    pub(crate) fn new(db: &Db) -> Self {
        let texts = db.open_tree(b"chats:probabilities:texts").unwrap();
        let stickers = db.open_tree(b"chats:probabilities:stickers").unwrap();
        Self { texts, stickers }
    }

    pub(crate) fn set_probability_for_texts(&self, id: i64, probability: f64) {
        assert!(probability <= 1f64);
        self.texts.insert(id.to_le_bytes(), &probability.to_le_bytes()).expect("Failed to set probability for text.");
    }

    pub(crate) fn get_probability_for_texts(&self, id: i64) -> Option<f64> {
        if let Some(res) = self.texts.get(id.to_le_bytes()).unwrap() {
            let probability = res.to_vec();
            let probability: [u8; 8] = (&probability[..]).try_into().unwrap();
            return Some(f64::from_le_bytes(probability));
        }

        None
    }

    pub(crate) fn set_probability_for_stickers(&self, id: i64, probability: f64) {
        assert!(probability <= 1f64);
        self.stickers.insert(id.to_le_bytes(), &probability.to_le_bytes()).expect("Failed to set probability for sticker.");
    }

    pub(crate) fn get_probability_for_stickers(&self, id: i64) -> Option<f64> {
        if let Some(res) = self.stickers.get(id.to_le_bytes()).unwrap() {
            let probability = res.to_vec();
            let probability: [u8; 8] = (&probability[..]).try_into().unwrap();
            return Some(f64::from_le_bytes(probability));
        }

        None
    }
}