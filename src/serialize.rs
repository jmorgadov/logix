use std::fs::{read_to_string, write};

use serde_json::Value;

use crate::components::composed_component::ComposedComponent;

pub trait JSONSerialize {
    fn to_json(&self) -> Value;
    fn from_json(json: &Value) -> Self
    where
        Self: Sized;
}

pub fn save(file_path: &str, value: &Value) {
    write(file_path, serde_json::to_string(value).unwrap()).expect("Unable to write file");
}

pub fn load(file_path: &str) -> ComposedComponent {
    let data = read_to_string(file_path).expect("Unable to read file");
    ComposedComponent::from_json(&serde_json::from_str::<Value>(&data).unwrap())
}
