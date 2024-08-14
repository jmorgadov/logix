use egui::Pos2;
use logix_core::component::Component;
use logix_sim::primitives::primitives::ExtraInfo;

pub struct ComponentBoard {
    pub component: Component<ExtraInfo>,
    pub subc_pos: Vec<Pos2>,
}

impl ComponentBoard {
    pub fn new(component: Component<ExtraInfo>, subc_pos: Vec<Pos2>) -> Self {
        // assert!(
        //     subc_designs.len() == subc_pos.len(),
        //     "subc_designs and subc_pos must have the same length"
        // );
        Self {
            component,
            subc_pos,
        }
    }
}
