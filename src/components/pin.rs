use super::component::{BaseComponent, ComponentBuilder};

pub struct Pin;

impl Pin {
    pub fn new(id: u32) -> BaseComponent {
        ComponentBuilder::new()
            .id(id)
            .upd_fn(|comp| {
                comp.outs[0] = comp.ins[0];
            })
            .build()
    }
}
