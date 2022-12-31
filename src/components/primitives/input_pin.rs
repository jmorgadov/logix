use crate::components::component::{Component, SimEvent};

use super::primitives::Primitive;

pub struct InputPin {
    id: u32,
    ins: Vec<bool>,
    outs: Vec<bool>,
}

impl InputPin {
    pub fn new(id: u32) -> Self {
        InputPin {
            id,
            ins: vec![false],
            outs: vec![false],
        }
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

    fn on_event(&mut self, event: &SimEvent) {
        match event {
            SimEvent::UpdateValues => {
                self.outs[0] = self.ins[0];
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{
        component::{Component, SimEvent},
        primitives::input_pin::InputPin,
    };

    #[test]
    fn input_pin() {
        let comp = &mut InputPin::new(0);

        comp.set_in(0, false);
        comp.on_event(&SimEvent::UpdateValues);
        assert!(!comp.outs[0]);

        comp.set_in(0, true);
        comp.on_event(&SimEvent::UpdateValues);
        assert!(comp.outs[0]);
    }
}
