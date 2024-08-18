use egui::{epaint::PathShape, Color32, Pos2, Rangef, Rect, Sense, Shape, Stroke};

use crate::app::{impls::constants::*, logix_app::WireDir, LogixApp};

impl LogixApp {
    pub fn draw_comp_conns(
        &mut self,
        ui: &mut egui::Ui,
        idx: usize,
        over_conn: &mut Option<usize>,
    ) {
        let mut i = 0;
        while i < self.current_comp.connections.len() {
            let conn = &self.current_comp.connections[i];
            if conn.from.0 == idx {
                let from_port = conn.from.1;
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

                    if self.sim.is_some() {
                        let data = self.current_comp.components[idx].outputs_data[from_port];
                        let val_in_bits =
                            format!("{:0width$b}", data.value, width = data.size as usize);
                        resp.on_hover_text(format!("{} - {}", val_in_bits, data.value));
                    }

                    let color = if self.sim.is_some() {
                        match self.current_comp.components[idx].outputs_data[from_port].value {
                            0 => LOW_COLOR,
                            _ => HIGH_COLOR,
                        }
                    } else if self.over_connection.is_some_and(|k| k == i) {
                        Color32::LIGHT_BLUE
                    } else {
                        Color32::WHITE
                    };

                    let is_one_bit_data =
                        self.current_comp.components[idx].outputs_data[from_port].size == 1;
                    let stroke_with = if is_one_bit_data { 2.0 } else { 4.0 };

                    ui.painter().add(Shape::Path(PathShape::line(
                        vec![p1, p2],
                        Stroke::new(stroke_with, color),
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
            i += 1;
        }
    }
}