use crate::components::component::{Component, SimEvent};

use super::primitives::Primitive;

pub struct Clock {
    id: u32,
    ins: Vec<bool>,
    outs: Vec<bool>,

    val: bool,
    interval: u128,
    full: u128,
    dirty: bool,
}

impl Clock {
    pub fn new(id: u32, frecuency: f64) -> Self {
        let nano_sec_dur: u128 = (1e9 / frecuency) as u128;
        Clock {
            id,
            ins: vec![],
            outs: vec![false],
            val: false,
            interval: nano_sec_dur,
            full: nano_sec_dur * 2,
            dirty: false,
        }
    }
}

impl Component for Clock {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> String {
        Primitive::Clock.to_string()
    }

    fn ins(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn outs(&mut self) -> &mut Vec<bool> {
        &mut self.outs
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn on_event(&mut self, event: &SimEvent) {
        match event {
            SimEvent::Update(time) => {
                self.val = (time % self.full) > self.interval;
                self.dirty = self.outs[0] != self.val;
            }
            SimEvent::UpdateValues => {
                self.outs[0] = self.val;
                self.dirty = false;
            }
        }
    }
}
