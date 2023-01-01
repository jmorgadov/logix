use std::fs::{read_to_string, write};

use serde_json::Value;

use crate::components::composed_component::ComposedComponent;

pub trait JSONSerialize {
    fn to_json(&self) -> Value;
    fn from_json(json: &Value) -> Self
    where
        Self: Sized;
}

/// Saves a `ComposedComponent` as a JSON file to a given location
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the file path where to save the component.
/// * `comp` - A reference to the component that will be stored.
///
/// # Examples
///
/// ```
/// // assuming `comp` is a variable that holds a `ComposedComponent`
/// save("example_comp.json", &comp);
/// ```
pub fn save(file_path: &str, comp: &ComposedComponent) {
    write(file_path, serde_json::to_string(&comp.to_json()).unwrap())
        .expect("Unable to write file");
}

/// Loads a `ComposedComponent` from a JSON file
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the file path where to load the component.
///
/// # Examples
///
/// ```
/// let comp = load("example_comp.json");
/// ```
pub fn load(file_path: &str) -> ComposedComponent {
    let data = read_to_string(file_path).expect("Unable to read file");
    ComposedComponent::from_json(&serde_json::from_str::<Value>(&data).unwrap())
}
