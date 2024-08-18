use egui::{emath::TSTransform, epaint::PathShape, Color32, Pos2, Shape, Stroke};

use crate::app::LogixApp;

impl LogixApp {
    pub fn draw_new_connection(&mut self, ui: &mut egui::Ui, idx: usize, transform: TSTransform) {
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
    }
}
