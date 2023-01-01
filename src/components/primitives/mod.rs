pub mod and_gate;
pub mod clock;
pub mod constant;
pub mod input_pin;
pub mod nand_gate;
pub mod nor_gate;
pub mod not_gate;
pub mod or_gate;
pub mod output_pin;
pub mod primitive;
pub mod xor_gate;

pub mod prelude {
    pub use crate::components::primitives::{
        and_gate::AndGate,
        clock::Clock,
        constant::Const,
        input_pin::InputPin,
        nand_gate::NandGate,
        nor_gate::NorGate,
        not_gate::NotGate,
        or_gate::OrGate,
        output_pin::OutputPin,
        primitive::{Primitive, PrimitiveNotFound, PRIMITIVES},
        xor_gate::XorGate,
    };
}
