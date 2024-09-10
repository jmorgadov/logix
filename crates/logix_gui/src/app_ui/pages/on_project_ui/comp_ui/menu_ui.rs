use egui::Ui;
use logix_sim::primitives::primitive::Primitive;

use crate::app_ui::board_editing::BoardEditing;

impl BoardEditing {
    pub fn specific_comp_context_menu(&mut self, ui: &mut Ui, idx: usize) {
        let comp = self.board.components.get_mut(idx).unwrap();
        if let Some(prim) = comp.info.source.primitive_mut() {
            match prim {
                Primitive::AndGate
                | Primitive::OrGate
                | Primitive::NotGate
                | Primitive::NandGate
                | Primitive::NorGate
                | Primitive::XorGate
                | Primitive::Custom { .. }
                | Primitive::Splitter { .. }
                | Primitive::Joiner { .. }
                | Primitive::Const { .. }
                | Primitive::Switch { .. } => {}
                Primitive::Clock { period: current_p } => {
                    ui.add(
                        egui::Slider::from_get_set(1e-6..=1e9, |val| {
                            #[allow(clippy::cast_possible_truncation)]
                            #[allow(clippy::cast_sign_loss)]
                            if let Some(v) = val {
                                let val_to_ns = 1_000_000_000.0 / v;
                                *current_p = val_to_ns as u128;
                                return v;
                            }
                            #[allow(clippy::cast_precision_loss)]
                            return 1_000_000_000.0 / *current_p as f64;
                        })
                        .logarithmic(true)
                        .text("Frec (Hz)"),
                    );
                }

                Primitive::Input { bits: _ } => {
                    let in_idx = self.board.inputs.iter().position(|x| x.idx == idx).unwrap();
                    let resp = ui.add(
                        egui::TextEdit::singleline(&mut self.board.inputs[in_idx].name)
                            .hint_text("Name"),
                    );
                    if resp.lost_focus() {
                        ui.close_menu();
                    }
                }
                Primitive::Output { bits: _ } => {
                    let out_idx = self
                        .board
                        .outputs
                        .iter()
                        .position(|output| output.idx == idx)
                        .unwrap();
                    let resp = ui.add(
                        egui::TextEdit::singleline(&mut self.board.outputs[out_idx].name)
                            .hint_text("Name"),
                    );
                    if resp.lost_focus() {
                        ui.close_menu();
                    }
                }
            }
        }
    }
}
