use crate::components::component::{Component, SimEvent};

use super::primitives::Primitive;

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
        &mut self.ins
    }

    fn on_event(&mut self, event: &SimEvent) {
        if let SimEvent::UpdateValues = event {
            let out: bool = self.ins.as_slice().iter().any(|val| *val);
            self.outs[0] = out;
        }
    }
}
