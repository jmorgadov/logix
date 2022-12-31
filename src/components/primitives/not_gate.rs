use crate::components::component::{Component, SimEvent};

use super::primitives::Primitive;

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
        match event {
            SimEvent::UpdateValues => {
                self.outs[0] = !self.ins[0];
            }
            _ => (),
        }
    }
}
