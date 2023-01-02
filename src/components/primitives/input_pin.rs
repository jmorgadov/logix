use crate::{
    components::component::{Component, CompEvent},
    serialize::JSONSerialize,
};

use super::primitive::Primitive;

/// Represents an input pin.
///
/// This component must be use when creating composed components that have
/// external inputs.
#[derive(Debug)]
pub struct InputPin {
    id: u32,
    ins: Vec<bool>,
    outs: Vec<bool>,
}

impl InputPin {
    /// Creates a new `OutputPin` component given an id.
    ///
    /// # Arguments
    ///
    /// * `id` - Integer that represents the component id.
    ///
    /// # Example
    ///
    /// ```
    /// let gate = OutputPin::new(0);
    /// ```
    pub fn new(id: u32) -> Self {
        InputPin {
            id,
            ins: vec![false],
            outs: vec![false],
        }
    }
}

impl JSONSerialize for InputPin {
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "name": Primitive::InputPin.to_string(),
        })
    }

    fn from_json(json: &serde_json::Value) -> Self
    where
        Self: Sized,
    {
        InputPin::new(json["id"].as_u64().unwrap() as u32)
    }
}

impl Component for InputPin {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> String {
        Primitive::InputPin.to_string()
    }

    fn ins(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn outs(&mut self) -> &mut Vec<bool> {
        &mut self.outs
    }

    fn on_event(&mut self, event: &CompEvent) {
        if let CompEvent::UpdateValues = event {
            self.outs[0] = self.ins[0];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{
        component::{Component, CompEvent},
        primitives::input_pin::InputPin,
    };

    #[test]
    fn input_pin() {
        let comp = &mut InputPin::new(0);

        comp.set_in(0, false);
        comp.on_event(&CompEvent::UpdateValues);
        assert!(!comp.outs[0]);

        comp.set_in(0, true);
        comp.on_event(&CompEvent::UpdateValues);
        assert!(comp.outs[0]);
    }
}
