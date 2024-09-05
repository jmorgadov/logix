use std::{path::PathBuf, str::FromStr};

use egui::Ui;

use crate::app_ui::{board::BoardComponent, logix_app::LogixApp};

use super::canvas_payload::CanvasPayload;

const BIT_RANGE: [u8; 10] = [2, 3, 4, 5, 6, 7, 8, 16, 32, 64];

impl LogixApp {
    pub fn show_library(&mut self, ui: &mut Ui) {
        egui::CollapsingHeader::new("Gates").show(ui, |ui| {
            self.gates_library(ui);
        });
    }

    pub fn gates_library(&mut self, ui: &mut Ui) {
        let id = self.board_editing().next_id;
        Library::clock_entry(ui, id);
        Library::not_entry(ui, id);
        Library::const_high_entry(ui, id);
        Library::const_low_entry(ui, id);
        egui::CollapsingHeader::new("Inputs").show(ui, |ui| {
            BIT_RANGE
                .iter()
                .for_each(|i| Library::input_entry(ui, id, *i));
        });
        egui::CollapsingHeader::new("Outputs").show(ui, |ui| {
            BIT_RANGE
                .iter()
                .for_each(|i| Library::output_entry(ui, id, *i));
        });
        egui::CollapsingHeader::new("AND").show(ui, |ui| {
            BIT_RANGE
                .iter()
                .for_each(|i| Library::and_entry(ui, id, *i as usize));
        });
        egui::CollapsingHeader::new("OR").show(ui, |ui| {
            BIT_RANGE
                .iter()
                .for_each(|i| Library::or_entry(ui, id, *i as usize));
        });
        egui::CollapsingHeader::new("NAND").show(ui, |ui| {
            BIT_RANGE
                .iter()
                .for_each(|i| Library::nand_entry(ui, id, *i as usize));
        });
        egui::CollapsingHeader::new("NOR").show(ui, |ui| {
            BIT_RANGE
                .iter()
                .for_each(|i| Library::nor_entry(ui, id, *i as usize));
        });
        egui::CollapsingHeader::new("XOR").show(ui, |ui| {
            BIT_RANGE
                .iter()
                .for_each(|i| Library::xor_entry(ui, id, *i as usize));
        });
        egui::CollapsingHeader::new("Joiner").show(ui, |ui| {
            BIT_RANGE
                .iter()
                .for_each(|i| Library::joiner_entry(ui, id, *i));
        });
        egui::CollapsingHeader::new("Splitter").show(ui, |ui| {
            BIT_RANGE
                .iter()
                .for_each(|i| Library::splitter_entry(ui, id, *i));
        });
    }
}

struct Library;

const LIB_LOCAL_URI: &str = "assets/lib/";

impl Library {
    pub fn lib_uri() -> PathBuf {
        PathBuf::from_str(env!("CARGO_MANIFEST_DIR"))
            .unwrap()
            .join(LIB_LOCAL_URI)
    }

    pub fn and_entry(ui: &mut Ui, id: usize, inputs: usize) {
        ui.add(
            egui::Label::new(format!("AND {inputs}"))
                .selectable(false)
                .sense(egui::Sense::click_and_drag()),
        )
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .dnd_set_drag_payload(CanvasPayload::Component(BoardComponent::and_gate(
            id, inputs,
        )));
    }

    pub fn nand_entry(ui: &mut Ui, id: usize, inputs: usize) {
        ui.add(
            egui::Label::new(format!("NAND {inputs}"))
                .selectable(false)
                .sense(egui::Sense::click_and_drag()),
        )
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .dnd_set_drag_payload(CanvasPayload::Component(BoardComponent::nand_gate(
            id, inputs,
        )));
    }

    pub fn or_entry(ui: &mut Ui, id: usize, inputs: usize) {
        ui.add(
            egui::Label::new(format!("OR {inputs}"))
                .selectable(false)
                .sense(egui::Sense::click_and_drag()),
        )
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .dnd_set_drag_payload(CanvasPayload::Component(BoardComponent::or_gate(
            id, inputs,
        )));
    }

    pub fn nor_entry(ui: &mut Ui, id: usize, inputs: usize) {
        ui.add(
            egui::Label::new(format!("NOR {inputs}"))
                .selectable(false)
                .sense(egui::Sense::click_and_drag()),
        )
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .dnd_set_drag_payload(CanvasPayload::Component(BoardComponent::nor_gate(
            id, inputs,
        )));
    }

    pub fn xor_entry(ui: &mut Ui, id: usize, inputs: usize) {
        ui.add(
            egui::Label::new(format!("XOR {inputs}"))
                .selectable(false)
                .sense(egui::Sense::click_and_drag()),
        )
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .dnd_set_drag_payload(CanvasPayload::Component(BoardComponent::xor_gate(
            id, inputs,
        )));
    }

    pub fn joiner_entry(ui: &mut Ui, id: usize, inputs: u8) {
        ui.add(
            egui::Label::new(format!("Joiner {inputs}"))
                .selectable(false)
                .sense(egui::Sense::click_and_drag()),
        )
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .dnd_set_drag_payload(CanvasPayload::Component(BoardComponent::joiner(id, inputs)));
    }

    pub fn splitter_entry(ui: &mut Ui, id: usize, outputs: u8) {
        ui.add(
            egui::Label::new(format!("Splitter {outputs}"))
                .selectable(false)
                .sense(egui::Sense::click_and_drag()),
        )
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .dnd_set_drag_payload(CanvasPayload::Component(BoardComponent::splitter(
            id, outputs,
        )));
    }

    pub fn not_entry(ui: &mut Ui, id: usize) {
        ui.add(
            egui::Label::new("NOT")
                .selectable(false)
                .sense(egui::Sense::click_and_drag()),
        )
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .dnd_set_drag_payload(CanvasPayload::Component(BoardComponent::not_gate(id)));
    }

    pub fn clock_entry(ui: &mut Ui, id: usize) {
        ui.add(
            egui::Label::new("Clock")
                .selectable(false)
                .sense(egui::Sense::click_and_drag()),
        )
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .dnd_set_drag_payload(CanvasPayload::Component(BoardComponent::clock_gate(id)));
    }

    pub fn const_high_entry(ui: &mut Ui, id: usize) {
        ui.add(
            egui::Label::new("HIGH")
                .selectable(false)
                .sense(egui::Sense::click_and_drag()),
        )
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .dnd_set_drag_payload(CanvasPayload::Component(BoardComponent::const_high_gate(
            id,
        )));
    }

    pub fn const_low_entry(ui: &mut Ui, id: usize) {
        ui.add(
            egui::Label::new("LOW")
                .selectable(false)
                .sense(egui::Sense::click_and_drag()),
        )
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .dnd_set_drag_payload(CanvasPayload::Component(BoardComponent::const_low_gate(id)));
    }

    pub fn input_entry(ui: &mut Ui, id: usize, bits: u8) {
        ui.add(
            egui::Label::new(format!("Input {bits}"))
                .selectable(false)
                .sense(egui::Sense::click_and_drag()),
        )
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .dnd_set_drag_payload(CanvasPayload::Component(BoardComponent::input(id, bits)));
    }

    pub fn output_entry(ui: &mut Ui, id: usize, bits: u8) {
        ui.add(
            egui::Label::new(format!("Output {bits}"))
                .selectable(false)
                .sense(egui::Sense::click_and_drag()),
        )
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .dnd_set_drag_payload(CanvasPayload::Component(BoardComponent::output(id, bits)));
    }
}
