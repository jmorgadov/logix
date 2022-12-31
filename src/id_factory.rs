use std::collections::HashMap;

#[derive(Default)]
pub struct IDFactory {
    ids: HashMap<String, u32>,
    last: u32,
}

impl IDFactory {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set(&mut self, id: &str) -> u32 {
        let (key, val) = (id.to_string(), self.last);
        if self.ids.contains_key(&key) {
            panic!("Id already exists")
        }
        self.ids.insert(key, val);
        self.last += 1;
        val
    }

    pub fn get(&self, id: &str) -> u32 {
        match self.ids.get(&id.to_string()) {
            Some(val) => *val,
            _ => panic!("Id does not exists"),
        }
    }
}
