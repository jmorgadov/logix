use egui::{Color32, Pos2, Rangef, Rect, Sense, Shape, Ui, Vec2};

use crate::app::{impls::constants::*, logix_app::WireDir, LogixApp};

impl LogixApp {
    pub fn draw_output_pins(&mut self, ui: &mut Ui, idx: usize, outputs: Vec<Pos2>) {
        for (i, pin_pos) in outputs.iter().enumerate() {
            let resp = ui.interact(
                Rect::from_center_size(pin_pos.clone(), Vec2::splat(PIN_SIZE)),
                ui.id().with(("output", i, idx)),
                Sense::click(),
            );
            let pin_name_rect = Rect::from_x_y_ranges(
                Rangef::new(pin_pos.x - 50.0, pin_pos.x - 8.0),
                Rangef::new(pin_pos.y - PIN_SIZE, pin_pos.y + PIN_SIZE),
            );

            let pin_name = self.current_comp.components[idx].outputs_name[i].clone();
            ui.allocate_ui_at_rect(pin_name_rect, |ui| {
                // ui.horizontal_centered(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(egui::Label::new(pin_name).selectable(false));
                });
                // });
            });
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
