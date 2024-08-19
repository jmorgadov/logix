use egui::{emath::TSTransform, Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

use crate::app::{impls::constants::*, logix_app::WireDir, LogixApp};

impl LogixApp {
    pub fn get_ghost_point(last_point: (Pos2, WireDir), cursor_pos: Pos2) -> Pos2 {
        match last_point.1 {
            WireDir::Horizontal => Pos2::new(cursor_pos.x, last_point.0.y),
            WireDir::Vertical => Pos2::new(last_point.0.x, cursor_pos.y),
        }
    }

    pub fn update_comp_pos(&mut self, idx: usize, new_pos: Pos2) {
        self.current_comp.comp_pos[idx] = new_pos;
    }

    pub fn _draw_comp(
        &mut self,
        ui: &mut Ui,
        idx: usize,
    ) -> (Response, Vec<Response>, Vec<Response>) {
        let font_size_y = 15.0;
        let font_id = egui::FontId::monospace(font_size_y);

        let in_count = self.current_comp.components[idx].input_count();
        let out_count = self.current_comp.components[idx].output_count();

        let in_height = (in_count as f32) * font_size_y;
        let out_height = (out_count as f32) * font_size_y;

        let pins_max_height = in_height.max(out_height);

        let in_offset: f32 = (pins_max_height - in_height) / 2.0;
        let out_offset: f32 = (pins_max_height - out_height) / 2.0;

        let height = pins_max_height;
        let name_offset: f32 = (height - font_size_y) / 2.0;

        let mut in_resps = vec![];
        let mut out_resps = vec![];

        let in_names = &self.current_comp.components[idx].inputs_name;
        let out_names = &self.current_comp.components[idx].outputs_name;

        let out_names_max_len = out_names.iter().map(|x| x.len()).max().unwrap_or(0);
        let out_names = out_names
            .iter()
            .map(|x| format!("{: >width$}", x, width = out_names_max_len))
            .collect::<Vec<String>>();

        let mut resp = egui::Frame::default()
            .fill(Color32::from_rgb(50, 50, 50))
            .inner_margin(egui::Margin::symmetric(0.0, font_size_y / 4.0))
            .rounding(4.0)
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    // Inputs
                    let r = ui
                        .with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                            ui.add_space(in_offset);
                            for i in 0..in_count {
                                ui.with_layout(
                                    egui::Layout::left_to_right(egui::Align::Min),
                                    |ui| {
                                        let r = ui
                                            .add(egui::Label::new(
                                                egui::RichText::new(format!(
                                                    " {}",
                                                    in_names[i].clone()
                                                ))
                                                .font(font_id.clone())
                                                .line_height(Some(font_size_y)),
                                            ))
                                            .rect;
                                        let pos = Pos2::new(r.left(), r.center().y);
                                        let resp = ui.interact(
                                            Rect::from_center_size(
                                                pos,
                                                Vec2::splat(PIN_SIZE * 2.0),
                                            ),
                                            ui.id().with(("input", i, idx)),
                                            Sense::click_and_drag(),
                                        );
                                        in_resps.push(resp);
                                    },
                                );
                            }
                        })
                        .response
                        .rect;
                    println!("r: {:?}", r.size());

                    // Name
                    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                        ui.add_space(name_offset);
                        ui.add(egui::Label::new(
                            egui::RichText::new(self.current_comp.components[idx].name.clone())
                                .font(font_id.clone())
                                .line_height(Some(font_size_y))
                                .color(Color32::WHITE),
                        ));
                    });

                    // Outputs
                    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                        ui.add_space(out_offset);
                        for i in 0..out_count {
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                                let r = ui
                                    .add(egui::Label::new(
                                        egui::RichText::new(format!("{} ", out_names[i].clone()))
                                            .font(font_id.clone())
                                            .line_height(Some(font_size_y)),
                                    ))
                                    .rect;
                                let pos = Pos2::new(r.right(), r.center().y);
                                let resp = ui.interact(
                                    Rect::from_center_size(pos, Vec2::splat(PIN_SIZE * 2.0)),
                                    ui.id().with(("output", i, idx)),
                                    Sense::click_and_drag(),
                                );
                                out_resps.push(resp);
                            });
                        }
                    });
                });
            })
            .response;

        println!("whole: {:?}", resp.rect.size());
        resp = resp.interact(Sense::click_and_drag());

        for i in 0..self.current_comp.connections.len() {
            let conn = &self.current_comp.connections[i];
            let conn_info = &mut self.current_comp.comp_conns[i];

            // If it is an output connection
            if conn.from.0 == idx {
                let points = &mut conn_info.points;
                points[0] = out_resps[conn.from.1].rect.center();
                points[1].y = points[0].y;
            }

            // If it is an input connection
            if conn.to.0 == idx {
                let points = &mut conn_info.points;
                let p_count = points.len();
                points[p_count - 1] = in_resps[conn.to.1].rect.center();
                points[p_count - 2].y = points[p_count - 1].y;
            }
        }

        // Update connections according to the current position
        (resp, in_resps, out_resps)
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
        let (resp, inputs, outputs) = self._draw_comp(ui, idx);

        // ui.painter().add(Shape::Vec(vec![Shape::rect_filled(
        //     s_rect,
        //     Rounding::same(4.0),
        //     Color32::from_rgb(50, 50, 50),
        // )]));

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
        // let mut resp = ui.put(
        //     s_rect,
        //     egui::Label::new(comp_name.clone()).selectable(false),
        // );
        // resp = resp.interact(Sense::click_and_drag());

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
