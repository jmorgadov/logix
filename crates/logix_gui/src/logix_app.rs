use egui::{
    emath::TSTransform, epaint::PathShape, CollapsingHeader, Color32, Id, Pos2, Rangef, Rect,
    Rounding, Sense, Shape, Stroke, Ui, Vec2,
};
use logix_core::component::{Component, PortAddr};
use logix_sim::primitives::primitives::ExtraInfo;
use rfd::FileDialog;
use std::path::PathBuf;

use crate::{comp_board::ComponentBoard, folder_tree::Folder};

const PIN_SIZE: f32 = 8.0;
const PIN_MARGIN: f32 = 15.0;

#[derive(Debug, Clone, Copy)]
enum WireDir {
    Horizontal,
    Vertical,
}

impl WireDir {
    pub fn opposite(&self) -> Self {
        match self {
            WireDir::Horizontal => WireDir::Vertical,
            WireDir::Vertical => WireDir::Horizontal,
        }
    }
}

pub struct LogixApp {
    folder: Folder,
    selected_file: Option<PathBuf>,
    transform: TSTransform,
    current_comp: ComponentBoard,
    last_id: usize,
    new_conn: Option<(PortAddr, Vec<(Pos2, WireDir)>)>,
    last_click_pos: Pos2,
    over_connection: Option<usize>,
}

impl Default for LogixApp {
    fn default() -> Self {
        let current_folder = std::env::current_dir().unwrap().display().to_string();
        Self {
            folder: Folder::from_str_path(&current_folder),
            selected_file: None,
            transform: TSTransform::default(),
            current_comp: Default::default(),
            last_id: 0,
            last_click_pos: Pos2::ZERO,
            new_conn: None,
            over_connection: None,
        }
    }
}

impl Folder {
    fn ui_impl(&mut self, ui: &mut Ui, selected_file: Option<&PathBuf>) -> Option<PathBuf> {
        let mut new_file = selected_file.cloned();
        for folder in self.folders.iter_mut() {
            let name = folder.current_path.file_name().unwrap().to_str().unwrap();
            CollapsingHeader::new(name).show(ui, |ui| {
                new_file = folder.ui_impl(ui, selected_file);
            });
        }

        for file in self.files.iter() {
            let name = file.file_name().unwrap().to_str().unwrap();
            // ui.label(name);
            let mut color = Color32::TRANSPARENT;
            if let Some(selected_file) = selected_file {
                if file == selected_file {
                    color = Color32::from_rgb(40, 40, 40);
                }
            }
            egui::Frame::default().fill(color).show(ui, |ui| {
                ui.allocate_space(Vec2::new(ui.available_width(), 0.0));
                let resp = ui.add(
                    egui::Label::new(name)
                        .selectable(false)
                        .wrap_mode(egui::TextWrapMode::Truncate)
                        .sense(Sense::click()),
                );
                if resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                if resp.double_clicked() {
                    new_file = Some(file.clone());
                }
            });
        }

        new_file
    }
}

impl LogixApp {
    fn file_menu(&mut self, ui: &mut Ui) {
        ui.set_max_width(200.0); // To make sure we wrap long text

        if ui.button("New Board").clicked() {
            ui.close_menu();
        }
        ui.separator();
        if ui.button("Open folder").clicked() {
            let new_folder = FileDialog::new().pick_folder();
            self.folder = Folder::from_pathbuf(&new_folder.unwrap());
            ui.close_menu();
        }
        ui.separator();
        if ui.button("Exit").clicked() {
            std::process::exit(0);
        }
    }

    fn top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| self.file_menu(ui));
            });
        });
    }

    fn left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("side_panel")
            .min_width(180.0)
            .show(ctx, |ui| {
                ui.heading(
                    self.folder
                        .current_path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap(),
                );
                egui::ScrollArea::vertical()
                    .max_width(180.0)
                    .show(ui, |ui| {
                        let new_file = self.folder.ui_impl(ui, self.selected_file.as_ref());
                        if new_file != self.selected_file {
                            // Load the new file
                            //
                            // This is where we would load the file and parse it.
                            //
                            self.selected_file = new_file;
                        }
                    });
            });
    }

    fn draw_canvas(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
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

            let transform =
                TSTransform::from_translation(ui.min_rect().left_top().to_vec2()) * self.transform;

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
                if response.hovered() {
                    if response.clicked_by(egui::PointerButton::Primary) {
                        let last_point = new_conn.1.last().unwrap();
                        let cursor_pos =
                            transform.inverse() * response.interact_pointer_pos().unwrap();
                        let new_point = Self::get_ghost_point(last_point.clone(), cursor_pos);
                        new_conn.1.push((new_point, last_point.1.opposite()));
                    }
                }
                ui.painter().add(Shape::Path(PathShape::line(
                    new_conn.1.iter().map(|(p, _)| transform * *p).collect(),
                    Stroke::new(2.0, Color32::WHITE),
                )));
            }

            if response.hovered() || response.context_menu_opened() {
                response.context_menu(|ui| {
                    ui.label("Add Component");
                    if ui.button("And Gate").clicked() {
                        self.current_comp
                            .add_and_gate(self.last_id, 2, self.last_click_pos);
                        self.last_id += 1;
                        ui.close_menu();
                    }
                    if ui.button("Or Gate").clicked() {
                        self.current_comp
                            .add_or_gate(self.last_id, 3, self.last_click_pos);
                        self.last_id += 1;
                        ui.close_menu();
                    }
                });
            }

            self.draw_subs(ui, transform, id, rect);
        });
    }

    fn sub_draw_info(local_pos: Pos2, sub: &Component<ExtraInfo>) -> (Rect, Vec<Pos2>, Vec<Pos2>) {
        let in_count = sub.inputs;
        let out_count = sub.outputs;
        let in_height = (sub.inputs as f32) * (PIN_MARGIN) + PIN_MARGIN;
        let out_height = (sub.outputs as f32) * (PIN_MARGIN) + PIN_MARGIN;

        let height = in_height.max(out_height);

        let in_offset: f32 = (height - in_height) / 2.0;
        let out_offset: f32 = (height - out_height) / 2.0;

        let comp_size = Vec2::new(50.0, height);
        // let local_pos = self.current_comp.subc_pos[idx];
        let s_rect = Rect::from_min_size(local_pos, comp_size);

        let inputs = (0..in_count)
            .map(|i| {
                Pos2::new(
                    local_pos.x,
                    local_pos.y + in_offset + i as f32 * (PIN_MARGIN) + PIN_MARGIN,
                )
            })
            .collect();

        let outputs = (0..out_count)
            .map(|i| {
                Pos2::new(
                    local_pos.x + comp_size.x,
                    local_pos.y + out_offset + i as f32 * (PIN_MARGIN) + PIN_MARGIN,
                )
            })
            .collect();
        (s_rect, inputs, outputs)
    }

    fn draw_subs(&mut self, ui: &mut Ui, transform: TSTransform, id: Id, rect: Rect) {
        let window_layer = ui.layer_id();
        let mut over_conn: Option<usize> = None;
        let mut i = 0;
        while i < self.current_comp.components.len() {
            let id = egui::Area::new(id.with(("subc", i)))
                .fixed_pos(self.current_comp.comp_pos[i])
                .show(ui.ctx(), |ui| {
                    ui.set_clip_rect(transform.inverse() * rect);
                    self.draw_subc(ui, i, transform, &mut over_conn);
                })
                .response
                .layer_id;
            ui.ctx().set_transform_layer(id, transform);
            ui.ctx().set_sublayer(window_layer, id);
            i += 1;
        }
        self.over_connection = over_conn;
    }

    fn get_ghost_point(last_point: (Pos2, WireDir), cursor_pos: Pos2) -> Pos2 {
        match last_point.1 {
            WireDir::Horizontal => Pos2::new(cursor_pos.x, last_point.0.y),
            WireDir::Vertical => Pos2::new(last_point.0.x, cursor_pos.y),
        }
    }

    fn update_subc_pos(&mut self, idx: usize, new_pos: Pos2) {
        // Update positions vector
        self.current_comp.comp_pos[idx] = new_pos;
        let sub = self.current_comp.components.get(idx).unwrap();
        let (_, inputs, outputs) = Self::sub_draw_info(self.current_comp.comp_pos[idx], sub);

        // Update connections related to the subcomponent
        let conns_count = self.current_comp.connections.len();
        for i in 0..conns_count {
            let conn = &self.current_comp.connections[i];
            let conn_info = &mut self.current_comp.comp_conns[i];

            // If it is an output connection
            if conn.from.0 == idx {
                let points = &mut conn_info.points;
                points[0] = outputs[conn.from.1].clone();
                points[1].y = points[0].y;
            }

            // If it is an input connection
            if conn.to.0 == idx {
                let points = &mut conn_info.points;
                let p_count = points.len();
                points[p_count - 1] = inputs[conn.to.1].clone();
                points[p_count - 2].y = points[p_count - 1].y;
            }
        }
    }

    fn draw_subc(
        &mut self,
        ui: &mut Ui,
        idx: usize,
        transform: TSTransform,
        over_conn: &mut Option<usize>,
    ) {
        // -----------------------------------------------------------------------------
        // USE LOCAL COORDINATES IN THIS FUNCTION. The transform is applied
        // by the caller.
        // -----------------------------------------------------------------------------

        let sub = self.current_comp.components.get(idx).unwrap();
        let sub_name = sub.name.as_ref().unwrap().clone();

        let (s_rect, inputs, outputs) = Self::sub_draw_info(self.current_comp.comp_pos[idx], sub);

        ui.painter().add(Shape::Vec(vec![Shape::rect_filled(
            s_rect,
            Rounding::same(4.0),
            Color32::from_rgb(50, 50, 50),
        )]));

        // -----------------------------------------------------------------------------
        // Draw the connections comming from this subcomponent
        // -----------------------------------------------------------------------------
        for i in 0..self.current_comp.connections.len() {
            let conn = &self.current_comp.connections[i];
            if conn.from.0 == idx {
                let mut to_add: Vec<(usize, Pos2, WireDir)> = vec![];
                let mut to_remove: Vec<usize> = vec![];
                let points: Vec<Pos2> = self.current_comp.comp_conns[i]
                    .points
                    .iter()
                    .map(|p| (*p))
                    .collect();

                let mut c_orient = WireDir::Horizontal;
                for j in 0..points.len() - 1 {
                    let p1 = points[j];
                    let p2 = points[j + 1];

                    let min_x = p1.x.min(p2.x);
                    let max_x = p1.x.max(p2.x);
                    let min_y = p1.y.min(p2.y);
                    let max_y = p1.y.max(p2.y);
                    let sub_wire_rect = match c_orient {
                        WireDir::Horizontal => Rect::from_x_y_ranges(
                            Rangef::new(min_x, max_x),
                            Rangef::new(min_y - 4.0, max_y + 4.0),
                        ),
                        WireDir::Vertical => Rect::from_x_y_ranges(
                            Rangef::new(min_x - 4.0, max_x + 4.0),
                            Rangef::new(min_y, max_y),
                        ),
                    };

                    let resp = ui.interact(
                        sub_wire_rect,
                        ui.id().with(("wire", i, j)),
                        Sense::click_and_drag(),
                    );

                    let is_midd_wire: bool = j != 0 && j != points.len() - 2;

                    if resp.contains_pointer() {
                        *over_conn = Some(i);
                        if is_midd_wire {
                            ui.ctx().set_cursor_icon(match c_orient {
                                WireDir::Horizontal => egui::CursorIcon::ResizeVertical,
                                WireDir::Vertical => egui::CursorIcon::ResizeHorizontal,
                            });
                        }
                    }

                    if is_midd_wire && resp.dragged() {
                        let delta = resp.drag_delta();
                        match c_orient {
                            WireDir::Vertical => {
                                self.current_comp.comp_conns[i].points[j].x += delta.x;
                                self.current_comp.comp_conns[i].points[j + 1].x += delta.x;
                            }
                            WireDir::Horizontal => {
                                self.current_comp.comp_conns[i].points[j].y += delta.y;
                                self.current_comp.comp_conns[i].points[j + 1].y += delta.y;
                            }
                        }
                    }

                    if resp.hovered() && resp.clicked_by(egui::PointerButton::Secondary) {
                        self.last_click_pos = resp.interact_pointer_pos().unwrap();
                        match c_orient {
                            WireDir::Horizontal => {
                                self.last_click_pos.y = p1.y;
                            }
                            WireDir::Vertical => {
                                self.last_click_pos.x = p1.x;
                            }
                        }
                    }

                    resp.context_menu(|ui| {
                        if ui.button("Add point").clicked() {
                            to_add.push((j + 1, self.last_click_pos, c_orient.opposite()));
                            to_add.push((j + 1, self.last_click_pos, c_orient));
                        }
                        if ui.button("Remove Connection").clicked() {
                            self.current_comp.remove_conn(i);
                        }
                    });

                    let color = if self.over_connection.is_some_and(|k| k == i) {
                        Color32::LIGHT_RED
                    } else {
                        Color32::WHITE
                    };
                    ui.painter().add(Shape::Path(PathShape::line(
                        vec![p1, p2],
                        Stroke::new(2.0, color),
                    )));
                    if j > 0 {
                        ui.painter().add(Shape::circle_filled(p1, 3.0, color));
                    }
                    c_orient = c_orient.opposite();
                }

                for p in to_add {
                    self.current_comp.comp_conns[i].points.insert(p.0, p.1);
                }

                to_remove.sort();
                to_remove.reverse();
                for idx in to_remove {
                    self.current_comp.comp_conns[i].points.remove(idx);
                }
            }
        }

        // -----------------------------------------------------------------------------
        // Draw the new connection being created if there is one
        // -----------------------------------------------------------------------------
        if let Some((from, points)) = self.new_conn.as_mut() {
            let cursor_pos = ui.ctx().pointer_hover_pos();
            if from.0 == idx && cursor_pos.is_some() {
                let cursor_pos = transform.inverse() * cursor_pos.unwrap();
                let (from_pos, last_dir) = points.last().unwrap().clone();

                let x_diff = cursor_pos.x - from_pos.x;
                let y_diff = cursor_pos.y - from_pos.y;

                let mut points = vec![from_pos];

                if x_diff.abs() > 10.0 && y_diff.abs() > 10.0 {
                    let ghost_point = Self::get_ghost_point((from_pos, last_dir), cursor_pos);
                    points.push(ghost_point);
                    points.push(cursor_pos);
                } else if x_diff.abs() > 10.0 {
                    points.push(Pos2::new(cursor_pos.x, from_pos.y));
                } else if y_diff.abs() > 10.0 {
                    points.push(Pos2::new(from_pos.x, cursor_pos.y));
                }

                ui.painter().add(Shape::Path(PathShape::line(
                    points,
                    Stroke::new(2.0, Color32::WHITE),
                )));
            }
        }

        let mut resp = ui.put(s_rect, egui::Label::new(sub_name.clone()).selectable(false));
        resp = resp.interact(Sense::click_and_drag());

        // -----------------------------------------------------------------------------
        // Handle dragging the subcomponent
        // -----------------------------------------------------------------------------
        if resp.dragged() && self.new_conn.is_none() {
            self.update_subc_pos(idx, self.current_comp.comp_pos[idx] + resp.drag_delta());
        }

        // -----------------------------------------------------------------------------
        // Add the input pins for the subcomponent
        // -----------------------------------------------------------------------------
        for (i, pin_pos) in inputs.into_iter().enumerate() {
            let resp = ui.interact(
                Rect::from_center_size(pin_pos, Vec2::splat(PIN_SIZE)),
                ui.id().with(("input", i, idx)),
                Sense::click_and_drag(),
            );
            ui.painter()
                .add(Shape::circle_filled(pin_pos, PIN_SIZE / 2.0, Color32::GRAY));

            let color = if resp.hovered() {
                Color32::LIGHT_RED
            } else {
                Color32::LIGHT_GRAY
            };
            ui.painter()
                .add(Shape::circle_filled(pin_pos, PIN_SIZE / 2.0, color));

            // If a new connection was been started, add the user clicked on the pin
            // then add the connection to the board
            let mut connection_added = false;
            if let Some((from, points)) = self.new_conn.as_mut() {
                if resp.clicked() {
                    connection_added = true;
                    let last_point = points.last().unwrap().clone();
                    let next_orientation = last_point.1.opposite();
                    let ghost_point = Self::get_ghost_point(last_point, pin_pos);
                    points.push((ghost_point, next_orientation));
                    points.push((pin_pos, WireDir::Horizontal));

                    let to_conn = (idx, i);
                    self.current_comp.add_conn(
                        from.0,
                        to_conn.0,
                        from.1,
                        to_conn.1,
                        points.iter().map(|(p, _)| *p).collect(),
                    );
                }
            }
            if connection_added {
                self.new_conn = None;
            }
        }

        // -----------------------------------------------------------------------------
        // Add the output pins for the subcomponent
        // -----------------------------------------------------------------------------
        for (i, pin_pos) in outputs.iter().enumerate() {
            let resp = ui.interact(
                Rect::from_center_size(pin_pos.clone(), Vec2::splat(PIN_SIZE)),
                ui.id().with(("output", i, idx)),
                Sense::click(),
            );
            let color = if resp.hovered() {
                Color32::LIGHT_RED
            } else {
                Color32::LIGHT_GRAY
            };
            ui.painter()
                .add(Shape::circle_filled(pin_pos.clone(), PIN_SIZE / 2.0, color));

            if resp.clicked() {
                self.new_conn = Some(((idx, i), vec![(pin_pos.clone(), WireDir::Horizontal)]));
            }
        }

        // -----------------------------------------------------------------------------
        // Handle context menu for the subcomponent
        // -----------------------------------------------------------------------------
        if resp.hovered() || resp.context_menu_opened() {
            resp.context_menu(|ui| {
                if ui.button("Remove").clicked() {
                    self.current_comp.remove_subc(idx);
                    ui.close_menu();
                }
            });
        }
    }
}

impl eframe::App for LogixApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.top_panel(ctx);
        self.left_panel(ctx);
        self.draw_canvas(ctx);
    }
}
