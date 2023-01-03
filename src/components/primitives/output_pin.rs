use crate::components::prelude::*;

use super::primitive::Primitive;

/// Represents an output pin.
///
/// This component must be use when creating composed components that have
/// external outputs.
#[derive(Debug)]
pub struct OutputPin {
    pub id: u32,
    pub ins: Vec<bool>,
    pub outs: Vec<bool>,
}

impl OutputPin {
    /// Creates a new `OutputPin` component given an id.
    ///
    /// # Arguments
    ///
    /// * `id` - Integer that represents the component id.
    ///
    /// # Example
    ///
    /// ```
    /// let gate = OutputPin::new(0);
    /// ```
    pub fn new(id: u32) -> Self {
        OutputPin {
            id,
            ins: vec![false],
            outs: vec![false],
        }
    }
}

impl ComponentCast for OutputPin {
    fn as_output_pin(&self) -> Option<&OutputPin> {
        Some(self)
    }
}

impl Component for OutputPin {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> String {
        Primitive::OutputPin.to_string()
    }

    fn ins(&mut self) -> &mut Vec<bool> {
        &mut self.ins
    }

    fn outs(&mut self) -> &mut Vec<bool> {
        &mut self.outs
    }

    fn on_event(&mut self, event: &CompEvent) {
        if let CompEvent::UpdateValues = event {
            self.outs[0] = self.ins[0];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{
        component::{CompEvent, Component},
        primitives::output_pin::OutputPin,
    };

    #[test]
    fn output_pin() {
        let comp = &mut OutputPin::new(0);

        comp.set_in(0, false);
        comp.on_event(&CompEvent::UpdateValues);
        assert!(!comp.outs[0]);

        comp.set_in(0, true);
        comp.on_event(&CompEvent::UpdateValues);
        assert!(comp.outs[0]);
    }
}
