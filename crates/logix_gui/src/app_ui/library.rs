use indexmap::IndexMap;

use super::board::BoardComponent;

#[derive(Default)]
pub struct Library {
    pub name: String,
    pub components: IndexMap<String, BoardComponent>,
    pub sub_libs: Vec<Library>,
}

pub const BIT_RANGE: [u8; 10] = [2, 3, 4, 5, 6, 7, 8, 16, 32, 64];

impl Library {
    fn new(
        name: String,
        components: IndexMap<String, BoardComponent>,
        sub_libs: Vec<Self>,
    ) -> Self {
        Self {
            name,
            components,
            sub_libs,
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn load() -> Self {
        Self::new(
            String::new(),
            [
                ("CLK".into(), BoardComponent::clock_gate()),
                ("NOT".into(), BoardComponent::not_gate()),
                ("HIGH".into(), BoardComponent::const_high_gate()),
                ("LOW".into(), BoardComponent::const_low_gate()),
            ]
            .iter()
            .cloned()
            .collect(),
            vec![
                Self::new(
                    "Inputs".into(),
                    BIT_RANGE
                        .iter()
                        .map(|bit| (format!("IN {bit}"), BoardComponent::input(*bit)))
                        .collect(),
                    vec![],
                ),
                Self::new(
                    "Outputs".into(),
                    BIT_RANGE
                        .iter()
                        .map(|bit| (format!("OUT {bit}"), BoardComponent::output(*bit)))
                        .collect(),
                    vec![],
                ),
                Self::new(
                    "AND".into(),
                    BIT_RANGE
                        .iter()
                        .map(|bit| {
                            (
                                format!("AND {bit}"),
                                BoardComponent::and_gate(*bit as usize),
                            )
                        })
                        .collect(),
                    vec![],
                ),
                Self::new(
                    "OR".into(),
                    BIT_RANGE
                        .iter()
                        .map(|bit| (format!("OR {bit}"), BoardComponent::or_gate(*bit as usize)))
                        .collect(),
                    vec![],
                ),
                Self::new(
                    "NAND".into(),
                    BIT_RANGE
                        .iter()
                        .map(|bit| {
                            (
                                format!("NAND {bit}"),
                                BoardComponent::nand_gate(*bit as usize),
                            )
                        })
                        .collect(),
                    vec![],
                ),
                Self::new(
                    "NOR".into(),
                    BIT_RANGE
                        .iter()
                        .map(|bit| {
                            (
                                format!("NOR {bit}"),
                                BoardComponent::nor_gate(*bit as usize),
                            )
                        })
                        .collect(),
                    vec![],
                ),
                Self::new(
                    "XOR".into(),
                    BIT_RANGE
                        .iter()
                        .map(|bit| {
                            (
                                format!("XOR {bit}"),
                                BoardComponent::xor_gate(*bit as usize),
                            )
                        })
                        .collect(),
                    vec![],
                ),
                Self::new(
                    "Joiner".into(),
                    BIT_RANGE
                        .iter()
                        .map(|bit| (format!("Joiner {bit}"), BoardComponent::joiner(*bit)))
                        .collect(),
                    vec![],
                ),
                Self::new(
                    "Splitter".into(),
                    BIT_RANGE
                        .iter()
                        .map(|bit| (format!("Splitter {bit}"), BoardComponent::splitter(*bit)))
                        .collect(),
                    vec![],
                ),
            ],
        )
    }
}
