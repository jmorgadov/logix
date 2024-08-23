use egui::{Key, KeyboardShortcut, Modifiers};

pub const SAVE: KeyboardShortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::S);
pub const RUN: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F5);
pub const STOP: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F6);

pub fn shortcut_string(shortcut: KeyboardShortcut) -> String {
    let mut res = String::new();
    if shortcut.modifiers.ctrl {
        res.push_str("Ctrl + ");
    }
    if shortcut.modifiers.shift {
        res.push_str("Shift + ");
    }
    if shortcut.modifiers.alt {
        res.push_str("Alt + ");
    }
    res.push_str(&format!("{:?}", shortcut.logical_key));
    res
}
