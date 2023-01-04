use crate::components::prelude::*;

use super::primitive::Primitive;

/// Represents a NOR gate component.
#[derive(Debug)]
pub struct NorGate {
    pub ins: Vec<bool>,
    pub outs: Vec<bool>,
}

impl NorGate {
    /// Creates a new `NorGate` component given an id and the count of input
    /// pins.
    ///
    /// # Arguments
    ///
    /// * `in_count` - Integer that represent how many input pins the gate has.
    ///
    /// # Example
    ///
    /// ```
    /// # use logix::prelude::NorGate;
    /// #
    /// let gate = NorGate::new(2);
    /// ```
    pub fn new(in_count: usize) -> NorGate {
        NorGate {
            ins: vec![false; in_count],
            outs: vec![false],
        }
    }
}

impl ComponentCast for NorGate {
    fn as_nor_gate(&self) -> Option<&NorGate> {
        Some(self)
    }
    fn as_nor_gate_mut(&mut self) -> Option<&mut NorGate> {
        Some(self)
    }
}

impl Component for NorGate {
    fn name(&self) -> String {
        Primitive::NorGate.to_string()
    }

    fn ins(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn outs(&mut self) -> &mut Vec<bool> {
        &mut self.outs
    }
}
