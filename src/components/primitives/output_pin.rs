use crate::{
    components::component::{Component, SimEvent},
    serialize::JSONSerialize,
};

use super::primitive::Primitive;

/// Represents an output pin.
///
/// This component must be use when creating composed components that have
/// external outputs.
#[derive(Debug)]
pub struct OutputPin {
    id: u32,
    ins: Vec<bool>,
    outs: Vec<bool>,
}

impl OutputPin {
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
        OutputPin {
            id,
            ins: vec![false],
            outs: vec![false],
        }
    }
}

impl JSONSerialize for OutputPin {
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "name": Primitive::OutputPin.to_string(),
        })
    }

    fn from_json(json: &serde_json::Value) -> Self
    where
        Self: Sized,
    {
        OutputPin::new(json["id"].as_u64().unwrap() as u32)
    }
}

impl Component for OutputPin {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> String {
        Primitive::OutputPin.to_string()
    }

    fn ins(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn outs(&mut self) -> &mut Vec<bool> {
        &mut self.outs
    }

    fn on_event(&mut self, event: &SimEvent) {
        if let SimEvent::UpdateValues = event {
            self.outs[0] = self.ins[0];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{
        component::{Component, SimEvent},
        primitives::output_pin::OutputPin,
    };

    #[test]
    fn output_pin() {
        let comp = &mut OutputPin::new(0);

        comp.set_in(0, false);
        comp.on_event(&SimEvent::UpdateValues);
        assert!(!comp.outs[0]);

        comp.set_in(0, true);
        comp.on_event(&SimEvent::UpdateValues);
        assert!(comp.outs[0]);
    }
}
