use crate::component::{Component, ComponentBuilder};

pub struct LogicGate;

impl LogicGate {
    pub fn and(id: &str, in_count: usize) -> Component {
        ComponentBuilder::new()
            .id(id.to_string())
            .upd_fn(|comp| {
                let out: bool = comp.ins.as_slice().iter().all(|e| *e);
                comp.outs[0] = out;
            })
            .input_count(in_count)
            .build()
    }

    pub fn or(id: &str, in_count: usize) -> Component {
        ComponentBuilder::new()
            .id(id.to_string())
            .upd_fn(|comp| {
                let out: bool = comp.ins.as_slice().iter().any(|e| *e);
                comp.outs[0] = out;
            })
            .input_count(in_count)
            .build()
    }

    pub fn not(id: &str) -> Component {
        ComponentBuilder::new()
            .id(id.to_string())
            .upd_fn(|comp| {
                comp.outs[0] = !comp.ins[0];
            })
            .build()
    }
}
