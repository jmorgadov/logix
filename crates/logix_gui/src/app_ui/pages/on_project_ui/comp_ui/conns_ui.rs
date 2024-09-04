use egui::{epaint::PathShape, Color32, Pos2, Rect, Sense, Shape, Stroke};
use logix_core::component::Conn;

use crate::app_ui::{
    board::BoardAction,
    board_editing::BoardEditing,
    pages::on_project_ui::{
        constants::{HIGH_COLOR, LOW_COLOR},
        wire_dir::WireDir,
    },
};

impl BoardEditing {
    pub fn draw_comp_conns(&mut self, ui: &egui::Ui, idx: usize, over_conn: &mut Option<usize>) {
        let mut i = 0;
        while i < self.current_sim_board().conns.len() {
            let conn = self.current_sim_board().conns[i].conn;
            if conn.from.0 == idx {
                self.draw_connection(ui, &conn, i, over_conn, idx);
            }
            i += 1;
        }
    }

    #[allow(clippy::too_many_lines)]
    fn draw_connection(
        &mut self,
        ui: &egui::Ui,
        conn: &Conn,
        i: usize,
        over_conn: &mut Option<usize>,
        idx: usize,
    ) {
        let from_port = conn.from.1;
        let mut to_add: Vec<(usize, Pos2, WireDir)> = vec![];
        let mut to_remove: Vec<usize> = vec![];
        let points: Vec<Pos2> = self.current_sim_board().conns[i].points.clone();

        for j in 0..points.len() - 1 {
            let p1 = points[j];
            let p2 = points[j + 1];
            let c_orient = WireDir::get_dir(j + 1);

            let resp = ui.interact(
                Rect::from_two_pos(p1, p2).expand(4.0),
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

            if self.sim.is_none() && is_midd_wire {
                if resp.drag_started() {
                    self.dragging_conn_seg = Some((i, j, p1, p2));
                }
                if resp.dragged() {
                    let delta = resp.drag_delta();
                    match c_orient {
                        WireDir::Vertical => {
                            self.board.conns[i].points[j].x += delta.x;
                            self.board.conns[i].points[j + 1].x += delta.x;
                        }
                        WireDir::Horizontal => {
                            self.board.conns[i].points[j].y += delta.y;
                            self.board.conns[i].points[j + 1].y += delta.y;
                        }
                    }
                }
                if resp.drag_stopped() {
                    let (i, j, p1, p2) = self.dragging_conn_seg.expect("No dragging segment");
                    self.board.add_action(BoardAction::move_conn_segment(
                        i,
                        j,
                        (p1, p2),
                        (
                            self.board.conns[i].points[j],
                            self.board.conns[i].points[j + 1],
                        ),
                    ));
                }
            }

            if resp.hovered() && resp.clicked_by(egui::PointerButton::Secondary) {
                self.last_click_pos = resp.interact_pointer_pos().unwrap();
                match c_orient {
                    WireDir::Horizontal => self.last_click_pos.y = p1.y,
                    WireDir::Vertical => self.last_click_pos.x = p1.x,
                }
            }

            resp.context_menu(|ui| {
                if ui.button("Add point").clicked() {
                    to_add.push((j + 1, self.last_click_pos, c_orient.opposite()));
                    to_add.push((j + 1, self.last_click_pos, c_orient));
                }
                if ui.button("Remove Connection").clicked() {
                    self.current_sim_board().remove_conn(i);
                }
            });

            if resp.double_clicked() && self.new_conn.is_none() {
                let cursor_pos = resp.interact_pointer_pos().unwrap();
                let current_pos = match c_orient {
                    WireDir::Horizontal => Pos2::new(cursor_pos.x, p1.y),
                    WireDir::Vertical => Pos2::new(p1.x, cursor_pos.y),
                };
                let mut new_conn_points: Vec<Pos2> = points.iter().take(j + 1).copied().collect();
                new_conn_points.push(current_pos);
                self.new_conn = Some(((idx, from_port), new_conn_points));
            }

            if self.sim.is_some() {
                let data = self.current_sim_board().components[idx].outputs_data[from_port];
                let val_in_bits = format!("{:0width$b}", data.value, width = data.size as usize);
                resp.on_hover_text(format!("{} - {}", val_in_bits, data.value));
            }

            let color = if self.sim.is_some() {
                match self.current_sim_board().components[idx].outputs_data[from_port].value {
                    0 => LOW_COLOR,
                    _ => HIGH_COLOR,
                }
            } else if self.over_connection.is_some_and(|k| k == i) {
                Color32::LIGHT_RED
            } else {
                Color32::WHITE
            };

            let is_one_bit_data =
                self.current_sim_board().components[idx].outputs_data[from_port].size == 1;
            let stroke_with = if is_one_bit_data { 2.0 } else { 4.0 };

            ui.painter().add(Shape::Path(PathShape::line(
                vec![p1, p2],
                Stroke::new(stroke_with, color),
            )));
            if j > 0 {
                ui.painter().add(Shape::circle_filled(p1, 3.0, color));
            }
        }

        for p in to_add {
            self.current_sim_board().conns[i].points.insert(p.0, p.1);
        }

        to_remove.sort_unstable();
        to_remove.reverse();
        for idx in to_remove {
            self.current_sim_board().conns[i].points.remove(idx);
        }
    }
}
