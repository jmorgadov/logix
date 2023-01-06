use super::prelude::Primitive;
use logix_core::prelude::*;

fn base_component(name: &str, in_count: usize, out_count: usize) -> Component {
    ComponentBuilder::new(name)
        .port_count(in_count, out_count)
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
pub fn not_gate() -> Component {
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
pub fn and_gate(in_count: usize) -> Component {
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
pub fn or_gate(in_count: usize) -> Component {
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
pub fn nand_gate(in_count: usize) -> Component {
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
pub fn nor_gate(in_count: usize) -> Component {
    base_component(&Primitive::NorGate.to_string(), in_count, 1)
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
pub fn clock(frec: f64) -> Component {
    let frec_in_nano = (1e9 / frec) as u128;
    ComponentBuilder::new(&Primitive::Clock.to_string())
        .port_count(0, 1)
        .info(frec_in_nano.to_ne_bytes().to_vec())
        .build()
}

/// Creates a Const component.
///
/// # Arguments
///
/// * `val` - A bool that represents the value the constant will emit.
///
/// # Example
///
/// ```
/// # use logix_sim::primitives::prelude::*;
/// #
/// let comp = constant(true);
/// ```
pub fn constant(val: bool) -> Component {
    let mut comp = base_component(&Primitive::Const.to_string(), 0, 1);
    comp.outputs[0] = val;
    comp
}
