use eframe::egui;

#[derive(Debug, Clone, Copy)]
pub enum MenuAction {
    NewTab,
    Open,
    Save,
    SaveAs,
    CloseTab,
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
            if ui.button("Save\tCtrl+S").clicked() {
                action = Some(MenuAction::Save);
                ui.close_menu();
            }
            if ui.button("Save As...\tCtrl+Shift+S").clicked() {
                action = Some(MenuAction::SaveAs);
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
    });

    action
}
