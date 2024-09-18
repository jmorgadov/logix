use crate::app_ui::board::ComponentInfo;

use super::Library;

#[allow(clippy::too_many_lines)]
pub fn gates_lib() -> Library {
    Library::new(
        [
            ("HIGH".into(), ComponentInfo::const_high_gate()),
            ("LOW".into(), ComponentInfo::const_low_gate()),
            ("NOT".into(), ComponentInfo::not_gate()),
            ("IN".into(), ComponentInfo::input(1)),
            ("OUT".into(), ComponentInfo::output(1)),
            ("AND".into(), ComponentInfo::and_gate(2)),
            ("OR".into(), ComponentInfo::or_gate(2)),
            ("NAND".into(), ComponentInfo::nand_gate(2)),
            ("NOR".into(), ComponentInfo::nor_gate(2)),
            ("XOR".into(), ComponentInfo::xor_gate(2)),
            ("JOIN".into(), ComponentInfo::joiner(8)),
            ("SPLIT".into(), ComponentInfo::splitter(8)),
        ]
        .into(),
        [],
    )
}
