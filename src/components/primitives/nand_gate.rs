use crate::{
    components::component::{Component, CompEvent},
    serialize::JSONSerialize,
};

use super::primitive::Primitive;

/// Represents a NAND gate component.
#[derive(Debug)]
pub struct NandGate {
    id: u32,
    ins: Vec<bool>,
    outs: Vec<bool>,
}

impl NandGate {
    /// Creates a new `NandGate` component given an id and the count of input
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
    /// let gate = NandGate::new(0, 2);
    /// ```
    pub fn new(id: u32, in_count: usize) -> NandGate {
        NandGate {
            id,
            ins: vec![false; in_count],
            outs: vec![false],
        }
    }
}

impl JSONSerialize for NandGate {
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "name": Primitive::NandGate.to_string(),
            "in_count": self.ins.len(),
        })
    }

    fn from_json(json: &serde_json::Value) -> Self
    where
        Self: Sized,
    {
        NandGate::new(
            json["id"].as_u64().unwrap() as u32,
            json["in_count"].as_u64().unwrap() as usize,
        )
    }
}

impl Component for NandGate {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> String {
        Primitive::NandGate.to_string()
    }

    fn ins(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn outs(&mut self) -> &mut Vec<bool> {
        &mut self.outs
    }

    fn on_event(&mut self, event: &CompEvent) {
        if let CompEvent::UpdateValues = event {
            let out: bool = self.ins.as_slice().iter().all(|val| *val);
            self.outs[0] = !out;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NandGate;
    use crate::components::component::{Component, CompEvent};

    #[test]
    fn update_values() {
        let table = [
            [false, false, true],
            [true, false, true],
            [false, true, true],
            [true, true, false],
        ];
        let mut gate = NandGate::new(0, 2);
        for row in table {
            gate.set_in(0, row[0]);
            gate.set_in(1, row[1]);
            gate.on_event(&CompEvent::UpdateValues);
            assert!(gate.outs[0] == row[2]);
        }
    }
}
