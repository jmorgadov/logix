use crate::components::prelude::*;

use super::primitive::Primitive;

/// Represents a NAND gate component.
#[derive(Debug)]
pub struct NandGate {
    pub ins: Vec<bool>,
    pub outs: Vec<bool>,
}

impl NandGate {
    /// Creates a new `NandGate` component given an id and the count of input
    /// pins.
    ///
    /// # Arguments
    ///
    /// * `in_count` - Integer that represent how many input pins the gate has.
    ///
    /// # Example
    ///
    /// ```
    /// # use logix::prelude::NandGate;
    /// #
    /// let gate = NandGate::new(2);
    /// ```
    pub fn new(in_count: usize) -> NandGate {
        NandGate {
            ins: vec![false; in_count],
            outs: vec![false],
        }
    }
}

impl ComponentCast for NandGate {
    fn as_nand_gate(&self) -> Option<&NandGate> {
        Some(self)
    }
    fn as_nand_gate_mut(&mut self) -> Option<&mut NandGate> {
        Some(self)
    }
}

impl Component for NandGate {
    fn name(&self) -> String {
        Primitive::NandGate.to_string()
    }

    fn ins(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn outs(&mut self) -> &mut Vec<bool> {
        &mut self.outs
    }
}
