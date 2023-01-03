use crate::components::prelude::*;

pub trait CompVisitor<T> {
    fn visit_not_gate(&self, comp: &NotGate) -> T;
    fn visit_and_gate(&self, comp: &AndGate) -> T;
    fn visit_or_gate(&self, comp: &OrGate) -> T;
    fn visit_nand_gate(&self, comp: &NandGate) -> T;
    fn visit_nor_gate(&self, comp: &NorGate) -> T;
    fn visit_xor_gate(&self, comp: &XorGate) -> T;
    fn visit_clock(&self, comp: &Clock) -> T;
    fn visit_const(&self, comp: &Const) -> T;
    fn visit_composed(&self, comp: &ComposedComponent) -> T;
}

#[cfg(test)]
mod tests {
    use super::CompVisitor;
    use crate::components::prelude::*;

    struct TestCompVisitor;

    impl CompVisitor<Option<()>> for TestCompVisitor {
        fn visit_not_gate(&self, _: &NotGate) -> Option<()> {
            None
        }
        fn visit_and_gate(&self, _: &AndGate) -> Option<()> {
            None
        }
        fn visit_or_gate(&self, _: &OrGate) -> Option<()> {
            None
        }
        fn visit_nand_gate(&self, _: &NandGate) -> Option<()> {
            None
        }
        fn visit_nor_gate(&self, _: &NorGate) -> Option<()> {
            None
        }
        fn visit_xor_gate(&self, _: &XorGate) -> Option<()> {
            None
        }
        fn visit_clock(&self, _: &Clock) -> Option<()> {
            None
        }
        fn visit_const(&self, _: &Const) -> Option<()> {
            None
        }
        fn visit_composed(&self, _: &ComposedComponent) -> Option<()> {
            None
        }
    }

    #[test]
    fn component_visitor() {
        let test_visitor = TestCompVisitor {};

        let comp = ComposedComponentBuilder::new("Test").build().unwrap();
        assert!(test_visitor.visit_composed(&comp).is_none());

        for prim in PRIMITIVES {
            match prim {
                Primitive::NotGate => {
                    let comp = NotGate::new();
                    assert!(test_visitor.visit_not_gate(&comp).is_none());
                }
                Primitive::AndGate => {
                    let comp = AndGate::new(2);
                    assert!(test_visitor.visit_and_gate(&comp).is_none());
                }
                Primitive::OrGate => {
                    let comp = OrGate::new(2);
                    assert!(test_visitor.visit_or_gate(&comp).is_none());
                }
                Primitive::NandGate => {
                    let comp = NandGate::new(2);
                    assert!(test_visitor.visit_nand_gate(&comp).is_none());
                }
                Primitive::NorGate => {
                    let comp = NorGate::new(2);
                    assert!(test_visitor.visit_nor_gate(&comp).is_none());
                }
                Primitive::XorGate => {
                    let comp = XorGate::new(2);
                    assert!(test_visitor.visit_xor_gate(&comp).is_none());
                }
                Primitive::Clock => {
                    let comp = Clock::new(1.0);
                    assert!(test_visitor.visit_clock(&comp).is_none());
                }
                Primitive::ConstOne => {
                    let comp = Const::one();
                    assert!(test_visitor.visit_const(&comp).is_none());
                }
                Primitive::ConstZero => {
                    let comp = Const::zero();
                    assert!(test_visitor.visit_const(&comp).is_none());
                }
            }
        }
    }
}
