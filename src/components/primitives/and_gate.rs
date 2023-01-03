use crate::components::prelude::*;

use super::primitive::Primitive;

/// Represents an AND gate component.
#[derive(Debug)]
pub struct AndGate {
    pub id: u32,
    pub ins: Vec<bool>,
    pub outs: Vec<bool>,
}

impl AndGate {
    /// Creates a new `AndGate` component given an id and the count of input
    /// pins.
    ///
    /// # Arguments
    ///
    /// * `id` - Integer that represents the component id.
    /// * `in_count` - Integer that represent how many input pins the gate has.
    ///
    /// # Example
    ///
    /// ```
    /// let gate = AndGate::new(0, 2);
    /// ```
    pub fn new(id: u32, in_count: usize) -> AndGate {
        AndGate {
            id,
            ins: vec![false; in_count],
            outs: vec![false],
        }
    }
}

impl ComponentCast for AndGate {
    fn as_and_gate(&self) -> Option<&AndGate> {
        Some(self)
    }
}

impl Component for AndGate {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> String {
        Primitive::AndGate.to_string()
    }

    fn ins(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn outs(&mut self) -> &mut Vec<bool> {
        &mut self.outs
    }

    fn on_event(&mut self, event: &CompEvent) {
        if let CompEvent::UpdateValues = event {
            let out: bool = self.ins.as_slice().iter().all(|val| *val);
            self.outs[0] = out;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AndGate;
    use crate::components::component::{CompEvent, Component};

    #[test]
    fn update_values() {
        let table = [
            [false, false, false],
            [true, false, false],
            [false, true, false],
            [true, true, true],
        ];
        let mut gate = AndGate::new(0, 2);
        for row in table {
            gate.set_in(0, row[0]);
            gate.set_in(1, row[1]);
            gate.on_event(&CompEvent::UpdateValues);
            assert!(gate.outs[0] == row[2]);
        }
    }
}
