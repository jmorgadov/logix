use crate::components::prelude::*;

use super::primitive::Primitive;

/// Represents an OR gate component.
#[derive(Debug)]
pub struct OrGate {
    pub ins: Vec<bool>,
    pub outs: Vec<bool>,
}

impl OrGate {
    /// Creates a new `OrGate` component given an id and the count of input
    /// pins.
    ///
    /// # Arguments
    ///
    /// * `in_count` - Integer that represent how many input pins the gate has.
    ///
    /// # Example
    ///
    /// ```
    /// # use logix::prelude::OrGate;
    /// #
    /// let gate = OrGate::new(2);
    /// ```
    pub fn new(in_count: usize) -> OrGate {
        OrGate {
            ins: vec![false; in_count],
            outs: vec![false],
        }
    }
}

impl ComponentCast for OrGate {
    fn as_or_gate(&self) -> Option<&OrGate> {
        Some(self)
    }
    fn as_or_gate_mut(&mut self) -> Option<&mut OrGate> {
        Some(self)
    }
}

impl Component for OrGate {
    fn name(&self) -> String {
        Primitive::OrGate.to_string()
    }

    fn ins(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn outs(&mut self) -> &mut Vec<bool> {
        &mut self.outs
    }
}
