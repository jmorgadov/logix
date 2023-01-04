use crate::components::prelude::*;

use super::primitive::Primitive;

/// Represents a NOT gate component.
#[derive(Debug)]
pub struct NotGate {
    pub ins: Vec<bool>,
    pub outs: Vec<bool>,
}

impl NotGate {
    /// Creates a new `NotGate` component given an id.
    ///
    /// # Example
    ///
    /// ```
    /// # use logix::prelude::NotGate;
    /// #
    /// let gate = NotGate::new();
    /// ```
    pub fn new() -> NotGate {
        NotGate {
            ins: vec![false],
            outs: vec![false],
        }
    }
}

impl Default for NotGate {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentCast for NotGate {
    fn as_not_gate(&self) -> Option<&NotGate> {
        Some(self)
    }
    fn as_not_gate_mut(&mut self) -> Option<&mut NotGate> {
        Some(self)
    }
}

impl Component for NotGate {
    fn name(&self) -> String {
        Primitive::NotGate.to_string()
    }

    fn ins(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn outs(&mut self) -> &mut Vec<bool> {
        &mut self.outs
    }
}
