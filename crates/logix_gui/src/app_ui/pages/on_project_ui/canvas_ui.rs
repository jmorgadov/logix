use std::path::Path;

use egui::{emath::TSTransform, epaint::PathShape, Color32, Id, Rect, Response, Shape, Stroke, Ui};

use crate::app_ui::{
    board::BoardComponent,
    board_editing::BoardEditing,
    pages::on_project_ui::{canvas_payload::CanvasPayload, wire_dir::WireDir},
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

                if let Some(payload) = response.dnd_release_payload::<CanvasPayload>() {
                    let pos = transform.inverse() * response.hover_pos().unwrap();
                    match payload.as_ref() {
                        CanvasPayload::Component(comp) => {
                            self.board.add_comp(
                                BoardComponent::from_comp_info(comp.clone())
                                    .with_pos(pos)
                                    .with_id(self.next_id),
                            );
                        }
                        CanvasPayload::Path(path) => {
                            if path.clone() == self.file.clone() {
                                self.notify_err("Cannot import components recursively");
                                return;
                            }
                            let pos = transform.inverse() * response.hover_pos().unwrap();
                            self.import_comp(path.as_path(), pos);
                        }
                    }
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

    pub fn draw_canvas_menu(&mut self, response: &Response) {
        response.context_menu(|ui| {
            ui.set_max_width(150.0);
            ui.label("Board name");
            if ui
                .add(egui::TextEdit::singleline(&mut self.board.name))
                .lost_focus()
            {
                ui.close_menu();
            };
        });
    }

    pub fn import_comp<P: AsRef<Path>>(&mut self, file: P, pos: egui::Pos2) {
        let buf = file.as_ref().to_path_buf();
        if let Ok(comp_file) = buf.strip_prefix(self.project_folder.clone()) {
            self.board
                .import_comp(self.next_id, comp_file, pos)
                .expect("Error importing component");
            self.next_id += 1;
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
                .fixed_pos(self.current_sim_board().components[i].pos)
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
