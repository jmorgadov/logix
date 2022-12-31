use crate::components::component::{Component, SimEvent};

use super::primitive::Primitive;

pub struct NotGate {
    id: u32,
    ins: Vec<bool>,
    outs: Vec<bool>,
}

impl NotGate {
    pub fn new(id: u32) -> NotGate {
        NotGate {
            id,
            ins: vec![false],
            outs: vec![false],
        }
    }
}

impl Component for NotGate {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> String {
        Primitive::NotGate.to_string()
    }

    fn ins(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn outs(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn on_event(&mut self, event: &SimEvent) {
        if let SimEvent::UpdateValues = event {
            self.outs[0] = !self.ins[0];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NotGate;
    use crate::components::component::{Component, SimEvent};

    #[test]
    fn update_values() {
        let table = [
            [false, true],
            [true, false],
        ];
        let mut gate = NotGate::new(0);
        for row in table {
            gate.set_in(0, row[0]);
            gate.on_event(&SimEvent::UpdateValues);
            assert!(gate.outs[0] == row[1]);
        }
    }
}
