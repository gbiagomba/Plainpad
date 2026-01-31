use crate::{
    editor::Editor,
    shortcuts::{detect, ShortcutCommand},
    ui,
};
use eframe::egui;
use rfd::FileDialog;

#[derive(Clone, Copy)]
enum AppCommand {
    NewTab,
    Open,
    Save,
    SaveAs,
    CloseTab,
    NextTab,
    PreviousTab,
    Quit,
}

impl From<ui::menu::MenuAction> for AppCommand {
    fn from(action: ui::menu::MenuAction) -> Self {
        match action {
            ui::menu::MenuAction::NewTab => Self::NewTab,
            ui::menu::MenuAction::Open => Self::Open,
            ui::menu::MenuAction::Save => Self::Save,
            ui::menu::MenuAction::SaveAs => Self::SaveAs,
            ui::menu::MenuAction::CloseTab => Self::CloseTab,
            ui::menu::MenuAction::Quit => Self::Quit,
        }
    }
}

impl From<ShortcutCommand> for AppCommand {
    fn from(command: ShortcutCommand) -> Self {
        match command {
            ShortcutCommand::NewTab => Self::NewTab,
            ShortcutCommand::Open => Self::Open,
            ShortcutCommand::Save => Self::Save,
            ShortcutCommand::SaveAs => Self::SaveAs,
            ShortcutCommand::CloseTab => Self::CloseTab,
            ShortcutCommand::NextTab => Self::NextTab,
            ShortcutCommand::PreviousTab => Self::PreviousTab,
        }
    }
}

pub struct PlainpadApp {
    editor: Editor,
    confirm_close: Option<usize>,
    error_message: Option<String>,
}

impl PlainpadApp {
    pub fn new() -> Self {
        Self {
            editor: Editor::new(),
            confirm_close: None,
            error_message: None,
        }
    }

    fn handle_command(&mut self, ctx: &egui::Context, command: AppCommand) {
        match command {
            AppCommand::NewTab => self.editor.new_document(),
            AppCommand::Open => self.open_file_dialog(),
            AppCommand::Save => self.save_current(),
            AppCommand::SaveAs => self.save_as_current(),
            AppCommand::CloseTab => {
                let index = self.editor.active_index();
                self.request_close(index);
            }
            AppCommand::NextTab => self.editor.next_tab(),
            AppCommand::PreviousTab => self.editor.previous_tab(),
            AppCommand::Quit => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
        }
    }

    fn request_close(&mut self, index: usize) {
        if let Some(doc) = self.editor.documents().get(index) {
            if doc.is_dirty() {
                self.confirm_close = Some(index);
                return;
            }
        }
        self.editor.close_document(index);
        self.confirm_close = None;
    }

    fn open_file_dialog(&mut self) {
        if let Some(path) = FileDialog::new().pick_file() {
            if let Err(err) = self.editor.open_document(path) {
                self.error_message = Some(format!("Failed to open file: {err}"));
            }
        }
    }

    fn save_current(&mut self) {
        let index = self.editor.active_index();
        let path = self.editor.current().and_then(|doc| doc.path().cloned());

        if let Some(path) = path {
            if let Err(err) = self.editor.save_document(index, path) {
                self.error_message = Some(format!("Failed to save file: {err}"));
            }
        } else {
            self.save_as_current();
        }
    }

    fn save_as_current(&mut self) {
        let index = self.editor.active_index();
        if let Some(path) = FileDialog::new().save_file() {
            if let Err(err) = self.editor.save_document(index, path) {
                self.error_message = Some(format!("Failed to save file: {err}"));
            }
        }
    }
}

impl eframe::App for PlainpadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut command: Option<AppCommand> = None;

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            if let Some(action) = ui::menu::menu_bar(ui) {
                command = Some(action.into());
            }
        });

        egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
            let action = ui::tabs::tab_bar(ui, self.editor.documents(), self.editor.active_index());
            if let Some(index) = action.select {
                self.editor.set_active(index);
            }
            if let Some(index) = action.close {
                self.request_close(index);
            }
        });

        if let Some(shortcut) = detect(ctx) {
            command = Some(shortcut.into());
        }

        if let Some(command) = command {
            self.handle_command(ctx, command);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(doc) = self.editor.current_mut() {
                ui::editor_view::editor_view(ui, doc);
            }
        });

        if let Some(index) = self.confirm_close {
            egui::Window::new("Unsaved Changes")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("This tab has unsaved changes.");
                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() {
                            let active_index = self.editor.active_index();
                            self.editor.set_active(index);
                            self.save_current();
                            self.editor.set_active(active_index);
                            if let Some(doc) = self.editor.documents().get(index) {
                                if !doc.is_dirty() {
                                    self.editor.close_document(index);
                                    self.confirm_close = None;
                                }
                            }
                        }
                        if ui.button("Discard").clicked() {
                            self.editor.close_document(index);
                            self.confirm_close = None;
                        }
                        if ui.button("Cancel").clicked() {
                            self.confirm_close = None;
                        }
                    });
                });
        }

        if let Some(message) = self.error_message.clone() {
            egui::Window::new("Error")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(message);
                    if ui.button("Dismiss").clicked() {
                        self.error_message = None;
                    }
                });
        }
    }
}
