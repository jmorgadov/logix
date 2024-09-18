use std::ops::RangeInclusive;

use egui::Ui;
use logix_sim::primitives::primitive::Primitive;

use crate::app_ui::{
    board::{BoardComponent, ComponentInfo},
    board_editing::BoardEditing,
};

impl BoardEditing {
    fn comp_slider(ui: &mut Ui, text: &str, curr_val: usize, mut on_value: impl FnMut(usize)) {
        ui.add(
            egui::Slider::from_get_set(2.0..=256.0, |val| {
                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_sign_loss)]
                if let Some(v) = val {
                    on_value(v as usize);
                    return v;
                }
                #[allow(clippy::cast_precision_loss)]
                return curr_val as f64;
            })
            .logarithmic(true)
            .text(text),
        );
    }

    fn comp_slider_custom(
        ui: &mut Ui,
        range: RangeInclusive<f64>,
        log: bool,
        text: &str,
        curr_val: usize,
        mut on_value: impl FnMut(usize),
    ) {
        ui.add(
            egui::Slider::from_get_set(range, |val| {
                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_sign_loss)]
                if let Some(v) = val {
                    on_value(v as usize);
                    return v;
                }
                #[allow(clippy::cast_precision_loss)]
                return curr_val as f64;
            })
            .logarithmic(log)
            .text(text),
        );
    }

    #[allow(clippy::too_many_lines)]
    pub fn specific_comp_context_menu(&mut self, ui: &mut Ui, idx: usize) {
        let comp = self.board.components.get_mut(idx).unwrap();
        let mut new_comp = None;
        if let Some(prim) = comp.info.source.primitive_mut() {
            match prim {
                Primitive::NotGate | Primitive::Const { .. } | Primitive::Switch { .. } => {}
                Primitive::Custom { comp, state: _ } => match comp.name.as_str() {
                    "MUX" => {
                        Self::comp_slider_custom(
                            ui,
                            0.0..=8.0,
                            false,
                            "Size",
                            comp.inputs["SEL"],
                            |v| {
                                new_comp = Some(BoardComponent::from_comp_info(
                                    ComponentInfo::multiplexer(comp.inputs["0"], v),
                                ));
                            },
                        );
                        Self::comp_slider_custom(
                            ui,
                            1.0..=256.0,
                            true,
                            "Inputs size",
                            comp.inputs["0"],
                            |v| {
                                new_comp = Some(BoardComponent::from_comp_info(
                                    ComponentInfo::multiplexer(v, comp.inputs["SEL"]),
                                ));
                            },
                        );
                    }
                    "DMX" => {
                        todo!()
                    }
                    _ => {}
                },
                Primitive::AndGate => {
                    Self::comp_slider(ui, "Inputs", comp.info.inputs.len(), |v| {
                        comp.update_comp_info(ComponentInfo::and_gate(v));
                    });
                }
                Primitive::OrGate => {
                    Self::comp_slider(ui, "Inputs", comp.info.inputs.len(), |v| {
                        comp.update_comp_info(ComponentInfo::or_gate(v));
                    });
                }
                Primitive::NandGate => {
                    Self::comp_slider(ui, "Inputs", comp.info.inputs.len(), |v| {
                        comp.update_comp_info(ComponentInfo::nand_gate(v));
                    });
                }
                Primitive::NorGate => {
                    Self::comp_slider(ui, "Inputs", comp.info.inputs.len(), |v| {
                        comp.update_comp_info(ComponentInfo::nor_gate(v));
                    });
                }
                Primitive::XorGate => {
                    Self::comp_slider(ui, "Inputs", comp.info.inputs.len(), |v| {
                        comp.update_comp_info(ComponentInfo::xor_gate(v));
                    });
                }
                Primitive::Splitter { .. } => {
                    Self::comp_slider(ui, "Bits", comp.info.outputs.len(), |v| {
                        comp.update_comp_info(ComponentInfo::splitter(v));
                    });
                }
                Primitive::Joiner { .. } => {
                    Self::comp_slider(ui, "Bits", comp.info.inputs.len(), |v| {
                        comp.update_comp_info(ComponentInfo::joiner(v));
                    });
                }
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
                    Self::comp_slider_custom(
                        ui,
                        1.0..=256.0,
                        true,
                        "Bits",
                        comp.outputs_data[0].size,
                        |v| {
                            comp.update_comp_info(ComponentInfo::input(v));
                        },
                    );
                    ui.label(format!("Input order: {in_idx}"));
                    if ui.button("Move up").clicked() && in_idx > 0 {
                        self.board.inputs.swap(in_idx, in_idx - 1);
                    }
                    if ui.button("Move down").clicked() && in_idx < self.board.inputs.len() - 1 {
                        self.board.inputs.swap(in_idx, in_idx + 1);
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
                    Self::comp_slider_custom(
                        ui,
                        1.0..=256.0,
                        true,
                        "Bits",
                        comp.inputs_data[0].size,
                        |v| {
                            comp.update_comp_info(ComponentInfo::output(v));
                        },
                    );
                    ui.label(format!("Output order: {out_idx}"));
                    if ui.button("Move up").clicked() && out_idx > 0 {
                        self.board.outputs.swap(out_idx, out_idx - 1);
                    }
                    if ui.button("Move down").clicked() && out_idx < self.board.outputs.len() - 1 {
                        self.board.outputs.swap(out_idx, out_idx + 1);
                    }
                }
            }
        }
        if let Some(mut new_comp) = new_comp {
            new_comp.pos = comp.pos;
            new_comp.id = comp.id;
            self.board.components[idx] = new_comp;
        }
    }
}
