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
    /// let gate = OrGate::new(0, 2);
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

    fn on_event(&mut self, event: &CompEvent) {
        if let CompEvent::UpdateValues = event {
            let out: bool = self.ins.as_slice().iter().any(|val| *val);
            self.outs[0] = out;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OrGate;
    use crate::components::component::{CompEvent, Component};

    #[test]
    fn update_values() {
        let table = [
            [false, false, false],
            [true, false, true],
            [false, true, true],
            [true, true, true],
        ];
        let mut gate = OrGate::new(2);
        for row in table {
            gate.set_in(0, row[0]);
            gate.set_in(1, row[1]);
            gate.on_event(&CompEvent::UpdateValues);
            assert!(gate.outs[0] == row[2]);
        }
    }
}
