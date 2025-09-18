use std::collections::HashMap;
#[derive(Debug, Default)]
pub struct Inststorage {
    map: HashMap<String, i64>,
}

impl Inststorage {
    pub fn get(&self, key: &str) -> Option<i64> {
        self.map.get(key).copied()
    }
    pub fn set(&mut self, key: String, val: i64) {
        self.map.insert(key, val);
    }
}
