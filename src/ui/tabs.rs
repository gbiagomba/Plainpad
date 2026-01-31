use crate::document::Document;
use eframe::egui;

#[derive(Default)]
pub struct TabAction {
    pub select: Option<usize>,
    pub close: Option<usize>,
    pub new_tab: bool,
}

pub fn tab_bar(ui: &mut egui::Ui, documents: &[Document], active: usize) -> TabAction {
    let mut action = TabAction::default();

    ui.horizontal_wrapped(|ui| {
        for (index, doc) in documents.iter().enumerate() {
            let mut title = doc.title();
            if doc.is_dirty() {
                title.push_str(" *");
            }

            if ui.selectable_label(index == active, title).clicked() {
                action.select = Some(index);
            }

            if ui.small_button("×").clicked() {
                action.close = Some(index);
            }

            ui.add_space(6.0);
        }

        if ui.small_button("+").clicked() {
            action.new_tab = true;
        }
    });

    action
}
