use egui::{emath::TSTransform, epaint::PathShape, Color32, Id, Rect, Shape, Stroke, Ui};
use rfd::FileDialog;

use crate::app::{board_editing::BoardEditing, impls::wire_dir::WireDir};

impl BoardEditing {
    pub fn draw_canvas(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .inner_margin(egui::Margin::same(0.0))
                    .fill(Color32::from_rgb(35, 35, 35)),
            )
            .show(ctx, |ui| {
                let (id, rect) = ui.allocate_space(ui.available_size());

                let response = ui.interact(rect, id, egui::Sense::click_and_drag());

                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    self.new_conn = None;
                }

                // Allow dragging the background as well.
                if response.dragged() {
                    self.transform.translation += response.drag_delta();
                }

                // Plot-like reset
                if response.double_clicked() {
                    self.transform = TSTransform::default();
                }

                let transform = TSTransform::from_translation(ui.min_rect().left_top().to_vec2())
                    * self.transform;

                if let Some(pointer) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                    let pointer_in_layer = transform.inverse() * pointer;
                    let zoom_delta = ui.ctx().input(|i| i.zoom_delta());

                    // Zoom in on pointer:
                    self.transform = self.transform
                        * TSTransform::from_translation(pointer_in_layer.to_vec2())
                        * TSTransform::from_scaling(zoom_delta)
                        * TSTransform::from_translation(-pointer_in_layer.to_vec2());

                    if response.hovered() {
                        // Only pan if the mouse is over the background.
                        let pan_delta = ui.ctx().input(|i| i.smooth_scroll_delta);
                        self.transform = TSTransform::from_translation(pan_delta) * self.transform;
                    }
                }

                if response.hovered() && response.clicked_by(egui::PointerButton::Secondary) {
                    self.last_click_pos =
                        transform.inverse() * response.interact_pointer_pos().unwrap();
                }

                if self.new_conn.is_some() {
                    let new_conn = self.new_conn.as_mut().unwrap();
                    if response.hovered() && response.clicked_by(egui::PointerButton::Primary) {
                        let last_point = *new_conn.1.last().unwrap();
                        let cursor_pos =
                            transform.inverse() * response.interact_pointer_pos().unwrap();
                        let new_point = Self::get_ghost_point(
                            last_point,
                            WireDir::get_dir(new_conn.1.len()),
                            cursor_pos,
                        );
                        new_conn.1.push(new_point);
                    }
                    ui.painter().add(Shape::Path(PathShape::line(
                        new_conn.1.iter().map(|p| transform * *p).collect(),
                        Stroke::new(2.0, Color32::WHITE),
                    )));
                }

                if response.hovered() || response.context_menu_opened() {
                    response.context_menu(|ui| {
                        ui.set_max_width(150.0);
                        ui.label("Board");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.board.name)
                                .hint_text("Board name"),
                        );
                        ui.label("Add Component");
                        if self.file.is_some() && ui.button("Import Component").clicked() {
                            let comp_file = FileDialog::new()
                                .set_directory(self.file.as_ref().unwrap().clone())
                                .pick_file();
                            if let Some(comp_file) = comp_file {
                                if let Ok(comp_file) =
                                    comp_file.strip_prefix(self.file.as_ref().unwrap().clone())
                                {
                                    if self
                                        .board
                                        .import_comp(
                                            self.next_id,
                                            comp_file.to_path_buf(),
                                            self.last_click_pos,
                                        )
                                        .is_ok()
                                    {
                                        self.next_id += 1;
                                    }
                                }
                            }
                            ui.close_menu();
                        }
                        if ui.button("Clock").clicked() {
                            self.board.add_clock_gate(self.next_id, self.last_click_pos);
                            self.next_id += 1;
                            ui.close_menu();
                        }
                        if ui.button("High Const").clicked() {
                            self.board
                                .add_const_high_gate(self.next_id, self.last_click_pos);
                            self.next_id += 1;
                            ui.close_menu();
                        }
                        if ui.button("Low Const").clicked() {
                            self.board
                                .add_const_low_gate(self.next_id, self.last_click_pos);
                            self.next_id += 1;
                            ui.close_menu();
                        }

                        if ui.button("Not").clicked() {
                            self.board.add_not_gate(self.next_id, self.last_click_pos);
                            self.next_id += 1;
                            ui.close_menu();
                        }

                        ui.menu_button("Input", |ui| {
                            for i in 1..=8 {
                                if ui.button(format!("{} Bits", i)).clicked() {
                                    self.board.add_input(self.next_id, i, self.last_click_pos);
                                    self.next_id += 1;
                                    ui.close_menu();
                                }
                            }
                        });

                        ui.menu_button("Output", |ui| {
                            for i in 1..=8 {
                                if ui.button(format!("{} Bits", i)).clicked() {
                                    self.board.add_output(self.next_id, i, self.last_click_pos);
                                    self.next_id += 1;
                                    ui.close_menu();
                                }
                            }
                        });

                        ui.menu_button("And Gate", |ui| {
                            for i in 2..=8 {
                                if ui.button(format!("{} Inputs", i)).clicked() {
                                    self.board
                                        .add_and_gate(self.next_id, i, self.last_click_pos);
                                    self.next_id += 1;
                                    ui.close_menu();
                                }
                            }
                        });
                        ui.menu_button("Nand Gate", |ui| {
                            for i in 2..=8 {
                                if ui.button(format!("{} Inputs", i)).clicked() {
                                    self.board
                                        .add_nand_gate(self.next_id, i, self.last_click_pos);
                                    self.next_id += 1;
                                    ui.close_menu();
                                }
                            }
                        });
                        ui.menu_button("Or Gate", |ui| {
                            for i in 2..=8 {
                                if ui.button(format!("{} Inputs", i)).clicked() {
                                    self.board.add_or_gate(self.next_id, i, self.last_click_pos);
                                    self.next_id += 1;
                                    ui.close_menu();
                                }
                            }
                        });

                        ui.menu_button("Nor Gate", |ui| {
                            for i in 2..=8 {
                                if ui.button(format!("{} Inputs", i)).clicked() {
                                    self.board
                                        .add_nor_gate(self.next_id, i, self.last_click_pos);
                                    self.next_id += 1;
                                    ui.close_menu();
                                }
                            }
                        });

                        ui.menu_button("Xor Gate", |ui| {
                            for i in 2..=8 {
                                if ui.button(format!("{} Inputs", i)).clicked() {
                                    self.board
                                        .add_xor_gate(self.next_id, i, self.last_click_pos);
                                    self.next_id += 1;
                                    ui.close_menu();
                                }
                            }
                        });

                        ui.menu_button("Joiner", |ui| {
                            for i in 2..=8 {
                                if ui.button(format!("{} Inputs", i)).clicked() {
                                    self.board.add_joiner(self.next_id, i, self.last_click_pos);
                                    self.next_id += 1;
                                    ui.close_menu();
                                }
                            }
                        });

                        ui.menu_button("Splitter", |ui| {
                            for i in 2..=8 {
                                if ui.button(format!("{} Outputs", i)).clicked() {
                                    self.board
                                        .add_splitter(self.next_id, i, self.last_click_pos);
                                    self.next_id += 1;
                                    ui.close_menu();
                                }
                            }
                        });
                    });
                }

                self.draw_subs(ui, transform, id, rect);
                if self.sim.is_some() {
                    ui.ctx().request_repaint();
                }
            });
    }

    pub fn draw_subs(&mut self, ui: &mut Ui, transform: TSTransform, id: Id, rect: Rect) {
        self.update_comp_vals();
        let window_layer = ui.layer_id();
        let mut over_conn: Option<usize> = None;
        let mut i = 0;
        while i < self.board.components.len() {
            let id = egui::Area::new(id.with(("subc", i)))
                .fixed_pos(self.board.comp_pos[i])
                .constrain(false)
                .show(ui.ctx(), |ui| {
                    ui.set_clip_rect(transform.inverse() * rect);
                    self.draw_comp(ui, i, transform, &mut over_conn);
                })
                .response
                .layer_id;
            ui.ctx().set_transform_layer(id, transform);
            ui.ctx().set_sublayer(window_layer, id);
            i += 1;
        }
        self.over_connection = over_conn;
    }
}
