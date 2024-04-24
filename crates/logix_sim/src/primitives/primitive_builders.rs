use super::{prelude::ExtraInfo, primitives::Primitive};
use crate::bit::Bit;
use logix_core::prelude::*;

fn base_component_extra(
    name: &str,
    in_count: usize,
    out_count: usize,
    info: ExtraInfo,
) -> Component<Bit, ExtraInfo> {
    ComponentBuilder::new(name)
        .port_count(in_count, out_count)
        .extra(info)
        .build()
}

/// Creates a NOT gate component.
///
/// # Example
///
/// ```
/// # use logix_sim::primitives::prelude::*;
/// #
/// let comp = not_gate();
/// ```
pub fn not_gate(id: String) -> Component<Bit, ExtraInfo> {
    base_component_extra(
        &Primitive::NotGate.to_string(),
        1,
        1,
        ExtraInfo::from_primitive(id, Primitive::NotGate),
    )
}

/// Creates an AND gate component.
///
/// # Arguments
///
/// * `in_count` - Amount of input ports.
///
/// # Example
///
/// ```
/// # use logix_sim::primitives::prelude::*;
/// #
/// let comp = and_gate(2);
/// ```
pub fn and_gate(id: String, in_count: usize) -> Component<Bit, ExtraInfo> {
    base_component_extra(
        &Primitive::AndGate.to_string(),
        in_count,
        1,
        ExtraInfo::from_primitive(id, Primitive::AndGate),
    )
}

pub fn or_gate(id: String, in_count: usize) -> Component<Bit, ExtraInfo> {
    base_component_extra(
        &Primitive::OrGate.to_string(),
        in_count,
        1,
        ExtraInfo::from_primitive(id, Primitive::OrGate),
    )
}

pub fn nand_gate(id: String, in_count: usize) -> Component<Bit, ExtraInfo> {
    base_component_extra(
        &Primitive::NandGate.to_string(),
        in_count,
        1,
        ExtraInfo::from_primitive(id, Primitive::NandGate),
    )
}

pub fn nor_gate(id: String, in_count: usize) -> Component<Bit, ExtraInfo> {
    base_component_extra(
        &Primitive::NorGate.to_string(),
        in_count,
        1,
        ExtraInfo::from_primitive(id, Primitive::NorGate),
    )
}

pub fn xor_gate(id: String, in_count: usize) -> Component<Bit, ExtraInfo> {
    base_component_extra(
        &Primitive::XorGate.to_string(),
        in_count,
        1,
        ExtraInfo::from_primitive(id, Primitive::XorGate),
    )
}

pub fn clock(id: String, frec: f64) -> Component<Bit, ExtraInfo> {
    let frec_in_nano = (1e9 / frec) as u128;
    let prim = Primitive::Clock {
        period: frec_in_nano,
    };
    let mut comp =
        base_component_extra(&prim.to_string(), 0, 1, ExtraInfo::from_primitive(id, prim));
    comp.outputs[0] = false;
    comp
}

pub fn high_const(id: String) -> Component<Bit, ExtraInfo> {
    let prim = Primitive::Const { value: true };
    let mut comp =
        base_component_extra(&prim.to_string(), 0, 1, ExtraInfo::from_primitive(id, prim));
    comp.outputs[0] = true;
    comp
}

pub fn low_const(id: String) -> Component<Bit, ExtraInfo> {
    let prim = Primitive::Const { value: false };
    let mut comp =
        base_component_extra(&prim.to_string(), 0, 1, ExtraInfo::from_primitive(id, prim));
    comp.outputs[0] = false;
    comp
}
