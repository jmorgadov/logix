use super::component::{BaseComponent, ComponentBuilder};

pub struct Pin;

impl Pin {
    pub fn input(id: u32) -> BaseComponent {
        ComponentBuilder::new()
            .name("PinInput")
            .id(id)
            .upd_fn(|comp| {
                comp.outs[0] = comp.ins[0];
            })
            .build()
    }

    pub fn output(id: u32) -> BaseComponent {
        ComponentBuilder::new()
            .name("PinOutput")
            .id(id)
            .upd_fn(|comp| {
                comp.outs[0] = comp.ins[0];
            })
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::Pin;
    use crate::components::component::Component;

    #[test]
    fn pin_input() {
        let comp = &mut Pin::input(0);

        comp.set_ins(vec![false]);
        comp.check_values();
        assert!(!comp.outs[0]);

        comp.set_ins(vec![true]);
        comp.check_values();
        assert!(comp.outs[0]);
    }

    #[test]
    fn pin_output() {
        let comp = &mut Pin::output(0);

        comp.set_ins(vec![false]);
        comp.check_values();
        assert!(!comp.outs[0]);

        comp.set_ins(vec![true]);
        comp.check_values();
        assert!(comp.outs[0]);
    }
}
