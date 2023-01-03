use crate::components::prelude::*;

use super::primitive::Primitive;

/// Represents an input pin.
///
/// This component must be use when creating composed components that have
/// external inputs.
#[derive(Debug)]
pub struct InputPin {
    pub id: u32,
    pub ins: Vec<bool>,
    pub outs: Vec<bool>,
}

impl InputPin {
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
        InputPin {
            id,
            ins: vec![false],
            outs: vec![false],
        }
    }
}

impl ComponentCast for InputPin {
    fn as_input_pin(&self) -> Option<&InputPin> {
        Some(self)
    }
}

impl Component for InputPin {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> String {
        Primitive::InputPin.to_string()
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
        primitives::input_pin::InputPin,
    };

    #[test]
    fn input_pin() {
        let comp = &mut InputPin::new(0);

        comp.set_in(0, false);
        comp.on_event(&CompEvent::UpdateValues);
        assert!(!comp.outs[0]);

        comp.set_in(0, true);
        comp.on_event(&CompEvent::UpdateValues);
        assert!(comp.outs[0]);
    }
}
