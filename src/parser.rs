use crate::components::prelude::*;

pub type ParseResult<T> = Result<T, ()>;

pub trait CompParser<T> {
    fn parse_not_gate(&self, obj: T) -> ParseResult<NotGate>;
    fn parse_and_gate(&self, obj: T) -> ParseResult<AndGate>;
    fn parse_or_gate(&self, obj: T) -> ParseResult<OrGate>;
    fn parse_nand_gate(&self, obj: T) -> ParseResult<NandGate>;
    fn parse_nor_gate(&self, obj: T) -> ParseResult<NorGate>;
    fn parse_xor_gate(&self, obj: T) -> ParseResult<XorGate>;
    fn parse_clock(&self, obj: T) -> ParseResult<Clock>;
    fn parse_input_pin(&self, obj: T) -> ParseResult<InputPin>;
    fn parse_output_pin(&self, obj: T) -> ParseResult<OutputPin>;
    fn parse_const(&self, obj: T) -> ParseResult<Const>;
    fn parse_composed(&self, obj: T) -> ParseResult<ComposedComponent>;
}

#[cfg(test)]
mod tests {
    use super::{CompParser, ParseResult};
    use crate::components::prelude::*;

    struct TestCompParser;

    impl CompParser<()> for TestCompParser {
        fn parse_not_gate(&self, _: ()) -> ParseResult<NotGate> {
            Err(())
        }
        fn parse_and_gate(&self, _: ()) -> ParseResult<AndGate> {
            Err(())
        }
        fn parse_or_gate(&self, _: ()) -> ParseResult<OrGate> {
            Err(())
        }
        fn parse_nand_gate(&self, _: ()) -> ParseResult<NandGate> {
            Err(())
        }
        fn parse_nor_gate(&self, _: ()) -> ParseResult<NorGate> {
            Err(())
        }
        fn parse_xor_gate(&self, _: ()) -> ParseResult<XorGate> {
            Err(())
        }
        fn parse_clock(&self, _: ()) -> ParseResult<Clock> {
            Err(())
        }
        fn parse_input_pin(&self, _: ()) -> ParseResult<InputPin> {
            Err(())
        }
        fn parse_output_pin(&self, _: ()) -> ParseResult<OutputPin> {
            Err(())
        }
        fn parse_const(&self, _: ()) -> ParseResult<Const> {
            Err(())
        }
        fn parse_composed(&self, _: ()) -> ParseResult<ComposedComponent> {
            Err(())
        }
    }

    #[test]
    fn component_parser() {
        let test_parser = TestCompParser {};

        assert!(test_parser.parse_composed(()).is_err());

        for prim in PRIMITIVES {
            match prim {
                Primitive::NotGate => {
                    assert!(test_parser.parse_not_gate(()).is_err());
                }
                Primitive::AndGate => {
                    assert!(test_parser.parse_and_gate(()).is_err());
                }
                Primitive::OrGate => {
                    assert!(test_parser.parse_or_gate(()).is_err());
                }
                Primitive::NandGate => {
                    assert!(test_parser.parse_nand_gate(()).is_err());
                }
                Primitive::NorGate => {
                    assert!(test_parser.parse_nor_gate(()).is_err());
                }
                Primitive::XorGate => {
                    assert!(test_parser.parse_xor_gate(()).is_err());
                }
                Primitive::Clock => {
                    assert!(test_parser.parse_clock(()).is_err());
                }
                Primitive::InputPin => {
                    assert!(test_parser.parse_input_pin(()).is_err());
                }
                Primitive::OutputPin => {
                    assert!(test_parser.parse_output_pin(()).is_err());
                }
                Primitive::ConstOne => {
                    assert!(test_parser.parse_const(()).is_err());
                }
                Primitive::ConstZero => {
                    assert!(test_parser.parse_const(()).is_err());
                }
            }
        }
    }
}
