use crate::app_ui::board::ComponentInfo;

use super::Library;

#[allow(clippy::too_many_lines)]
pub fn plexers_lib() -> Library {
    Library::new(
        [
            ("MUX".into(), ComponentInfo::multiplexer(1, 2)),
            //("DMX".into(), ComponentInfo::const_low_gate()),
        ]
        .into(),
        [],
    )
}
