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
    /// let gate = XorGate::new(0, 2);
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

    fn on_event(&mut self, event: &CompEvent) {
        if let CompEvent::UpdateValues = event {
            let mut out = false;
            for i in 1..self.ins.len() {
                if self.ins[i - 1] != self.ins[i] {
                    out = true;
                    break;
                }
            }
            self.outs[0] = out;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::XorGate;
    use crate::components::component::{CompEvent, Component};

    #[test]
    fn update_values() {
        let table = [
            [false, false, false],
            [true, false, true],
            [false, true, true],
            [true, true, false],
        ];
        let mut gate = XorGate::new(2);
        for row in table {
            gate.set_in(0, row[0]);
            gate.set_in(1, row[1]);
            gate.on_event(&CompEvent::UpdateValues);
            assert!(gate.outs[0] == row[2]);
        }
    }
}
