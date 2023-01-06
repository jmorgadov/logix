use logix::prelude::*;

use crate::primitive::Primitive;

fn base_component(name: &str, in_count: usize, out_count: usize) -> Component {
    ComponentBuilder::new(name)
        .port_count(in_count, out_count)
        .build()
}

pub fn not_gate() -> Component {
    base_component(&Primitive::NotGate.to_string(), 1, 1)
}

pub fn and_gate(in_count: usize) -> Component {
    base_component(&Primitive::AndGate.to_string(), in_count, 1)
}

pub fn or_gate(in_count: usize) -> Component {
    base_component(&Primitive::OrGate.to_string(), in_count, 1)
}

pub fn nand_gate(in_count: usize) -> Component {
    base_component(&Primitive::NandGate.to_string(), in_count, 1)
}

pub fn nor_gate(in_count: usize) -> Component {
    base_component(&Primitive::NorGate.to_string(), in_count, 1)
}

pub fn clock(frec: f64) -> Component {
    let frec_in_nano = (1e9 / frec) as u128;
    ComponentBuilder::new(&Primitive::Clock.to_string())
        .port_count(0, 1)
        .info(frec_in_nano.to_ne_bytes().to_vec())
        .build()
}

pub fn constant(val: bool) -> Component {
    let mut comp = base_component(&Primitive::Const.to_string(), 0, 1);
    comp.outputs[0] = val;
    comp
}
