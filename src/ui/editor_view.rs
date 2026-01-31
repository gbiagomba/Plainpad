use crate::document::Document;
use eframe::egui;

pub fn editor_view(
    ui: &mut egui::Ui,
    doc: &mut Document,
    show_line_numbers: bool,
) -> egui::Response {
    let response = if show_line_numbers {
        let line_count = doc.text().lines().count().max(1);
        let digits = line_count.to_string().len();
        let gutter_width = 12.0 + (digits as f32 * 8.0);
        let line_numbers = (1..=line_count)
            .map(|line| line.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        let mut editor_response = None;

        ui.horizontal(|ui| {
            ui.add_sized(
                [gutter_width, ui.available_height()],
                egui::Label::new(egui::RichText::new(line_numbers).monospace()).wrap(false),
            );
            editor_response = Some(
                ui.add(
                    egui::TextEdit::multiline(doc.text_mut())
                        .id_source("plainpad_editor")
                        .font(egui::TextStyle::Monospace)
                        .desired_rows(24)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY),
                ),
            );
        });

        editor_response
            .unwrap_or_else(|| ui.allocate_response(ui.available_size(), egui::Sense::hover()))
    } else {
        ui.add(
            egui::TextEdit::multiline(doc.text_mut())
                .id_source("plainpad_editor")
                .font(egui::TextStyle::Monospace)
                .desired_rows(24)
                .lock_focus(true)
                .desired_width(f32::INFINITY),
        )
    };

    if response.changed() {
        doc.sync_rope();
    }

    response
}
