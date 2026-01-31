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

    if input.modifiers.command && input.key_pressed(egui::Key::W) {
        return Some(ShortcutCommand::CloseTab);
    }

    if input.modifiers.command && input.key_pressed(egui::Key::Tab) {
        if input.modifiers.shift {
            return Some(ShortcutCommand::PreviousTab);
        }
        return Some(ShortcutCommand::NextTab);
    }

    None
}
