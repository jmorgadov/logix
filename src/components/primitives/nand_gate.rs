use crate::components::component::{Component, SimEvent};

use super::primitive::Primitive;

pub struct NandGate {
    id: u32,
    ins: Vec<bool>,
    outs: Vec<bool>,
}

impl NandGate {
    pub fn new(id: u32, in_count: usize) -> NandGate {
        NandGate {
            id,
            ins: vec![false; in_count],
            outs: vec![false],
        }
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
        &mut self.ins
    }

    fn on_event(&mut self, event: &SimEvent) {
        if let SimEvent::UpdateValues = event {
            let out: bool = self.ins.as_slice().iter().all(|val| *val);
            self.outs[0] = !out;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NandGate;
    use crate::components::component::{Component, SimEvent};

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
            gate.on_event(&SimEvent::UpdateValues);
            assert!(gate.outs[0] == row[2]);
        }
    }
}
