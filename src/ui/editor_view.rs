use crate::document::Document;
use eframe::egui;

pub fn editor_view(ui: &mut egui::Ui, doc: &mut Document) {
    let response = ui.add(
        egui::TextEdit::multiline(doc.text_mut())
            .font(egui::TextStyle::Monospace)
            .desired_rows(24)
            .lock_focus(true)
            .desired_width(f32::INFINITY),
    );

    if response.changed() {
        doc.sync_rope();
    }
}
