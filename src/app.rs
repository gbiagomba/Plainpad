use crate::{
    document::Document,
    editor::Editor,
    shortcuts::{detect, ShortcutCommand},
    ui,
};
use arboard::Clipboard;
use eframe::egui;
use eframe::egui::text::{CCursor, CCursorRange};
use regex::Regex;
use rfd::FileDialog;
use std::ops::Range;

#[derive(Clone, Copy)]
enum AppCommand {
    NewTab,
    Open,
    Print,
    Save,
    SaveAs,
    SaveAll,
    CloseAll,
    CloseTab,
    CloseTabsLeft,
    CloseTabsRight,
    NextTab,
    PreviousTab,
    SelectTab(usize),
    SelectLastTab,
    Undo,
    Redo,
    Find,
    Replace,
    Cut,
    Copy,
    Paste,
    SelectAll,
    ToggleStatusBar(bool),
    ToggleLineNumbers(bool),
    Quit,
    ForceQuit,
}

impl From<ui::menu::MenuAction> for AppCommand {
    fn from(action: ui::menu::MenuAction) -> Self {
        match action {
            ui::menu::MenuAction::NewTab => Self::NewTab,
            ui::menu::MenuAction::Open => Self::Open,
            ui::menu::MenuAction::Print => Self::Print,
            ui::menu::MenuAction::Save => Self::Save,
            ui::menu::MenuAction::SaveAs => Self::SaveAs,
            ui::menu::MenuAction::SaveAll => Self::SaveAll,
            ui::menu::MenuAction::CloseAll => Self::CloseAll,
            ui::menu::MenuAction::CloseTab => Self::CloseTab,
            ui::menu::MenuAction::CloseTabsLeft => Self::CloseTabsLeft,
            ui::menu::MenuAction::CloseTabsRight => Self::CloseTabsRight,
            ui::menu::MenuAction::Undo => Self::Undo,
            ui::menu::MenuAction::Redo => Self::Redo,
            ui::menu::MenuAction::Find => Self::Find,
            ui::menu::MenuAction::Replace => Self::Replace,
            ui::menu::MenuAction::Cut => Self::Cut,
            ui::menu::MenuAction::Copy => Self::Copy,
            ui::menu::MenuAction::Paste => Self::Paste,
            ui::menu::MenuAction::SelectAll => Self::SelectAll,
            ui::menu::MenuAction::ToggleStatusBar(enabled) => Self::ToggleStatusBar(enabled),
            ui::menu::MenuAction::ToggleLineNumbers(enabled) => Self::ToggleLineNumbers(enabled),
            ui::menu::MenuAction::Quit => Self::Quit,
            ui::menu::MenuAction::ForceQuit => Self::ForceQuit,
        }
    }
}

impl From<ShortcutCommand> for AppCommand {
    fn from(command: ShortcutCommand) -> Self {
        match command {
            ShortcutCommand::NewTab => Self::NewTab,
            ShortcutCommand::Open => Self::Open,
            ShortcutCommand::Print => Self::Print,
            ShortcutCommand::Save => Self::Save,
            ShortcutCommand::SaveAs => Self::SaveAs,
            ShortcutCommand::CloseTab => Self::CloseTab,
            ShortcutCommand::NextTab => Self::NextTab,
            ShortcutCommand::PreviousTab => Self::PreviousTab,
            ShortcutCommand::SelectTab(index) => Self::SelectTab(index),
            ShortcutCommand::SelectLastTab => Self::SelectLastTab,
            ShortcutCommand::Find => Self::Find,
            ShortcutCommand::Replace => Self::Replace,
            ShortcutCommand::Quit => Self::Quit,
        }
    }
}

#[derive(Default)]
struct FindPanel {
    open: bool,
    query: String,
    replace: String,
    use_regex: bool,
    error: Option<String>,
}

pub struct PlainpadApp {
    editor: Editor,
    confirm_close: Option<usize>,
    pending_close: Vec<usize>,
    confirm_quit: bool,
    allow_quit: bool,
    error_message: Option<String>,
    editor_focused: bool,
    editor_id: Option<egui::Id>,
    show_status_bar: bool,
    show_line_numbers: bool,
    find_panel: FindPanel,
}

impl PlainpadApp {
    pub fn new() -> Self {
        Self {
            editor: Editor::new(),
            confirm_close: None,
            pending_close: Vec::new(),
            confirm_quit: false,
            allow_quit: false,
            error_message: None,
            editor_focused: false,
            editor_id: None,
            show_status_bar: true,
            show_line_numbers: false,
            find_panel: FindPanel::default(),
        }
    }

    fn handle_command(&mut self, ctx: &egui::Context, command: AppCommand) {
        match command {
            AppCommand::NewTab => self.editor.new_document(),
            AppCommand::Open => self.open_file_dialog(),
            AppCommand::Print => self.show_print_notice(),
            AppCommand::Save => self.save_current(),
            AppCommand::SaveAs => self.save_as_current(),
            AppCommand::SaveAll => self.save_all_non_empty(),
            AppCommand::CloseAll => self.close_all_tabs(),
            AppCommand::CloseTab => {
                let index = self.editor.active_index();
                self.request_close(index);
            }
            AppCommand::CloseTabsLeft => self.close_tabs_left_of_active(),
            AppCommand::CloseTabsRight => self.close_tabs_right_of_active(),
            AppCommand::NextTab => self.editor.next_tab(),
            AppCommand::PreviousTab => self.editor.previous_tab(),
            AppCommand::SelectTab(index) => self.editor.set_active(index),
            AppCommand::SelectLastTab => self.select_last_tab(),
            AppCommand::Undo => self.send_edit_key(ctx, egui::Key::Z, false),
            AppCommand::Redo => self.send_edit_key(ctx, egui::Key::Y, false),
            AppCommand::Find => self.open_find_panel(),
            AppCommand::Replace => self.open_replace_panel(),
            AppCommand::Cut => self.send_edit_event(ctx, egui::Event::Cut),
            AppCommand::Copy => self.send_edit_event(ctx, egui::Event::Copy),
            AppCommand::Paste => self.paste_from_clipboard(ctx),
            AppCommand::SelectAll => self.send_edit_key(ctx, egui::Key::A, false),
            AppCommand::ToggleStatusBar(enabled) => self.show_status_bar = enabled,
            AppCommand::ToggleLineNumbers(enabled) => self.show_line_numbers = enabled,
            AppCommand::Quit => self.request_quit(ctx),
            AppCommand::ForceQuit => self.force_quit(ctx),
        }
    }

    fn send_edit_key(&self, ctx: &egui::Context, key: egui::Key, shift: bool) {
        let Some(editor_id) = self.editor_id else {
            return;
        };

        ctx.memory_mut(|memory| memory.request_focus(editor_id));

        let modifiers = egui::Modifiers {
            command: true,
            shift,
            ..Default::default()
        };

        ctx.input_mut(|input| {
            input.events.push(egui::Event::Key {
                key,
                physical_key: None,
                pressed: true,
                modifiers,
                repeat: false,
            });
            input.events.push(egui::Event::Key {
                key,
                physical_key: None,
                pressed: false,
                modifiers,
                repeat: false,
            });
        });
    }

    fn send_edit_event(&self, ctx: &egui::Context, event: egui::Event) {
        let Some(editor_id) = self.editor_id else {
            return;
        };

        ctx.memory_mut(|memory| memory.request_focus(editor_id));

        ctx.input_mut(|input| input.events.push(event));
    }

    fn paste_from_clipboard(&mut self, ctx: &egui::Context) {
        let Some(editor_id) = self.editor_id else {
            return;
        };

        ctx.memory_mut(|memory| memory.request_focus(editor_id));

        let clipboard_text = Clipboard::new()
            .and_then(|mut clipboard| clipboard.get_text())
            .ok();

        if let Some(text) = clipboard_text {
            ctx.input_mut(|input| input.events.push(egui::Event::Paste(text)));
        } else {
            self.error_message = Some("Failed to read clipboard contents.".to_string());
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
        self.close_next_pending();
    }

    fn queue_close_tabs(&mut self, mut indices: Vec<usize>) {
        if indices.is_empty() {
            return;
        }
        indices.sort_unstable_by(|a, b| b.cmp(a));
        self.pending_close = indices;
        self.close_next_pending();
    }

    fn close_next_pending(&mut self) {
        if self.confirm_close.is_some() {
            return;
        }
        if let Some(index) = self.pending_close.pop() {
            self.request_close(index);
        }
    }

    fn close_tabs_left_of_active(&mut self) {
        let active = self.editor.active_index();
        if active == 0 {
            return;
        }
        let indices = (0..active).collect::<Vec<_>>();
        self.queue_close_tabs(indices);
    }

    fn close_tabs_right_of_active(&mut self) {
        let total = self.editor.documents().len();
        let active = self.editor.active_index();
        if active + 1 >= total {
            return;
        }
        let indices = (active + 1..total).collect::<Vec<_>>();
        self.queue_close_tabs(indices);
    }

    fn close_all_tabs(&mut self) {
        let total = self.editor.documents().len();
        if total == 0 {
            return;
        }
        let indices = (0..total).collect::<Vec<_>>();
        self.queue_close_tabs(indices);
    }

    fn select_last_tab(&mut self) {
        let total = self.editor.documents().len();
        if total == 0 {
            return;
        }
        self.editor.set_active(total.saturating_sub(1));
    }

    fn open_find_panel(&mut self) {
        self.find_panel.open = true;
        self.find_panel.error = None;
    }

    fn open_replace_panel(&mut self) {
        self.find_panel.open = true;
        self.find_panel.error = None;
    }

    fn selection_char_range(&self, ctx: &egui::Context) -> Option<Range<usize>> {
        let editor_id = self.editor_id?;
        let state = egui::text_edit::TextEditState::load(ctx, editor_id)?;
        let range = state.cursor.char_range()?;
        let [start, end] = range.sorted();
        Some(start.index..end.index)
    }

    fn select_char_range(&self, ctx: &egui::Context, range: Range<usize>) {
        let Some(editor_id) = self.editor_id else {
            return;
        };
        if let Some(mut state) = egui::text_edit::TextEditState::load(ctx, editor_id) {
            let selection = CCursorRange::two(CCursor::new(range.start), CCursor::new(range.end));
            state.cursor.set_char_range(Some(selection));
            state.store(ctx, editor_id);
            ctx.memory_mut(|memory| memory.request_focus(editor_id));
        }
    }

    fn find_next(&mut self, ctx: &egui::Context) {
        let Some(doc) = self.editor.current() else {
            return;
        };
        let query = self.find_panel.query.clone();
        if query.is_empty() {
            self.find_panel.error = Some("Enter search text to find matches.".to_string());
            return;
        }
        let text = doc.text();
        let selection_end = self
            .selection_char_range(ctx)
            .map(|range| range.end)
            .unwrap_or(0);
        let start_byte = byte_index_from_char(text, selection_end);
        let match_range = match self.find_match_range(text, &query, start_byte) {
            Ok(Some(range)) => Some(range),
            Ok(None) => self.find_match_range(text, &query, 0).ok().flatten(),
            Err(error) => {
                self.find_panel.error = Some(error);
                return;
            }
        };

        if let Some(range) = match_range {
            self.select_char_range(ctx, range);
            self.find_panel.error = None;
        } else {
            self.find_panel.error = Some("No matches found.".to_string());
        }
    }

    fn replace_current(&mut self, ctx: &egui::Context) {
        let query = self.find_panel.query.clone();
        if query.is_empty() {
            self.find_panel.error = Some("Enter search text to replace.".to_string());
            return;
        }
        if self.try_replace_selection(ctx, &query) {
            if self.find_panel.error.is_none() {
                self.find_next(ctx);
            }
            return;
        }

        self.find_next(ctx);

        if self.try_replace_selection(ctx, &query) && self.find_panel.error.is_none() {
            self.find_next(ctx);
        }
    }

    fn replace_all(&mut self) {
        let Some(doc) = self.editor.current_mut() else {
            return;
        };
        let query = self.find_panel.query.clone();
        if query.is_empty() {
            self.find_panel.error = Some("Enter search text to replace.".to_string());
            return;
        }
        let use_regex = self.find_panel.use_regex;
        let replacement = self.find_panel.replace.clone();
        let replaced = match replace_all_matches(use_regex, doc.text(), &query, &replacement) {
            Ok(text) => text,
            Err(error) => {
                self.find_panel.error = Some(error);
                return;
            }
        };

        if replaced != doc.text() {
            *doc.text_mut() = replaced;
            doc.sync_rope();
        }
        self.find_panel.error = None;
    }

    fn try_replace_selection(&mut self, ctx: &egui::Context, query: &str) -> bool {
        let selection = match self.selection_char_range(ctx) {
            Some(range) if range.start != range.end => range,
            _ => return false,
        };
        let Some(doc_text) = self.editor.current().map(Document::text) else {
            return false;
        };
        let selection_text = slice_char_range(doc_text, &selection);
        let replacement = match self.replace_match(query, selection_text) {
            Ok(Some(replacement)) => replacement,
            Ok(None) => return false,
            Err(error) => {
                self.find_panel.error = Some(error);
                return true;
            }
        };

        if let Some(doc) = self.editor.current_mut() {
            let start_byte = byte_index_from_char(doc.text(), selection.start);
            let end_byte = byte_index_from_char(doc.text(), selection.end);
            doc.text_mut()
                .replace_range(start_byte..end_byte, &replacement);
            doc.sync_rope();
        }

        let end_char = selection.start + replacement.chars().count();
        self.select_char_range(ctx, selection.start..end_char);
        self.find_panel.error = None;
        true
    }

    fn find_match_range(
        &self,
        text: &str,
        query: &str,
        start_byte: usize,
    ) -> Result<Option<Range<usize>>, String> {
        if self.find_panel.use_regex {
            let regex = Regex::new(query).map_err(|err| err.to_string())?;
            if let Some(found) = regex.find_at(text, start_byte) {
                return Ok(Some(byte_range_to_char_range(text, found.range())));
            }
            return Ok(None);
        }

        let found = text[start_byte..].find(query);
        Ok(found.map(|offset| {
            let start = start_byte + offset;
            let end = start + query.len();
            byte_range_to_char_range(text, start..end)
        }))
    }

    fn replace_match(&self, query: &str, selection: &str) -> Result<Option<String>, String> {
        if self.find_panel.use_regex {
            let regex = Regex::new(query).map_err(|err| err.to_string())?;
            if let Some(found) = regex.find(selection) {
                if found.start() == 0 && found.end() == selection.len() {
                    return Ok(Some(
                        regex
                            .replace(selection, self.find_panel.replace.as_str())
                            .into(),
                    ));
                }
            }
            return Ok(None);
        }

        if selection == query {
            return Ok(Some(self.find_panel.replace.clone()));
        }

        Ok(None)
    }

    fn request_quit(&mut self, ctx: &egui::Context) {
        self.allow_quit = false;
        if self.has_dirty_documents() {
            self.confirm_quit = true;
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    fn force_quit(&mut self, ctx: &egui::Context) {
        self.confirm_quit = false;
        self.confirm_close = None;
        self.pending_close.clear();
        self.allow_quit = true;
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }

    fn handle_close_request(&mut self, ctx: &egui::Context) {
        if self.allow_quit {
            return;
        }
        if self.confirm_quit {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            return;
        }
        if self.has_dirty_documents() {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.confirm_quit = true;
        }
    }

    fn has_dirty_documents(&self) -> bool {
        self.editor.documents().iter().any(Document::is_dirty)
    }

    fn save_all_dirty(&mut self) -> bool {
        let total = self.editor.documents().len();
        for index in 0..total {
            let (dirty, path) = {
                let doc = &self.editor.documents()[index];
                (doc.is_dirty(), doc.path().cloned())
            };

            if !dirty {
                continue;
            }

            let path = match path {
                Some(path) => Some(path),
                None => self.save_dialog_for(index).save_file(),
            };

            let Some(path) = path else {
                return false;
            };

            if let Err(err) = self.editor.save_document(index, path) {
                self.error_message = Some(format!("Failed to save file: {err}"));
                return false;
            }
        }

        true
    }

    fn open_file_dialog(&mut self) {
        if let Some(path) = FileDialog::new().pick_file() {
            if let Err(err) = self.editor.open_document(path) {
                self.error_message = Some(format!("Failed to open file: {err}"));
            }
        }
    }

    fn show_print_notice(&mut self) {
        self.error_message = Some("Printing is not available yet.".to_string());
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
        let dialog = self.save_dialog_for(index);
        if let Some(path) = dialog.save_file() {
            if let Err(err) = self.editor.save_document(index, path) {
                self.error_message = Some(format!("Failed to save file: {err}"));
            }
        }
    }

    fn save_all_non_empty(&mut self) {
        let total = self.editor.documents().len();
        for index in 0..total {
            let (is_empty, path) = {
                let doc = &self.editor.documents()[index];
                (doc.is_empty(), doc.path().cloned())
            };

            if is_empty {
                continue;
            }

            let path = match path {
                Some(path) => Some(path),
                None => self.save_dialog_for(index).save_file(),
            };

            if let Some(path) = path {
                if let Err(err) = self.editor.save_document(index, path) {
                    self.error_message = Some(format!("Failed to save file: {err}"));
                    return;
                }
            }
        }
    }

    fn save_dialog_for(&self, index: usize) -> FileDialog {
        let name = self
            .editor
            .documents()
            .get(index)
            .map(Document::title)
            .unwrap_or_else(|| "Untitled".to_string());

        let file_name = if name.to_lowercase().ends_with(".txt") {
            name
        } else {
            format!("{name}.txt")
        };

        FileDialog::new()
            .add_filter("Text", &["txt"])
            .set_file_name(file_name)
    }
}

fn slice_char_range<'a>(text: &'a str, range: &Range<usize>) -> &'a str {
    let start_byte = byte_index_from_char(text, range.start);
    let end_byte = byte_index_from_char(text, range.end);
    &text[start_byte..end_byte]
}

fn byte_range_to_char_range(text: &str, range: Range<usize>) -> Range<usize> {
    let start = text[..range.start].chars().count();
    let end = text[..range.end].chars().count();
    start..end
}

fn byte_index_from_char(text: &str, char_index: usize) -> usize {
    text.char_indices()
        .nth(char_index)
        .map(|(index, _)| index)
        .unwrap_or_else(|| text.len())
}

fn replace_all_matches(
    use_regex: bool,
    text: &str,
    query: &str,
    replacement: &str,
) -> Result<String, String> {
    if use_regex {
        let regex = Regex::new(query).map_err(|err| err.to_string())?;
        return Ok(regex.replace_all(text, replacement).into());
    }

    Ok(text.replace(query, replacement))
}

fn count_matches(use_regex: bool, text: &str, query: &str) -> Result<usize, String> {
    if use_regex {
        let regex = Regex::new(query).map_err(|err| err.to_string())?;
        return Ok(regex.find_iter(text).count());
    }

    if query.is_empty() {
        return Ok(0);
    }

    Ok(text.match_indices(query).count())
}

impl eframe::App for PlainpadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut command: Option<AppCommand> = None;

        if ctx.input(|input| input.viewport().close_requested()) {
            self.handle_close_request(ctx);
        }

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            if let Some(action) =
                ui::menu::menu_bar(ui, self.show_status_bar, self.show_line_numbers)
            {
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
            if action.new_tab {
                self.editor.new_document();
            }
        });

        if let Some(shortcut) = detect(ctx) {
            command = Some(shortcut.into());
        }

        if let Some(command) = command {
            self.handle_command(ctx, command);
        }

        self.editor_focused = false;
        self.editor_id = None;

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(doc) = self.editor.current_mut() {
                let response = ui::editor_view::editor_view(ui, doc, self.show_line_numbers);
                self.editor_focused = response.has_focus();
                self.editor_id = Some(response.id);
            }
        });

        if self.show_status_bar {
            egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
                if let Some(doc) = self.editor.current() {
                    let text = doc.text();
                    let word_count = text.split_whitespace().count();
                    let char_count = text.chars().count();
                    let byte_count = text.len();
                    let line_count = text.lines().count().max(1);
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "Words: {word_count} | Chars: {char_count} | Bytes: {byte_count} | Lines: {line_count}"
                        ));
                    });
                }
            });
        }

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
                                    self.close_next_pending();
                                }
                            }
                        }
                        if ui.button("Discard").clicked() {
                            self.editor.close_document(index);
                            self.confirm_close = None;
                            self.close_next_pending();
                        }
                        if ui.button("Cancel").clicked() {
                            self.confirm_close = None;
                            self.pending_close.clear();
                        }
                    });
                });
        }

        if self.confirm_quit && self.confirm_close.is_none() {
            egui::Window::new("Quit Plainpad")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("You have unsaved changes.");
                    ui.horizontal(|ui| {
                        if ui.button("Save All").clicked() && self.save_all_dirty() {
                            self.confirm_quit = false;
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        if ui.button("Discard").clicked() {
                            self.confirm_quit = false;
                            self.allow_quit = true;
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        if ui.button("Force Quit").clicked() {
                            self.force_quit(ctx);
                        }
                        if ui.button("Cancel").clicked() {
                            self.confirm_quit = false;
                        }
                    });
                });
        }

        if self.find_panel.open {
            let mut open = self.find_panel.open;
            egui::Window::new("Find & Replace")
                .open(&mut open)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Find:");
                    ui.text_edit_singleline(&mut self.find_panel.query);
                    ui.label("Replace:");
                    ui.text_edit_singleline(&mut self.find_panel.replace);
                    ui.checkbox(&mut self.find_panel.use_regex, "Use regex");

                    if let Some(doc) = self.editor.current() {
                        if !self.find_panel.query.is_empty() {
                            match count_matches(
                                self.find_panel.use_regex,
                                doc.text(),
                                &self.find_panel.query,
                            ) {
                                Ok(count) => {
                                    self.find_panel.error = None;
                                    ui.label(format!("Matches: {count}"));
                                }
                                Err(error) => {
                                    self.find_panel.error = Some(error);
                                }
                            }
                        }
                    }

                    if let Some(error) = self.find_panel.error.clone() {
                        ui.label(error);
                    }

                    ui.horizontal(|ui| {
                        if ui.button("Find Next").clicked() {
                            self.find_next(ctx);
                        }
                        if ui.button("Replace").clicked() {
                            self.replace_current(ctx);
                        }
                        if ui.button("Replace All").clicked() {
                            self.replace_all();
                        }
                    });
                });
            self.find_panel.open = open;
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
