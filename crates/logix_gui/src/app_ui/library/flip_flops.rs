use crate::app_ui::board::ComponentInfo;

use super::Library;

pub fn flip_flops_lib() -> Library {
    Library::new(
        [(
            "JK Master-Slave (Falling edge)".into(),
            ComponentInfo::from_asm_comp_code(include_str!(
                "./asmhdl_components/jkff_ms_fe.asmhdl"
            )),
        )]
        .into(),
        [],
    )
}
