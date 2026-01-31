use eframe::egui;

#[derive(Debug, Clone, Copy)]
pub enum ShortcutCommand {
    NewTab,
    Open,
    Print,
    Save,
    SaveAs,
    CloseTab,
    NextTab,
    PreviousTab,
    SelectTab(usize),
    SelectLastTab,
    Find,
    Replace,
    Quit,
}

pub fn detect(ctx: &egui::Context) -> Option<ShortcutCommand> {
    let input = ctx.input(|input| input.clone());

    if input.modifiers.command && input.modifiers.shift && input.key_pressed(egui::Key::S) {
        return Some(ShortcutCommand::SaveAs);
    }

    if input.modifiers.command && input.key_pressed(egui::Key::S) {
        return Some(ShortcutCommand::Save);
    }

    if input.modifiers.command && input.key_pressed(egui::Key::N) {
        return Some(ShortcutCommand::NewTab);
    }

    if input.modifiers.command && input.key_pressed(egui::Key::O) {
        return Some(ShortcutCommand::Open);
    }

    if input.modifiers.command && input.key_pressed(egui::Key::P) {
        return Some(ShortcutCommand::Print);
    }

    if input.modifiers.command && input.modifiers.shift && input.key_pressed(egui::Key::W) {
        return Some(ShortcutCommand::Quit);
    }

    if input.modifiers.command && input.key_pressed(egui::Key::W) {
        return Some(ShortcutCommand::CloseTab);
    }

    if input.modifiers.command && input.key_pressed(egui::Key::Tab) {
        if input.modifiers.shift {
            return Some(ShortcutCommand::PreviousTab);
        }
        return Some(ShortcutCommand::NextTab);
    }

    if input.modifiers.command && input.key_pressed(egui::Key::F) {
        return Some(ShortcutCommand::Find);
    }

    if input.modifiers.command && input.key_pressed(egui::Key::H) {
        return Some(ShortcutCommand::Replace);
    }

    if input.modifiers.command {
        let index = match () {
            _ if input.key_pressed(egui::Key::Num1) => Some(0),
            _ if input.key_pressed(egui::Key::Num2) => Some(1),
            _ if input.key_pressed(egui::Key::Num3) => Some(2),
            _ if input.key_pressed(egui::Key::Num4) => Some(3),
            _ if input.key_pressed(egui::Key::Num5) => Some(4),
            _ if input.key_pressed(egui::Key::Num6) => Some(5),
            _ if input.key_pressed(egui::Key::Num7) => Some(6),
            _ if input.key_pressed(egui::Key::Num8) => Some(7),
            _ if input.key_pressed(egui::Key::Num9) => None,
            _ => None,
        };

        if input.key_pressed(egui::Key::Num9) {
            return Some(ShortcutCommand::SelectLastTab);
        }

        if let Some(index) = index {
            return Some(ShortcutCommand::SelectTab(index));
        }
    }

    None
}
