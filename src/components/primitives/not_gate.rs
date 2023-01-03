use crate::components::prelude::*;

use super::primitive::Primitive;

/// Represents a NOT gate component.
#[derive(Debug)]
pub struct NotGate {
    pub ins: Vec<bool>,
    pub outs: Vec<bool>,
}

impl NotGate {
    /// Creates a new `OutputPin` component given an id.
    ///
    /// # Example
    ///
    /// ```
    /// let gate = OutputPin::new(0);
    /// ```
    pub fn new() -> NotGate {
        NotGate {
            ins: vec![false],
            outs: vec![false],
        }
    }
}

impl ComponentCast for NotGate {
    fn as_not_gate(&self) -> Option<&NotGate> {
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

    fn on_event(&mut self, event: &CompEvent) {
        if let CompEvent::UpdateValues = event {
            self.outs[0] = !self.ins[0];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NotGate;
    use crate::components::component::{CompEvent, Component};

    #[test]
    fn update_values() {
        let table = [[false, true], [true, false]];
        let mut gate = NotGate::new();
        for row in table {
            gate.set_in(0, row[0]);
            gate.on_event(&CompEvent::UpdateValues);
            assert!(gate.outs[0] == row[1]);
        }
    }
}
