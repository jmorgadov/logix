use egui::{emath::TSTransform, Color32, Pos2, Rect, Rounding, Sense, Shape, Ui, Vec2};

use crate::app::{comp_board::ComponentInfo, impls::constants::*, logix_app::WireDir, LogixApp};

impl LogixApp {
    pub fn get_ghost_point(last_point: (Pos2, WireDir), cursor_pos: Pos2) -> Pos2 {
        match last_point.1 {
            WireDir::Horizontal => Pos2::new(cursor_pos.x, last_point.0.y),
            WireDir::Vertical => Pos2::new(last_point.0.x, cursor_pos.y),
        }
    }

    pub fn update_comp_pos(&mut self, idx: usize, new_pos: Pos2) {
        // Update positions vector
        self.current_comp.comp_pos[idx] = new_pos;
        let sub = self.current_comp.components.get(idx).unwrap();
        let (_, inputs, outputs) = Self::comp_draw_info(self.current_comp.comp_pos[idx], sub);

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

    pub fn comp_draw_info(local_pos: Pos2, comp: &ComponentInfo) -> (Rect, Vec<Pos2>, Vec<Pos2>) {
        let in_count = comp.input_count();
        let out_count = comp.output_count();
        let in_height = (in_count as f32) * (PIN_MARGIN) + PIN_MARGIN;
        let out_height = (out_count as f32) * (PIN_MARGIN) + PIN_MARGIN;

        let height = in_height.max(out_height);

        let in_offset: f32 = (height - in_height) / 2.0;
        let out_offset: f32 = (height - out_height) / 2.0;

        let comp_size = Vec2::new(75.0, height);
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

    pub fn draw_comp(
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

        let comp = self.current_comp.components.get(idx).unwrap();
        let comp_name = comp.name.clone();

        let (s_rect, inputs, outputs) = Self::comp_draw_info(self.current_comp.comp_pos[idx], comp);

        ui.painter().add(Shape::Vec(vec![Shape::rect_filled(
            s_rect,
            Rounding::same(4.0),
            Color32::from_rgb(50, 50, 50),
        )]));

        // -----------------------------------------------------------------------------
        // Draw the connections comming from this component
        // -----------------------------------------------------------------------------
        self.draw_comp_conns(ui, idx, over_conn);

        // -----------------------------------------------------------------------------
        // Draw the new connection being created if there is one
        // -----------------------------------------------------------------------------
        self.draw_new_connection(ui, idx, transform);

        // -----------------------------------------------------------------------------
        // Define the component rect
        // -----------------------------------------------------------------------------
        let mut resp = ui.put(
            s_rect,
            egui::Label::new(comp_name.clone()).selectable(false),
        );
        resp = resp.interact(Sense::click_and_drag());

        // -----------------------------------------------------------------------------
        // Handle dragging the component
        // -----------------------------------------------------------------------------
        if resp.dragged() && self.new_conn.is_none() {
            self.update_comp_pos(idx, self.current_comp.comp_pos[idx] + resp.drag_delta());
        }

        // -----------------------------------------------------------------------------
        // Add the component's pins
        // -----------------------------------------------------------------------------
        self.draw_input_pins(ui, idx, inputs);
        self.draw_output_pins(ui, idx, outputs);

        // -----------------------------------------------------------------------------
        // Handle context menu for the component
        // -----------------------------------------------------------------------------
        if resp.hovered() || resp.context_menu_opened() {
            resp.context_menu(|ui| {
                self.specific_comp_context_menu(ui, idx);
                if ui.button("Remove").clicked() {
                    self.current_comp.remove_comp(idx);
                    ui.close_menu();
                }
            });
        }
    }
}
