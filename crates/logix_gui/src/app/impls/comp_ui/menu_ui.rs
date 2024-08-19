use egui::Ui;
use logix_sim::primitives::primitives::Primitive;

use crate::app::LogixApp;

impl LogixApp {
    pub fn specific_comp_context_menu(&mut self, ui: &mut Ui, idx: usize) {
        let comp = self.board.components.get_mut(idx).unwrap();
        if let Some(prim) = &mut comp.primitive {
            match prim {
                Primitive::Clock { period: current_p } => {
                    ui.add(
                        egui::Slider::from_get_set(1.0..=10000.0, |val| {
                            if let Some(v) = val {
                                let val_to_ns = v * 1_000_000.0;
                                *current_p = val_to_ns as u128;
                                return v;
                            }
                            *current_p as f64 / 1_000_000.0
                        })
                        .text("Frec (ms)"),
                    );
                }
                Primitive::AndGate => {}
                Primitive::OrGate => {}
                Primitive::NotGate => {}
                Primitive::NandGate => {}
                Primitive::NorGate => {}
                Primitive::XorGate => {}
                Primitive::Input { bits: _ } => {
                    let in_idx = self
                        .board
                        .inputs_idx
                        .iter()
                        .position(|&x| x == idx)
                        .unwrap();
                    ui.add(
                        egui::TextEdit::singleline(&mut self.board.inputs_name[in_idx])
                            .hint_text("Name"),
                    );
                }
                Primitive::Output { bits: _ } => {
                    let out_idx = self
                        .board
                        .outputs_idx
                        .iter()
                        .position(|&x| x == idx)
                        .unwrap();
                    ui.add(
                        egui::TextEdit::singleline(&mut self.board.outputs_name[out_idx])
                            .hint_text("Name"),
                    );
                }
                Primitive::Splitter { bits: _ } => {}
                Primitive::Joiner { bits: _ } => {}
                Primitive::Const { value: _ } => {}
            }
        }
    }
}
