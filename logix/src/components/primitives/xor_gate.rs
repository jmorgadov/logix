use crate::components::prelude::*;

use super::primitive::Primitive;

/// Represents an XOR gate component.
#[derive(Debug)]
pub struct XorGate {
    pub ins: Vec<bool>,
    pub outs: Vec<bool>,
}

impl XorGate {
    /// Creates a new `XorGate` component given an id and the count of input
    /// pins.
    ///
    /// # Arguments
    ///
    /// * `in_count` - Integer that represent how many input pins the gate has.
    ///
    /// # Example
    ///
    /// ```
    /// # use logix::prelude::XorGate;
    /// #
    /// let gate = XorGate::new(2);
    /// ```
    pub fn new(in_count: usize) -> XorGate {
        XorGate {
            ins: vec![false; in_count],
            outs: vec![false],
        }
    }
}

impl ComponentCast for XorGate {
    fn as_xor_gate(&self) -> Option<&XorGate> {
        Some(self)
    }
    fn as_xor_gate_mut(&mut self) -> Option<&mut XorGate> {
        Some(self)
    }
}

impl Component for XorGate {
    fn name(&self) -> String {
        Primitive::XorGate.to_string()
    }

    fn ins(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn outs(&mut self) -> &mut Vec<bool> {
        &mut self.outs
    }
}
