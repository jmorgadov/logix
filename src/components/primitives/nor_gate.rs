use crate::{
    components::component::{Component, CompEvent},
    serialize::JSONSerialize,
};

use super::primitive::Primitive;

/// Represents a NOR gate component.
#[derive(Debug)]
pub struct NorGate {
    id: u32,
    ins: Vec<bool>,
    outs: Vec<bool>,
}

impl NorGate {
    /// Creates a new `NorGate` component given an id and the count of input
    /// pins.
    ///
    /// # Arguments
    ///
    /// * `id` - Integer that represents the component id.
    /// * `in_count` - Integer that represent how many input pins the gate has.
    ///
    /// # Example
    ///
    /// ```
    /// let gate = NorGate::new(0, 2);
    /// ```
    pub fn new(id: u32, in_count: usize) -> NorGate {
        NorGate {
            id,
            ins: vec![false; in_count],
            outs: vec![false],
        }
    }
}

impl JSONSerialize for NorGate {
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "name": Primitive::NorGate.to_string(),
            "in_count": self.ins.len(),
        })
    }

    fn from_json(json: &serde_json::Value) -> Self
    where
        Self: Sized,
    {
        NorGate::new(
            json["id"].as_u64().unwrap() as u32,
            json["in_count"].as_u64().unwrap() as usize,
        )
    }
}

impl Component for NorGate {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> String {
        Primitive::NorGate.to_string()
    }

    fn ins(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn outs(&mut self) -> &mut Vec<bool> {
        &mut self.outs
    }

    fn on_event(&mut self, event: &CompEvent) {
        if let CompEvent::UpdateValues = event {
            let out: bool = self.ins.as_slice().iter().any(|val| *val);
            self.outs[0] = !out;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NorGate;
    use crate::components::component::{Component, CompEvent};

    #[test]
    fn update_values() {
        let table = [
            [false, false, true],
            [true, false, false],
            [false, true, false],
            [true, true, false],
        ];
        let mut gate = NorGate::new(0, 2);
        for row in table {
            gate.set_in(0, row[0]);
            gate.set_in(1, row[1]);
            gate.on_event(&CompEvent::UpdateValues);
            assert!(gate.outs[0] == row[2]);
        }
    }
}
