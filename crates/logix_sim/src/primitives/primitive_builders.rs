use super::prelude::Primitive;
use crate::bit::BitArray;
use logix_core::prelude::*;

#[derive(Debug, Clone, Default)]
pub enum BaseExtra {
    #[default]
    NoExtra,
    Clock(u128),
}

fn base_component(name: &str, in_count: usize, out_count: usize) -> Component<BitArray, BaseExtra> {
    base_component_extra(name, in_count, out_count, BaseExtra::NoExtra)
}

fn base_component_extra(
    name: &str,
    in_count: usize,
    out_count: usize,
    info: BaseExtra,
) -> Component<BitArray, BaseExtra> {
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
pub fn not_gate() -> Component<BitArray, BaseExtra> {
    base_component(&Primitive::NotGate.to_string(), 1, 1)
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
pub fn and_gate(in_count: usize) -> Component<BitArray, BaseExtra> {
    base_component(&Primitive::AndGate.to_string(), in_count, 1)
}

/// Creates an OR gate component.
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
/// let comp = or_gate(2);
/// ```
pub fn or_gate(in_count: usize) -> Component<BitArray, BaseExtra> {
    base_component(&Primitive::OrGate.to_string(), in_count, 1)
}

/// Creates a NAND gate component.
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
/// let comp = nand_gate(2);
/// ```
pub fn nand_gate(in_count: usize) -> Component<BitArray, BaseExtra> {
    base_component(&Primitive::NandGate.to_string(), in_count, 1)
}

/// Creates a NOR gate component.
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
/// let comp = nor_gate(2);
/// ```
pub fn nor_gate(in_count: usize) -> Component<BitArray, BaseExtra> {
    base_component(&Primitive::NorGate.to_string(), in_count, 1)
}

/// Creates a XOR gate component.
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
/// let comp = xor_gate(2);
/// ```
pub fn xor_gate(in_count: usize) -> Component<BitArray, BaseExtra> {
    base_component(&Primitive::XorGate.to_string(), in_count, 1)
}

/// Creates a Clock component.
///
/// # Arguments
///
/// * `frec` - A float that represents the frequency of the clock in Hertz.
///
/// # Example
///
/// ```
/// # use logix_sim::primitives::prelude::*;
/// #
/// let comp = clock(4.0); // 4Hz - 250ms
/// ```
pub fn clock(frec: f64) -> Component<BitArray, BaseExtra> {
    let frec_in_nano = (1e9 / frec) as u128;
    let mut comp = base_component_extra(
        &Primitive::Clock.to_string(),
        0,
        1,
        BaseExtra::Clock(frec_in_nano),
    );
    comp.outputs[0].set(0);
    comp
}

/// Creates a HighConst component.
///
/// # Example
///
/// ```
/// # use logix_sim::primitives::prelude::*;
/// #
/// let comp = high_const();
/// ```
pub fn high_const() -> Component<BitArray, BaseExtra> {
    let mut comp = base_component(&Primitive::HighConst.to_string(), 0, 1);
    comp.outputs[0].set(1);
    comp
}

/// Creates a LowConst component.
///
/// # Example
///
/// ```
/// # use logix_sim::primitives::prelude::*;
/// #
/// let comp = low_const();
/// ```
pub fn low_const() -> Component<BitArray, BaseExtra> {
    let mut comp = base_component(&Primitive::HighConst.to_string(), 0, 1);
    comp.outputs[0].set(0);
    comp
}
