use egui::{Color32, Sense, Stroke};

use crate::app_ui::logix_app::LogixApp;

impl LogixApp {
    pub fn draw_tabs(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("tabs")
            .frame(
                egui::Frame::default()
                    .inner_margin(egui::Margin::same(0.0))
                    .fill(ctx.style().visuals.panel_fill),
            )
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    let mut i = 0;
                    let mut next_current_tab = self.current_tab;
                    while i < self.board_tabs.len() {
                        let mut removed = false;
                        let color = if i == self.current_tab {
                            Color32::from_gray(35)
                        } else {
                            ui.style().visuals.panel_fill
                        };
                        egui::Frame::default().fill(color).show(ui, |ui| {
                            ui.allocate_space(egui::vec2(0.0, ui.available_height()));
                            ui.horizontal(|ui| {
                                ui.set_max_width(150.0);
                                let tab_label = if self.board_tabs[i].board.name.is_empty() {
                                    "Untitled"
                                } else {
                                    self.board_tabs[i].board.name.as_str()
                                };
                                let resp = ui
                                    .add(egui::Label::new(tab_label).truncate().selectable(false))
                                    .interact(Sense::click());

                                if resp.clicked() {
                                    next_current_tab = i;
                                }
                            });
                            if ui
                                .add(
                                    egui::Button::new("❌")
                                        .stroke(Stroke::new(0.0, Color32::TRANSPARENT))
                                        .fill(Color32::TRANSPARENT),
                                )
                                .clicked()
                            {
                                self.board_tabs.remove(i);
                                removed = true;

                                if i < self.current_tab
                                    || (i == self.current_tab
                                        && i == self.board_tabs.len()
                                        && i > 0)
                                {
                                    next_current_tab -= 1;
                                }
                            }
                        });

                        if !removed {
                            i += 1;
                        }
                    }
                    if self.board_tabs.is_empty() {
                        self.selected_file = None;
                    } else {
                        self.set_current_tab(next_current_tab);
                    }
                });
            });
    }
}
