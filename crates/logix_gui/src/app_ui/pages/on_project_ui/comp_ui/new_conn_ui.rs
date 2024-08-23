use egui::{emath::TSTransform, epaint::PathShape, Color32, Pos2, Shape, Stroke};

use crate::app_ui::{
    board_editing::BoardEditing,
    pages::on_project_ui::{constants::GHOST_POINT_THRESHOLD, wire_dir::WireDir},
};

impl BoardEditing {
    pub fn draw_new_connection(&mut self, ui: &egui::Ui, idx: usize, transform: TSTransform) {
        if let Some((from, points)) = self.new_conn.as_mut() {
            let cursor_pos = ui.ctx().pointer_hover_pos();
            if from.0 == idx && cursor_pos.is_some() {
                let cursor_pos = transform.inverse() * cursor_pos.unwrap();
                let from_pos = *points.last().unwrap();

                let x_diff = cursor_pos.x - from_pos.x;
                let y_diff = cursor_pos.y - from_pos.y;

                let mut ghost_points = vec![from_pos];

                if x_diff.abs() > GHOST_POINT_THRESHOLD && y_diff.abs() > GHOST_POINT_THRESHOLD {
                    let ghost_point =
                        Self::get_ghost_point(from_pos, WireDir::get_dir(points.len()), cursor_pos);
                    ghost_points.push(ghost_point);
                    ghost_points.push(cursor_pos);
                } else if x_diff.abs() > 10.0 {
                    ghost_points.push(Pos2::new(cursor_pos.x, from_pos.y));
                } else if y_diff.abs() > 10.0 {
                    ghost_points.push(Pos2::new(from_pos.x, cursor_pos.y));
                }

                ui.painter().add(Shape::Path(PathShape::line(
                    ghost_points,
                    Stroke::new(2.0, Color32::WHITE),
                )));
            }
        }
    }
}
