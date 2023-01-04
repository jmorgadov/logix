use crate::components::prelude::*;

#[derive(Default, Debug)]
pub struct ParseError;

pub type ParseResult<T> = Result<T, ParseError>;

pub trait CompParser<T> {
    fn parse_not_gate(obj: T) -> ParseResult<NotGate>;
    fn parse_and_gate(obj: T) -> ParseResult<AndGate>;
    fn parse_or_gate(obj: T) -> ParseResult<OrGate>;
    fn parse_nand_gate(obj: T) -> ParseResult<NandGate>;
    fn parse_nor_gate(obj: T) -> ParseResult<NorGate>;
    fn parse_xor_gate(obj: T) -> ParseResult<XorGate>;
    fn parse_clock(obj: T) -> ParseResult<Clock>;
    fn parse_const(obj: T) -> ParseResult<Const>;
    fn parse_composed(obj: T) -> ParseResult<ComposedComponent>;
}

#[cfg(test)]
mod tests {
    use super::{CompParser, ParseResult};
    use crate::components::prelude::*;

    struct TestCompParser;

    impl CompParser<()> for TestCompParser {
        fn parse_not_gate(_: ()) -> ParseResult<NotGate> {
            Err(Default::default())
        }
        fn parse_and_gate(_: ()) -> ParseResult<AndGate> {
            Err(Default::default())
        }
        fn parse_or_gate(_: ()) -> ParseResult<OrGate> {
            Err(Default::default())
        }
        fn parse_nand_gate(_: ()) -> ParseResult<NandGate> {
            Err(Default::default())
        }
        fn parse_nor_gate(_: ()) -> ParseResult<NorGate> {
            Err(Default::default())
        }
        fn parse_xor_gate(_: ()) -> ParseResult<XorGate> {
            Err(Default::default())
        }
        fn parse_clock(_: ()) -> ParseResult<Clock> {
            Err(Default::default())
        }
        fn parse_const(_: ()) -> ParseResult<Const> {
            Err(Default::default())
        }
        fn parse_composed(_: ()) -> ParseResult<ComposedComponent> {
            Err(Default::default())
        }
    }

    #[test]
    fn component_parser() {
        assert!(TestCompParser::parse_composed(()).is_err());

        for prim in PRIMITIVES {
            match prim {
                Primitive::NotGate => {
                    assert!(TestCompParser::parse_not_gate(()).is_err());
                }
                Primitive::AndGate => {
                    assert!(TestCompParser::parse_and_gate(()).is_err());
                }
                Primitive::OrGate => {
                    assert!(TestCompParser::parse_or_gate(()).is_err());
                }
                Primitive::NandGate => {
                    assert!(TestCompParser::parse_nand_gate(()).is_err());
                }
                Primitive::NorGate => {
                    assert!(TestCompParser::parse_nor_gate(()).is_err());
                }
                Primitive::XorGate => {
                    assert!(TestCompParser::parse_xor_gate(()).is_err());
                }
                Primitive::Clock => {
                    assert!(TestCompParser::parse_clock(()).is_err());
                }
                Primitive::ConstOne => {
                    assert!(TestCompParser::parse_const(()).is_err());
                }
                Primitive::ConstZero => {
                    assert!(TestCompParser::parse_const(()).is_err());
                }
            }
        }
    }
}
