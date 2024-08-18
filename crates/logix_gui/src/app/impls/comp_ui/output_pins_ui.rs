use egui::{Color32, Pos2, Rect, Sense, Shape, Ui, Vec2};

use crate::app::{impls::constants::*, logix_app::WireDir, LogixApp};

impl LogixApp {
    pub fn draw_output_pins(&mut self, ui: &mut Ui, idx: usize, outputs: Vec<Pos2>) {
        for (i, pin_pos) in outputs.iter().enumerate() {
            let resp = ui.interact(
                Rect::from_center_size(pin_pos.clone(), Vec2::splat(PIN_SIZE)),
                ui.id().with(("output", i, idx)),
                Sense::click(),
            );
            let color = if self.sim.is_some() {
                match self.current_comp.components[idx].outputs_data[i].value {
                    0 => LOW_COLOR,
                    _ => HIGH_COLOR,
                }
            } else if resp.hovered() {
                Color32::LIGHT_BLUE
            } else {
                Color32::LIGHT_GRAY
            };
            ui.painter()
                .add(Shape::circle_filled(pin_pos.clone(), PIN_SIZE / 2.0, color));

            if resp.clicked() {
                self.new_conn = Some(((idx, i), vec![(pin_pos.clone(), WireDir::Horizontal)]));
            }
        }
    }
}
