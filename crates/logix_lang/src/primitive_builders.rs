use asmhdl::Data;
use logix_core::prelude::*;
use logix_sim::primitives::prelude::{ExtraInfo, Primitive};

fn base_component_extra(
    id: usize,
    in_count: usize,
    out_count: usize,
    info: ExtraInfo,
) -> Component<ExtraInfo> {
    let name = info.primitive.as_ref().unwrap().to_string();
    ComponentBuilder::new(id)
        .port_count(in_count, out_count)
        .name(name)
        .extra(info)
        .build()
}

pub fn not_gate(id: usize) -> Component<ExtraInfo> {
    base_component_extra(id, 1, 1, ExtraInfo::from_primitive(id, Primitive::NotGate))
}

pub fn and_gate(id: usize, in_count: usize) -> Component<ExtraInfo> {
    base_component_extra(
        id,
        in_count,
        1,
        ExtraInfo::from_primitive(id, Primitive::AndGate),
    )
}

pub fn or_gate(id: usize, in_count: usize) -> Component<ExtraInfo> {
    base_component_extra(
        id,
        in_count,
        1,
        ExtraInfo::from_primitive(id, Primitive::OrGate),
    )
}

pub fn nand_gate(id: usize, in_count: usize) -> Component<ExtraInfo> {
    base_component_extra(
        id,
        in_count,
        1,
        ExtraInfo::from_primitive(id, Primitive::NandGate),
    )
}

pub fn nor_gate(id: usize, in_count: usize) -> Component<ExtraInfo> {
    base_component_extra(
        id,
        in_count,
        1,
        ExtraInfo::from_primitive(id, Primitive::NorGate),
    )
}

pub fn xor_gate(id: usize, in_count: usize) -> Component<ExtraInfo> {
    base_component_extra(
        id,
        in_count,
        1,
        ExtraInfo::from_primitive(id, Primitive::XorGate),
    )
}

pub fn input(id: usize, bits: usize) -> Component<ExtraInfo> {
    let prim = Primitive::Input { bits };
    base_component_extra(id, 1, 1, ExtraInfo::from_primitive(id, prim))
}

pub fn output(id: usize, bits: usize) -> Component<ExtraInfo> {
    let prim = Primitive::Output { bits };
    base_component_extra(id, 1, 1, ExtraInfo::from_primitive(id, prim))
}

pub fn splitter(id: usize, bits: usize) -> Component<ExtraInfo> {
    let prim = Primitive::Splitter { bits };
    base_component_extra(id, 1, bits, ExtraInfo::from_primitive(id, prim))
}

pub fn joiner(id: usize, bits: usize) -> Component<ExtraInfo> {
    let prim = Primitive::Joiner { bits };
    base_component_extra(id, bits, 1, ExtraInfo::from_primitive(id, prim))
}

pub fn clock(id: usize, frec: f64) -> Component<ExtraInfo> {
    let frec_in_nano = (1e9 / frec) as u128;
    let prim = Primitive::Clock {
        period: frec_in_nano,
    };
    base_component_extra(id, 0, 1, ExtraInfo::from_primitive(id, prim))
}

pub fn high_const(id: usize) -> Component<ExtraInfo> {
    let prim = Primitive::Const {
        value: Data::high(),
    };
    base_component_extra(id, 0, 1, ExtraInfo::from_primitive(id, prim))
}

pub fn low_const(id: usize) -> Component<ExtraInfo> {
    let prim = Primitive::Const { value: Data::low() };
    base_component_extra(id, 0, 1, ExtraInfo::from_primitive(id, prim))
}
