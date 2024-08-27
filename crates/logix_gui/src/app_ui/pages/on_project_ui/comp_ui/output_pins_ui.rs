use egui::{Color32, Response, Sense, Shape, Ui};

use crate::app_ui::{
    board_editing::BoardEditing,
    pages::on_project_ui::constants::{HIGH_COLOR, LOW_COLOR, PIN_SIZE},
};

impl BoardEditing {
    pub fn draw_output_pins(&mut self, ui: &Ui, idx: usize, outputs: &[Response]) {
        for (i, resp) in outputs.iter().enumerate() {
            let pin_pos = resp.rect.center();
            let resp = ui.interact(resp.rect, resp.id.with(i), Sense::click_and_drag());

            let color = if self.sim.is_some() {
                match self.current_sim_board().components[idx].outputs_data[i].value {
                    0 => LOW_COLOR,
                    _ => HIGH_COLOR,
                }
            } else if resp.hovered() {
                Color32::LIGHT_RED
            } else {
                Color32::LIGHT_GRAY
            };
            ui.painter()
                .add(Shape::circle_filled(pin_pos, PIN_SIZE / 2.0, color));

            if self.sim.is_none() && resp.clicked() {
                self.new_conn = Some(((idx, i), vec![pin_pos]));
            }
        }
    }
}
