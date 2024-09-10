use super::Library;

pub fn flip_flops_lib() -> Library {
    Library::new(
        [Library::entry_from_code(include_str!(
            "./asmhdl_components/jkff_ms_fe.asmhdl"
        ))]
        .into(),
        [],
    )
}
