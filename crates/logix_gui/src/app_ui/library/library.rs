use asmhdl::AsmComponent;
use indexmap::IndexMap;

use crate::app_ui::board::ComponentInfo;

use super::{flip_flops::flip_flops_lib, gates::gates_lib};

#[derive(Default)]
pub struct Library {
    pub components: IndexMap<String, ComponentInfo>,
    pub sub_libs: IndexMap<String, Library>,
}

impl Library {
    pub fn new(
        components: IndexMap<String, ComponentInfo>,
        sub_libs: impl Into<IndexMap<String, Self>>,
    ) -> Self {
        Self {
            components,
            sub_libs: sub_libs.into(),
        }
    }

    pub fn load() -> Self {
        Self::new(
            [
                ("CLK".into(), ComponentInfo::clock_gate()),
                ("Switch".into(), ComponentInfo::switch()),
            ]
            .into(),
            [
                ("Gates".into(), gates_lib()),
                ("Memory".into(), flip_flops_lib()),
            ],
        )
    }

    pub fn entry_from_code(code: &str) -> (String, ComponentInfo) {
        let comp = AsmComponent::from_code(code);
        let entry_name = match &comp.description {
            Some(desc) => desc.clone(),
            None => comp.name.clone(),
        };
        let info = ComponentInfo::custom(comp);
        (entry_name, info)
    }
}
