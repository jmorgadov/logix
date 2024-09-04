use egui::{Key, KeyboardShortcut, Modifiers};

pub const NEW_BOARD: KeyboardShortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::N);
pub const SAVE: KeyboardShortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::S);
pub const SAVE_AS: KeyboardShortcut =
    KeyboardShortcut::new(Modifiers::CTRL.plus(Modifiers::SHIFT), Key::S);

pub const RUN_SIM: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F5);
pub const PAUSE_RESUME_SIM: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F6);
pub const STOP_SIM: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F8);

pub const UNDO: KeyboardShortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::Z);
pub const REDO: KeyboardShortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::Y);

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
