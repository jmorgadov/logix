use crate::components::component::{Component, SimEvent};

use super::primitive::Primitive;

#[derive(Debug)]
pub struct OrGate {
    id: u32,
    ins: Vec<bool>,
    outs: Vec<bool>,
}

impl OrGate {
    pub fn new(id: u32, in_count: usize) -> OrGate {
        OrGate {
            id,
            ins: vec![false; in_count],
            outs: vec![false],
        }
    }
}

impl Component for OrGate {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> String {
        Primitive::OrGate.to_string()
    }

    fn ins(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn outs(&mut self) -> &mut Vec<bool> {
        &mut self.outs
    }

    fn on_event(&mut self, event: &SimEvent) {
        if let SimEvent::UpdateValues = event {
            let out: bool = self.ins.as_slice().iter().any(|val| *val);
            self.outs[0] = out;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OrGate;
    use crate::components::component::{Component, SimEvent};

    #[test]
    fn update_values() {
        let table = [
            [false, false, false],
            [true, false, true],
            [false, true, true],
            [true, true, true],
        ];
        let mut gate = OrGate::new(0, 2);
        for row in table {
            gate.set_in(0, row[0]);
            gate.set_in(1, row[1]);
            gate.on_event(&SimEvent::UpdateValues);
            assert!(gate.outs[0] == row[2]);
        }
    }
}
