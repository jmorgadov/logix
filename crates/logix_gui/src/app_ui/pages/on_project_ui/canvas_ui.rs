use std::path::{Path, PathBuf};

use egui::{emath::TSTransform, epaint::PathShape, Color32, Id, Rect, Response, Shape, Stroke, Ui};

use crate::app_ui::{
    board_editing::BoardEditing, comp_board::ComponentBoard,
    pages::on_project_ui::wire_dir::WireDir,
};

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

                // Allow dragging the background as well.
                if response.dragged() {
                    self.transform.translation += response.drag_delta();
                }

                if response.double_clicked() {
                    self.transform = TSTransform::default();
                }

                let transform = TSTransform::from_translation(ui.min_rect().left_top().to_vec2())
                    * self.transform;

                if let Some(pointer) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                    let pointer_in_layer = transform.inverse() * pointer;
                    let zoom_delta = ui.ctx().input(egui::InputState::zoom_delta);

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

                if let Some(comp) = response.dnd_release_payload::<PathBuf>() {
                    if comp.to_path_buf() == self.file.clone() {
                        self.notify_err("Cannot import components recursively");
                        return;
                    }
                    let pos = transform.inverse() * response.hover_pos().unwrap();
                    self.import_comp(comp.as_path(), pos);
                }

                // Delete new connection if escape is pressed
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    self.new_conn = None;
                }

                // Draw the new connection if user is creating one
                if self.new_conn.is_some() {
                    self.draw_new_conn(ui, &response, transform);
                }

                // Canvas context menu for adding components
                if response.hovered() || response.context_menu_opened() {
                    self.draw_canvas_menu(&response);
                }

                // Draw the components
                self.draw_subs(ui, transform, id, rect);

                if self.sim.is_some() {
                    // Alwais repaint if simulation is running
                    ui.ctx().request_repaint();
                }
            });
    }

    pub fn add_comp_button(
        &mut self,
        ui: &mut Ui,
        name: &str,
        add_fn: impl FnOnce(&mut ComponentBoard, usize),
    ) {
        if ui.button(name).clicked() {
            add_fn(&mut self.board, self.next_id);
            self.next_id += 1;
            ui.close_menu();
        }
    }

    pub fn editing_menu(&mut self, ui: &mut Ui) {
        ui.label("Board name");
        if ui
            .add(egui::TextEdit::singleline(&mut self.board.name))
            .lost_focus()
        {
            ui.close_menu();
        };

        let cursor_pos = self.last_click_pos;

        ui.separator();

        self.add_comp_button(ui, "Clock", |board, id| {
            board.add_clock_gate(id, cursor_pos);
        });
        self.add_comp_button(ui, "High Const", |board, id| {
            board.add_const_high_gate(id, cursor_pos);
        });
        self.add_comp_button(ui, "Low Const", |board, id| {
            board.add_const_low_gate(id, cursor_pos);
        });
        self.add_comp_button(ui, "Not", |board, id| board.add_not_gate(id, cursor_pos));

        ui.menu_button("Input", |ui| {
            for i in 1..=8 {
                self.add_comp_button(ui, format!("{i} Bits").as_str(), |board, id| {
                    board.add_input(id, i, cursor_pos);
                });
            }
        });

        ui.menu_button("Output", |ui| {
            for i in 1..=8 {
                self.add_comp_button(ui, format!("{i} Bits").as_str(), |board, id| {
                    board.add_output(id, i, cursor_pos);
                });
            }
        });

        ui.menu_button("And Gate", |ui| {
            for i in 2..=8 {
                self.add_comp_button(ui, format!("{i} Inputs").as_str(), |board, id| {
                    board.add_and_gate(id, i, cursor_pos);
                });
            }
        });

        ui.menu_button("Nand Gate", |ui| {
            for i in 2..=8 {
                self.add_comp_button(ui, format!("{i} Inputs").as_str(), |board, id| {
                    board.add_nand_gate(id, i, cursor_pos);
                });
            }
        });

        ui.menu_button("Or Gate", |ui| {
            for i in 2..=8 {
                self.add_comp_button(ui, format!("{i} Inputs").as_str(), |board, id| {
                    board.add_or_gate(id, i, cursor_pos);
                });
            }
        });

        ui.menu_button("Nor Gate", |ui| {
            for i in 2..=8 {
                self.add_comp_button(ui, format!("{i} Inputs").as_str(), |board, id| {
                    board.add_nor_gate(id, i, cursor_pos);
                });
            }
        });

        ui.menu_button("Xor Gate", |ui| {
            for i in 2..=8 {
                self.add_comp_button(ui, format!("{i} Inputs").as_str(), |board, id| {
                    board.add_xor_gate(id, i, cursor_pos);
                });
            }
        });

        ui.menu_button("Joiner", |ui| {
            for i in 2..=8 {
                self.add_comp_button(ui, format!("{i} Inputs").as_str(), |board, id| {
                    board.add_joiner(id, i, cursor_pos);
                });
            }
        });

        ui.menu_button("Splitter", |ui| {
            for i in 2..=8 {
                self.add_comp_button(ui, format!("{i} Outputs").as_str(), |board, id| {
                    board.add_splitter(id, i, cursor_pos);
                });
            }
        });
    }

    pub fn draw_canvas_menu(&mut self, response: &Response) {
        response.context_menu(|ui| {
            ui.set_max_width(150.0);
            self.editing_menu(ui);
        });
    }

    pub fn import_comp<P: AsRef<Path>>(&mut self, file: P, pos: egui::Pos2) {
        let buf = file.as_ref().to_path_buf();
        if let Ok(comp_file) = buf.strip_prefix(self.project_folder.clone()) {
            if self.board.import_comp(self.next_id, comp_file, pos).is_ok() {
                self.next_id += 1;
            }
        }
    }

    fn draw_new_conn(&mut self, ui: &Ui, response: &egui::Response, transform: TSTransform) {
        let new_conn = self.new_conn.as_mut().unwrap();
        if response.hovered() && response.clicked_by(egui::PointerButton::Primary) {
            let last_point = *new_conn.1.last().unwrap();
            let cursor_pos = transform.inverse() * response.interact_pointer_pos().unwrap();
            let new_point =
                Self::get_ghost_point(last_point, WireDir::get_dir(new_conn.1.len()), cursor_pos);
            new_conn.1.push(new_point);
        }
        ui.painter().add(Shape::Path(PathShape::line(
            new_conn.1.iter().map(|p| transform * *p).collect(),
            Stroke::new(2.0, Color32::WHITE),
        )));
    }

    fn draw_subs(&mut self, ui: &Ui, transform: TSTransform, id: Id, rect: Rect) {
        self.update_comp_vals();
        let window_layer = ui.layer_id();
        let mut over_conn: Option<usize> = None;
        let mut i = 0;
        while i < self.current_sim_board().components.len() {
            let id = egui::Area::new(id.with(("subc", i)))
                .fixed_pos(self.current_sim_board().comp_pos[i])
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
