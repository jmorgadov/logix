use std::fmt::Debug;

use crate::serialize::JSONSerialize;

pub enum SimEvent {
    Update(u128),
    UpdateValues,
}

pub trait Component: Debug + JSONSerialize {
    fn id(&self) -> u32;
    fn name(&self) -> String;
    fn ins(&mut self) -> &mut Vec<bool>;
    fn outs(&mut self) -> &mut Vec<bool>;
    fn set_in(&mut self, idx: usize, val: bool) {
        assert!(
            idx < self.ins().len(),
            "Invalid index {} for component {} with {} inputs.",
            idx,
            self.name(),
            self.ins().len()
        );
        self.ins()[idx] = val;
    }
    fn set_out(&mut self, idx: usize, val: bool) {
        assert!(
            idx < self.outs().len(),
            "Invalid index {} for component {} with {} inputs.",
            idx,
            self.name(),
            self.outs().len()
        );
        self.outs()[idx] = val;
    }
    fn is_dirty(&self) -> bool {
        false
    }
    fn on_event(&mut self, _event: &SimEvent) {}
}
