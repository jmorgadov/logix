use crate::{
    components::component::{Component, SimEvent},
    serialize::JSONSerialize,
};

use super::primitive::Primitive;

#[derive(Debug)]
pub struct XorGate {
    id: u32,
    ins: Vec<bool>,
    outs: Vec<bool>,
}

impl XorGate {
    pub fn new(id: u32, in_count: usize) -> XorGate {
        XorGate {
            id,
            ins: vec![false; in_count],
            outs: vec![false],
        }
    }
}

impl JSONSerialize for XorGate {
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "name": Primitive::XorGate.to_string(),
            "in_count": self.ins.len(),
        })
    }

    fn from_json(json: &serde_json::Value) -> Self
    where
        Self: Sized,
    {
        XorGate::new(
            json["id"].as_u64().unwrap() as u32,
            json["in_count"].as_u64().unwrap() as usize,
        )
    }
}

impl Component for XorGate {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> String {
        Primitive::XorGate.to_string()
    }

    fn ins(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn outs(&mut self) -> &mut Vec<bool> {
        &mut self.outs
    }

    fn on_event(&mut self, event: &SimEvent) {
        if let SimEvent::UpdateValues = event {
            let mut out = false;
            for i in 1..self.ins.len() {
                if self.ins[i - 1] != self.ins[i] {
                    out = true;
                    break;
                }
            }
            self.outs[0] = out;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::XorGate;
    use crate::components::component::{Component, SimEvent};

    #[test]
    fn update_values() {
        let table = [
            [false, false, false],
            [true, false, true],
            [false, true, true],
            [true, true, false],
        ];
        let mut gate = XorGate::new(0, 2);
        for row in table {
            gate.set_in(0, row[0]);
            gate.set_in(1, row[1]);
            gate.on_event(&SimEvent::UpdateValues);
            assert!(gate.outs[0] == row[2]);
        }
    }
}
