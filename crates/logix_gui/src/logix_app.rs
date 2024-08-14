use egui::{
    emath::TSTransform, CollapsingHeader, Color32, LayerId, Pos2, Rect, Rounding, Sense, Shape, Ui,
    Vec2,
};
use logix_core::component::{Component, SubComponent};
use rfd::FileDialog;
use std::path::PathBuf;

use crate::{comp_board::ComponentBoard, folder_tree::Folder};

const PIN_SIZE: f32 = 8.0;
const PIN_MARGIN: f32 = 15.0;

pub struct LogixApp {
    folder: Folder,
    selected_file: Option<PathBuf>,
    transform: TSTransform,
    current_comp: Option<ComponentBoard>,
}

impl Default for LogixApp {
    fn default() -> Self {
        let current_folder = std::env::current_dir().unwrap().display().to_string();
        Self {
            folder: Folder::from_str_path(&current_folder),
            selected_file: None,
            transform: TSTransform::default(),
            current_comp: Some(ComponentBoard::new(
                Component {
                    name: Some("My Component".to_string()),
                    id: 0,
                    inputs: 0,
                    outputs: 0,
                    sub: Some(SubComponent {
                        components: vec![Component {
                            name: Some("AND".to_string()),
                            id: 1,
                            inputs: 2,
                            outputs: 1,
                            sub: None,
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                vec![Pos2::new(10.0, 10.0)],
            )),
        }
    }
}

impl Folder {
    fn ui_impl(&mut self, ui: &mut Ui, selected_file: Option<&PathBuf>) -> Option<PathBuf> {
        let mut new_file = selected_file.cloned();
        for folder in self.folders.iter_mut() {
            let name = folder.current_path.file_name().unwrap().to_str().unwrap();
            CollapsingHeader::new(name).show(ui, |ui| {
                new_file = folder.ui_impl(ui, selected_file);
            });
        }

        for file in self.files.iter() {
            let name = file.file_name().unwrap().to_str().unwrap();
            // ui.label(name);
            let mut color = Color32::TRANSPARENT;
            if let Some(selected_file) = selected_file {
                if file == selected_file {
                    color = Color32::from_rgb(40, 40, 40);
                }
            }
            egui::Frame::default().fill(color).show(ui, |ui| {
                ui.allocate_space(Vec2::new(ui.available_width(), 0.0));
                let resp = ui.add(
                    egui::Label::new(name)
                        .selectable(false)
                        .wrap_mode(egui::TextWrapMode::Truncate)
                        .sense(Sense::click()),
                );
                if resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                if resp.double_clicked() {
                    new_file = Some(file.clone());
                }
            });
        }

        new_file
    }
}

impl LogixApp {
    fn file_menu(&mut self, ui: &mut Ui) {
        ui.set_max_width(200.0); // To make sure we wrap long text

        if ui.button("New Board").clicked() {
            ui.close_menu();
        }
        ui.separator();
        if ui.button("Open folder").clicked() {
            let new_folder = FileDialog::new().pick_folder();
            self.folder = Folder::from_pathbuf(&new_folder.unwrap());
            ui.close_menu();
        }
        ui.separator();
        if ui.button("Exit").clicked() {
            std::process::exit(0);
        }
    }

    fn top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| self.file_menu(ui));
            });
        });
    }

    fn left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("side_panel")
            .min_width(180.0)
            .show(ctx, |ui| {
                ui.heading(
                    self.folder
                        .current_path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap(),
                );
                egui::ScrollArea::vertical()
                    .max_width(180.0)
                    .show(ui, |ui| {
                        let new_file = self.folder.ui_impl(ui, self.selected_file.as_ref());
                        if new_file != self.selected_file {
                            // Load the new file
                            //
                            // This is where we would load the file and parse it.
                            //
                            self.selected_file = new_file;
                        }
                    });
            });
    }

    fn draw_canvas(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let (id, rect) = ui.allocate_space(ui.available_size());
            let response = ui.interact(rect, id, egui::Sense::click_and_drag());
            // Allow dragging the background as well.
            if response.dragged() {
                self.transform.translation += response.drag_delta();
            }

            // Plot-like reset
            if response.double_clicked() {
                self.transform = TSTransform::default();
            }

            let transform =
                TSTransform::from_translation(ui.min_rect().left_top().to_vec2()) * self.transform;

            if let Some(pointer) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                let pointer_in_layer = transform.inverse() * pointer;
                let zoom_delta = ui.ctx().input(|i| i.zoom_delta());

                // Zoom in on pointer:
                self.transform = self.transform
                    * TSTransform::from_translation(pointer_in_layer.to_vec2())
                    * TSTransform::from_scaling(zoom_delta)
                    * TSTransform::from_translation(-pointer_in_layer.to_vec2());

                if response.hovered() {
                    // Only pan if the mouse is over the background.
                    let pan_delta = ui.ctx().input(|i| i.smooth_scroll_delta);
                    self.transform = TSTransform::from_translation(pan_delta) * self.transform;
                }
            }

            let comp = self.current_comp.as_mut().unwrap();
            let sub = comp.component.sub.as_ref();

            if let Some(sub) = sub {
                let window_layer = ui.layer_id();
                for i in 0..sub.components.len() {
                    let new_id = LayerId::new(egui::Order::Middle, id.with(("sub", i)));
                    ui.with_layer_id(new_id, |ui| {
                        ui.set_clip_rect(transform.inverse() * rect);
                        self.draw_subc(ui, 0);

                        ui.ctx().set_transform_layer(new_id, transform);
                        ui.ctx().set_sublayer(window_layer, new_id);
                    });
                }
            }
        });
    }

    fn draw_subc(&mut self, ui: &mut Ui, idx: usize) {
        let comp = self.current_comp.as_mut().unwrap();
        let sub = comp
            .component
            .sub
            .as_ref()
            .unwrap()
            .components
            .get(idx)
            .unwrap();

        let in_height = (sub.inputs as f32) * (PIN_MARGIN) + PIN_MARGIN;
        let out_height = (sub.outputs as f32) * (PIN_MARGIN) + PIN_MARGIN;

        let height = in_height.max(out_height);

        let in_offset: f32 = (height - in_height) / 2.0;
        let out_offset: f32 = (height - out_height) / 2.0;

        let comp_size = Vec2::new(50.0, height);
        let local_pos = comp.subc_pos[idx];
        let s_rect = Rect::from_min_size(local_pos, comp_size);

        ui.painter().add(Shape::Vec(vec![Shape::rect_filled(
            s_rect,
            Rounding::same(4.0),
            Color32::from_rgb(50, 50, 50),
        )]));

        for i in 0..sub.inputs {
            let pin_pos = Pos2::new(
                local_pos.x,
                local_pos.y + in_offset + i as f32 * (PIN_MARGIN) + PIN_MARGIN,
            );
            ui.painter()
                .add(Shape::circle_filled(pin_pos, PIN_SIZE / 2.0, Color32::GRAY));
        }

        for i in 0..sub.outputs {
            let pin_pos = Pos2::new(
                local_pos.x + comp_size.x,
                local_pos.y + out_offset + i as f32 * (PIN_MARGIN) + PIN_MARGIN,
            );
            ui.painter()
                .add(Shape::circle_filled(pin_pos, PIN_SIZE / 2.0, Color32::GRAY));
        }

        let mut resp = ui.put(
            s_rect,
            egui::Label::new(sub.name.as_ref().unwrap().clone()).selectable(false),
        );

        resp = resp.interact(Sense::click_and_drag());
        if resp.dragged() {
            comp.subc_pos[idx] += resp.drag_delta();
        }
    }
}

impl eframe::App for LogixApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.top_panel(ctx);
        self.left_panel(ctx);
        self.draw_canvas(ctx);
    }
}
