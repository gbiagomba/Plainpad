use eframe::egui;

#[derive(Debug, Clone, Copy)]
pub enum MenuAction {
    NewTab,
    Open,
    Print,
    Save,
    SaveAs,
    SaveAll,
    CloseTab,
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    SelectAll,
    Quit,
}

pub fn menu_bar(ui: &mut egui::Ui) -> Option<MenuAction> {
    let mut action = None;

    egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("New Tab\tCtrl+N").clicked() {
                action = Some(MenuAction::NewTab);
                ui.close_menu();
            }
            if ui.button("Open...\tCtrl+O").clicked() {
                action = Some(MenuAction::Open);
                ui.close_menu();
            }
            if ui.button("Print...\tCtrl+P").clicked() {
                action = Some(MenuAction::Print);
                ui.close_menu();
            }
            if ui.button("Save\tCtrl+S").clicked() {
                action = Some(MenuAction::Save);
                ui.close_menu();
            }
            if ui.button("Save As...\tCtrl+Shift+S").clicked() {
                action = Some(MenuAction::SaveAs);
                ui.close_menu();
            }
            if ui.button("Save All").clicked() {
                action = Some(MenuAction::SaveAll);
                ui.close_menu();
            }
            ui.separator();
            if ui.button("Close Tab\tCtrl+W").clicked() {
                action = Some(MenuAction::CloseTab);
                ui.close_menu();
            }
            if ui.button("Quit").clicked() {
                action = Some(MenuAction::Quit);
                ui.close_menu();
            }
        });
        ui.menu_button("Edit", |ui| {
            if ui.button("Undo\tCtrl+Z").clicked() {
                action = Some(MenuAction::Undo);
                ui.close_menu();
            }
            if ui.button("Redo\tCtrl+Y").clicked() {
                action = Some(MenuAction::Redo);
                ui.close_menu();
            }
            ui.separator();
            if ui.button("Cut\tCtrl+X").clicked() {
                action = Some(MenuAction::Cut);
                ui.close_menu();
            }
            if ui.button("Copy\tCtrl+C").clicked() {
                action = Some(MenuAction::Copy);
                ui.close_menu();
            }
            if ui.button("Paste\tCtrl+V").clicked() {
                action = Some(MenuAction::Paste);
                ui.close_menu();
            }
            ui.separator();
            if ui.button("Select All\tCtrl+A").clicked() {
                action = Some(MenuAction::SelectAll);
                ui.close_menu();
            }
        });
    });

    action
}
