use crate::components::component::{Component, SimEvent};

use super::primitives::Primitive;

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
