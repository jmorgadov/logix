use super::component::{BaseComponent, Component, ComponentBuilder};

pub struct Clock {
    base: BaseComponent,
    val: bool,
    interval: u128,
    full: u128,
    dirty: bool,
}

impl Clock {
    pub fn new(id: u32, interval: u128) -> Self {
        Clock {
            base: ComponentBuilder::new()
                .id(id)
                .input_count(0)
                .build(),
            val: false,
            interval,
            full: interval * 2,
            dirty: false,
        }
    }
}

impl Component for Clock {
    fn id(&self) -> u32 {
        self.base.id()
    }

    fn ins(&self) -> &Vec<bool> {
        self.base.ins()
    }

    fn outs(&self) -> &Vec<bool> {
        self.base.outs()
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn check_values(&mut self) {
        self.set_out(0, self.val);
        self.dirty = false;
    }

    fn set_in(&mut self, idx: usize, val: bool) {
        self.base.set_in(idx, val);
    }

    fn set_out(&mut self, idx: usize, val: bool) {
        self.base.set_out(idx, val)
    }

    fn update(&mut self, time: u128) {
        self.val = (time % self.full) > self.interval;
        self.dirty = self.base.outs[0] != self.val;
    }
}
