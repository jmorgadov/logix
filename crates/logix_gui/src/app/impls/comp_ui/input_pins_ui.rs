use egui::{Color32, Response, Sense, Shape, Ui};

use crate::app::{impls::constants::*, logix_app::WireDir, LogixApp};

impl LogixApp {
    pub fn draw_input_pins(&mut self, ui: &mut Ui, idx: usize, inputs: Vec<Response>) {
        for (i, resp) in inputs.into_iter().enumerate() {
            let pin_pos = resp.rect.center();

            let resp = ui.interact(resp.rect, resp.id.with(i), Sense::click_and_drag());

            ui.painter()
                .add(Shape::circle_filled(pin_pos, PIN_SIZE / 2.0, Color32::GRAY));

            let color = if self.sim.is_some() {
                match self.board.components[idx].inputs_data[i].value {
                    0 => LOW_COLOR,
                    _ => HIGH_COLOR,
                }
            } else if resp.hovered() {
                Color32::LIGHT_BLUE
            } else {
                Color32::LIGHT_GRAY
            };
            ui.painter()
                .add(Shape::circle_filled(pin_pos, PIN_SIZE / 2.0, color));

            // If a new connection was been started, add the user clicked on the pin
            // then add the connection to the board
            let mut connection_added = false;
            if let Some((from, points)) = self.new_conn.as_mut() {
                if resp.clicked()
                    && self.board.components[from.0].outputs_data[from.1].size
                        == self.board.components[idx].inputs_data[i].size
                {
                    connection_added = true;
                    let last_point = points.last().unwrap().clone();
                    let next_orientation = last_point.1.opposite();
                    let ghost_point = Self::get_ghost_point(last_point, pin_pos);
                    points.push((ghost_point, next_orientation));
                    points.push((pin_pos, WireDir::Horizontal));

                    let to_conn = (idx, i);
                    self.board.add_conn(
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
    }
}
