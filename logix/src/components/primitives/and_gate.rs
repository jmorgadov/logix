use crate::components::prelude::*;

use super::primitive::Primitive;

/// Represents an AND gate component.
#[derive(Debug)]
pub struct AndGate {
    pub ins: Vec<bool>,
    pub outs: Vec<bool>,
}

impl AndGate {
    /// Creates a new `AndGate` component given an id and the count of input
    /// pins.
    ///
    /// # Arguments
    ///
    /// * `in_count` - Integer that represent how many input pins the gate has.
    ///
    /// # Example
    ///
    /// ```
    /// # use logix::prelude::AndGate;
    /// #
    /// let gate = AndGate::new(2);
    /// ```
    pub fn new(in_count: usize) -> AndGate {
        AndGate {
            ins: vec![false; in_count],
            outs: vec![false],
        }
    }
}

impl ComponentCast for AndGate {
    fn as_and_gate(&self) -> Option<&AndGate> {
        Some(self)
    }
    fn as_and_gate_mut(&mut self) -> Option<&mut AndGate> {
        Some(self)
    }
}

impl Component for AndGate {
    fn name(&self) -> String {
        Primitive::AndGate.to_string()
    }

    fn ins(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn outs(&mut self) -> &mut Vec<bool> {
        &mut self.outs
    }
}
