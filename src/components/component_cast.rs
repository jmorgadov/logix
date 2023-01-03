use super::{primitives::prelude::*, composed_component::ComposedComponent};

pub trait ComponentCast {
    fn as_not_gate(&self) -> Option<&NotGate> {
        None
    }
    fn as_and_gate(&self) -> Option<&AndGate> {
        None
    }
    fn as_or_gate(&self) -> Option<&OrGate> {
        None
    }
    fn as_nand_gate(&self) -> Option<&NandGate> {
        None
    }
    fn as_nor_gate(&self) -> Option<&NorGate> {
        None
    }
    fn as_xor_gate(&self) -> Option<&XorGate> {
        None
    }
    fn as_clock(&self) -> Option<&Clock> {
        None
    }
    fn as_input_pin(&self) -> Option<&InputPin> {
        None
    }
    fn as_output_pin(&self) -> Option<&OutputPin> {
        None
    }
    fn as_const(&self) -> Option<&Const> {
        None
    }
    fn as_composed(&self) -> Option<&ComposedComponent> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{primitives::prelude::*, composed_component::ComposedComponentBuilder};

    use super::ComponentCast;

    #[test]
    fn cast_composed_component() {
        let comp = ComposedComponentBuilder::new("Test").build();
        assert!(comp.as_composed().is_some())
    }

    #[test]
    fn cast_primitives() {
        // This is just to make sure there is a cast function for
        // every primitive.
        for prim in PRIMITIVES {
            match prim {
                Primitive::NotGate => {
                    let comp = NotGate::new();
                    assert!(comp.as_not_gate().is_some());
                }
                Primitive::AndGate => {
                    let comp = AndGate::new(2);
                    assert!(comp.as_and_gate().is_some());
                }
                Primitive::OrGate => {
                    let comp = OrGate::new(2);
                    assert!(comp.as_or_gate().is_some());
                }
                Primitive::NandGate => {
                    let comp = NandGate::new(2);
                    assert!(comp.as_nand_gate().is_some());
                }
                Primitive::NorGate => {
                    let comp = NorGate::new(2);
                    assert!(comp.as_nor_gate().is_some());
                }
                Primitive::XorGate => {
                    let comp = XorGate::new(2);
                    assert!(comp.as_xor_gate().is_some());
                }
                Primitive::Clock => {
                    let comp = Clock::new(1.0);
                    assert!(comp.as_clock().is_some());
                }
                Primitive::InputPin => {
                    let comp = InputPin::new(0);
                    assert!(comp.as_input_pin().is_some());
                }
                Primitive::OutputPin => {
                    let comp = OutputPin::new(0);
                    assert!(comp.as_output_pin().is_some());
                }
                Primitive::ConstOne => {
                    let comp = Const::one();
                    assert!(comp.as_const().is_some());
                }
                Primitive::ConstZero => {
                    let comp = Const::zero();
                    assert!(comp.as_const().is_some());
                }
            }
        }
    }
}
