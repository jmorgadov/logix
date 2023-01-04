use crate::components::prelude::*;

pub trait CompVisitor<T> {
    fn visit_not_gate(comp: &NotGate) -> T;
    fn visit_and_gate(comp: &AndGate) -> T;
    fn visit_or_gate(comp: &OrGate) -> T;
    fn visit_nand_gate(comp: &NandGate) -> T;
    fn visit_nor_gate(comp: &NorGate) -> T;
    fn visit_xor_gate(comp: &XorGate) -> T;
    fn visit_clock(comp: &Clock) -> T;
    fn visit_const(comp: &Const) -> T;
    fn visit_composed(comp: &ComposedComponent) -> T;
}

#[cfg(test)]
mod tests {
    use super::CompVisitor;
    use crate::components::prelude::*;

    struct TestCompVisitor;
    impl CompVisitor<Option<()>> for TestCompVisitor {
        fn visit_not_gate(_: &NotGate) -> Option<()> {
            None
        }
        fn visit_and_gate(_: &AndGate) -> Option<()> {
            None
        }
        fn visit_or_gate(_: &OrGate) -> Option<()> {
            None
        }
        fn visit_nand_gate(_: &NandGate) -> Option<()> {
            None
        }
        fn visit_nor_gate(_: &NorGate) -> Option<()> {
            None
        }
        fn visit_xor_gate(_: &XorGate) -> Option<()> {
            None
        }
        fn visit_clock(_: &Clock) -> Option<()> {
            None
        }
        fn visit_const(_: &Const) -> Option<()> {
            None
        }
        fn visit_composed(_: &ComposedComponent) -> Option<()> {
            None
        }
    }

    #[test]
    fn component_visitor() {
        let comp = ComposedComponentBuilder::new("Test").build().unwrap();
        assert!(TestCompVisitor::visit_composed(&comp).is_none());

        for prim in PRIMITIVES {
            match prim {
                Primitive::NotGate => {
                    let comp = NotGate::new();
                    assert!(TestCompVisitor::visit_not_gate(&comp).is_none());
                }
                Primitive::AndGate => {
                    let comp = AndGate::new(2);
                    assert!(TestCompVisitor::visit_and_gate(&comp).is_none());
                }
                Primitive::OrGate => {
                    let comp = OrGate::new(2);
                    assert!(TestCompVisitor::visit_or_gate(&comp).is_none());
                }
                Primitive::NandGate => {
                    let comp = NandGate::new(2);
                    assert!(TestCompVisitor::visit_nand_gate(&comp).is_none());
                }
                Primitive::NorGate => {
                    let comp = NorGate::new(2);
                    assert!(TestCompVisitor::visit_nor_gate(&comp).is_none());
                }
                Primitive::XorGate => {
                    let comp = XorGate::new(2);
                    assert!(TestCompVisitor::visit_xor_gate(&comp).is_none());
                }
                Primitive::Clock => {
                    let comp = Clock::new(1.0);
                    assert!(TestCompVisitor::visit_clock(&comp).is_none());
                }
                Primitive::ConstOne => {
                    let comp = Const::one();
                    assert!(TestCompVisitor::visit_const(&comp).is_none());
                }
                Primitive::ConstZero => {
                    let comp = Const::zero();
                    assert!(TestCompVisitor::visit_const(&comp).is_none());
                }
            }
        }
    }
}
