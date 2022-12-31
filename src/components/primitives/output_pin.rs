use crate::components::component::{Component, SimEvent};

use super::primitives::Primitive;

pub struct OutputPin {
    id: u32,
    ins: Vec<bool>,
    outs: Vec<bool>,
}

impl OutputPin {
    pub fn new(id: u32) -> Self {
        OutputPin {
            id,
            ins: vec![false],
            outs: vec![false],
        }
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

    fn on_event(&mut self, event: &SimEvent) {
        if let SimEvent::UpdateValues = event {
            self.outs[0] = self.ins[0];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{
        component::{Component, SimEvent},
        primitives::output_pin::OutputPin,
    };

    #[test]
    fn output_pin() {
        let comp = &mut OutputPin::new(0);

        comp.set_in(0, false);
        comp.on_event(&SimEvent::UpdateValues);
        assert!(!comp.outs[0]);

        comp.set_in(0, true);
        comp.on_event(&SimEvent::UpdateValues);
        assert!(comp.outs[0]);
    }
}
