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
                        .map(|bit| (format!("IN {bit}"), ComponentInfo::input(*bit)))
                        .collect(),
                    [],
                ),
            ),
            (
                "Outputs".into(),
                Library::new(
                    BIT_RANGE
                        .iter()
                        .map(|bit| (format!("OUT {bit}"), ComponentInfo::output(*bit)))
                        .collect(),
                    [],
                ),
            ),
            (
                "AND".into(),
                Library::new(
                    BIT_RANGE
                        .iter()
                        .map(|bit| (format!("AND {bit}"), ComponentInfo::and_gate(*bit)))
                        .collect(),
                    [],
                ),
            ),
            (
                "OR".into(),
                Library::new(
                    BIT_RANGE
                        .iter()
                        .map(|bit| (format!("OR {bit}"), ComponentInfo::or_gate(*bit)))
                        .collect(),
                    [],
                ),
            ),
            (
                "NAND".into(),
                Library::new(
                    BIT_RANGE
                        .iter()
                        .map(|bit| (format!("NAND {bit}"), ComponentInfo::nand_gate(*bit)))
                        .collect(),
                    [],
                ),
            ),
            (
                "NOR".into(),
                Library::new(
                    BIT_RANGE
                        .iter()
                        .map(|bit| (format!("NOR {bit}"), ComponentInfo::nor_gate(*bit)))
                        .collect(),
                    [],
                ),
            ),
            (
                "XOR".into(),
                Library::new(
                    BIT_RANGE
                        .iter()
                        .map(|bit| (format!("XOR {bit}"), ComponentInfo::xor_gate(*bit)))
                        .collect(),
                    [],
                ),
            ),
            (
                "Joiner".into(),
                Library::new(
                    BIT_RANGE
                        .iter()
                        .map(|bit| (format!("Joiner {bit}"), ComponentInfo::joiner(*bit)))
                        .collect(),
                    [],
                ),
            ),
            (
                "Splitter".into(),
                Library::new(
                    BIT_RANGE
                        .iter()
                        .map(|bit| (format!("Splitter {bit}"), ComponentInfo::splitter(*bit)))
                        .collect(),
                    [],
                ),
            ),
        ],
    )
}
