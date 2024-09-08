use crate::app_ui::board::ComponentInfo;

use super::{library::BIT_RANGE, Library};

#[allow(clippy::too_many_lines)]
pub fn gates_lib() -> Library {
    Library::new(
        [
            ("HIGH".into(), ComponentInfo::const_high_gate()),
            ("LOW".into(), ComponentInfo::const_low_gate()),
            ("NOT".into(), ComponentInfo::not_gate()),
        ]
        .into(),
        [
            (
                "Inputs".into(),
                Library::new(
                    BIT_RANGE
                        .iter()
                        .map(|bit| (format!("{bit} bits"), ComponentInfo::input(*bit)))
                        .collect(),
                    [],
                ),
            ),
            (
                "Outputs".into(),
                Library::new(
                    BIT_RANGE
                        .iter()
                        .map(|bit| (format!("{bit} bits"), ComponentInfo::output(*bit)))
                        .collect(),
                    [],
                ),
            ),
            (
                "AND".into(),
                Library::new(
                    BIT_RANGE
                        .iter()
                        .skip(1)
                        .map(|bit| (format!("{bit} bits"), ComponentInfo::and_gate(*bit)))
                        .collect(),
                    [],
                ),
            ),
            (
                "OR".into(),
                Library::new(
                    BIT_RANGE
                        .iter()
                        .skip(1)
                        .map(|bit| (format!("{bit} bits"), ComponentInfo::or_gate(*bit)))
                        .collect(),
                    [],
                ),
            ),
            (
                "NAND".into(),
                Library::new(
                    BIT_RANGE
                        .iter()
                        .skip(1)
                        .map(|bit| (format!("{bit} bits"), ComponentInfo::nand_gate(*bit)))
                        .collect(),
                    [],
                ),
            ),
            (
                "NOR".into(),
                Library::new(
                    BIT_RANGE
                        .iter()
                        .skip(1)
                        .map(|bit| (format!("{bit} bits"), ComponentInfo::nor_gate(*bit)))
                        .collect(),
                    [],
                ),
            ),
            (
                "XOR".into(),
                Library::new(
                    BIT_RANGE
                        .iter()
                        .skip(1)
                        .map(|bit| (format!("{bit} bits"), ComponentInfo::xor_gate(*bit)))
                        .collect(),
                    [],
                ),
            ),
            (
                "Joiner".into(),
                Library::new(
                    BIT_RANGE
                        .iter()
                        .skip(1)
                        .map(|bit| (format!("{bit} bits"), ComponentInfo::joiner(*bit)))
                        .collect(),
                    [],
                ),
            ),
            (
                "Splitter".into(),
                Library::new(
                    BIT_RANGE
                        .iter()
                        .skip(1)
                        .map(|bit| (format!("{bit} bits"), ComponentInfo::splitter(*bit)))
                        .collect(),
                    [],
                ),
            ),
        ],
    )
}
