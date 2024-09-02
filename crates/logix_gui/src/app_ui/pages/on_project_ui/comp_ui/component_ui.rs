use egui::{emath::TSTransform, Color32, Pos2, Rect, Response, Sense, Ui, Vec2};
use logix_sim::primitives::primitive::Primitive;

use crate::app_ui::{
    board_editing::BoardEditing,
    pages::on_project_ui::{
        constants::{COMP_FONT_SIZE, PIN_SIZE},
        wire_dir::WireDir,
    },
};

impl BoardEditing {
    pub fn update_comp_pos(&mut self, idx: usize, new_pos: Pos2) {
        self.board.components[idx].pos = new_pos;
    }

    pub const fn get_ghost_point(last_point: Pos2, dir: WireDir, cursor_pos: Pos2) -> Pos2 {
        match dir {
            WireDir::Horizontal => Pos2::new(cursor_pos.x, last_point.y),
            WireDir::Vertical => Pos2::new(last_point.x, cursor_pos.y),
        }
    }

    pub fn _draw_comp(
        &mut self,
        ui: &mut Ui,
        idx: usize,
    ) -> (Response, Vec<Response>, Vec<Response>) {
        let font_id = egui::FontId::monospace(COMP_FONT_SIZE);

        let in_count = self.current_sim_board().components[idx].input_count();
        let out_count = self.current_sim_board().components[idx].output_count();

        #[allow(clippy::cast_precision_loss)]
        let in_height = (in_count as f32) * COMP_FONT_SIZE;

        #[allow(clippy::cast_precision_loss)]
        let out_height = (out_count as f32) * COMP_FONT_SIZE;

        let pins_max_height = in_height.max(out_height);

        let in_offset: f32 = (pins_max_height - in_height) / 2.0;
        let out_offset: f32 = (pins_max_height - out_height) / 2.0;

        let height = pins_max_height;
        let name_offset: f32 = (height - COMP_FONT_SIZE) / 2.0;

        let mut in_resps = vec![];
        let mut out_resps = vec![];

        let board = self.current_sim_board_ref();

        let in_names = &board.components[idx].info.inputs_name;
        let out_names = &board.components[idx].info.outputs_name;

        let out_names_max_len = out_names
            .iter()
            .map(std::string::String::len)
            .max()
            .unwrap_or(0);
        let out_names = out_names
            .iter()
            .map(|x| format!("{x: >out_names_max_len$}"))
            .collect::<Vec<String>>();

        let mut resp = egui::Frame::default()
            .fill(Color32::from_rgb(70, 70, 70))
            .inner_margin(egui::Margin::symmetric(0.0, COMP_FONT_SIZE / 4.0))
            .rounding(4.0)
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    // Inputs
                    in_resps = draw_inputs(ui, in_offset, in_names, &font_id, idx);

                    // Name
                    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                        ui.add_space(name_offset);
                        let mut name = self.current_sim_board_ref().components[idx]
                            .info
                            .name
                            .clone();
                        if let Some(prim) =
                            &self.current_sim_board_ref().components[idx].info.primitive
                        {
                            match prim {
                                Primitive::Input { bits: _ } => {
                                    let in_order = board
                                        .inputs
                                        .iter()
                                        .position(|input| input.idx == idx)
                                        .unwrap();
                                    name.push_str(&format!(
                                        " {}",
                                        self.current_sim_board_ref().inputs[in_order].name
                                    ));
                                }
                                Primitive::Output { bits: _ } => {
                                    let out_order = board
                                        .outputs
                                        .iter()
                                        .position(|output| output.idx == idx)
                                        .unwrap();
                                    name.push_str(&format!(
                                        " {}",
                                        self.current_sim_board_ref().outputs[out_order].name
                                    ));
                                }
                                _ => {}
                            }
                        }
                        ui.add(egui::Label::new(
                            egui::RichText::new(name)
                                .font(font_id.clone())
                                .line_height(Some(COMP_FONT_SIZE))
                                .color(Color32::WHITE),
                        ));
                    });

                    // Outputs
                    out_resps = draw_ouputs(ui, out_offset, &out_names, &font_id, idx);
                });
            })
            .response;

        resp = resp.interact(Sense::click_and_drag());

        let len = self.current_sim_board().conns_info.len();
        for i in 0..len {
            // let conn = &self.current_sim_board().conns[i].clone();
            let info = &mut self.current_sim_board().conns_info[i];

            // If it is an output connection
            if info.conn.from.0 == idx {
                let points = &mut info.points;
                points[0] = out_resps[info.conn.from.1].rect.center();
                points[1].y = points[0].y;
            }

            // If it is an input connection
            if info.conn.to.0 == idx {
                let points = &mut info.points;
                let p_count = points.len();
                points[p_count - 1] = in_resps[info.conn.to.1].rect.center();
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
        // Handle dragging the component
        // -----------------------------------------------------------------------------
        if self.sim.is_none() && resp.dragged() && self.new_conn.is_none() {
            self.update_comp_pos(idx, self.board.components[idx].pos + resp.drag_delta());
        }

        // -----------------------------------------------------------------------------
        // Add the component's pins
        // -----------------------------------------------------------------------------
        self.draw_input_pins(ui, idx, &inputs);
        self.draw_output_pins(ui, idx, &outputs);

        // -----------------------------------------------------------------------------
        // Handle context menu for the component
        // -----------------------------------------------------------------------------
        if self.sim.is_none() && (resp.hovered() || resp.context_menu_opened()) {
            resp.context_menu(|ui| {
                ui.set_max_width(150.0);
                self.specific_comp_context_menu(ui, idx);
                if ui.button("Remove").clicked() {
                    self.board.remove_comp(idx);
                    ui.close_menu();
                }
            });
        }

        // -----------------------------------------------------------------------------
        // Handle clicking on the component
        // -----------------------------------------------------------------------------
        if resp.double_clicked()
            && self.sim.is_some()
            && self.current_sim_board().components[idx]
                .info
                .source
                .is_some()
        {
            self.enter_subc_sim(self.current_sim_board_ref().components[idx].info.id);
        }
    }
}

fn draw_ouputs(
    ui: &mut Ui,
    out_offset: f32,
    out_names: &[String],
    font_id: &egui::FontId,
    idx: usize,
) -> Vec<Response> {
    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
        let mut resps = vec![];
        ui.add_space(out_offset);
        for (i, name) in out_names.iter().enumerate() {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                let r = ui
                    .add(egui::Label::new(
                        egui::RichText::new(format!("{} ", name.clone()))
                            .font(font_id.clone())
                            .line_height(Some(COMP_FONT_SIZE)),
                    ))
                    .rect;
                let pos = Pos2::new(r.right(), r.center().y);
                let resp = ui.interact(
                    Rect::from_center_size(pos, Vec2::splat(PIN_SIZE * 2.0)),
                    ui.id().with(("output", i, idx)),
                    Sense::click_and_drag(),
                );
                resps.push(resp);
            });
        }
        resps
    })
    .inner
}

fn draw_inputs(
    ui: &mut Ui,
    in_offset: f32,
    in_names: &[String],
    font_id: &egui::FontId,
    idx: usize,
) -> Vec<Response> {
    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
        let mut resps = vec![];
        ui.add_space(in_offset);
        for (i, name) in in_names.iter().enumerate() {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                let r = ui
                    .add(egui::Label::new(
                        egui::RichText::new(format!(" {}", name.clone()))
                            .font(font_id.clone())
                            .line_height(Some(COMP_FONT_SIZE)),
                    ))
                    .rect;
                let pos = Pos2::new(r.left(), r.center().y);
                let resp = ui.interact(
                    Rect::from_center_size(pos, Vec2::splat(PIN_SIZE * 2.0)),
                    ui.id().with(("input", i, idx)),
                    Sense::click_and_drag(),
                );
                resps.push(resp);
            });
        }
        resps
    })
    .inner
}
