use std::collections::HashMap;

use crate::utils::normalize;

pub struct Index {
    pub index: HashMap<String, Vec<u32>>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }

    pub fn add(&mut self, term: &str, product_id: u32) {
        let key = normalize(term);

        if key.is_empty() {
            return;
        }

        let entry = self.index.entry(key).or_default();

        if !entry.contains(&product_id) {
            entry.push(product_id);
        }
    }

    pub fn search(&self, term: &str) -> Vec<u32> {
        let key = normalize(term);
        self.index.get(&key).cloned().unwrap_or_default()
    }
}