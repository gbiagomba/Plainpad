mod app;
mod document;
mod editor;
mod shortcuts;
mod ui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "plainpad",
        options,
        Box::new(|_cc| Box::new(app::PlainpadApp::new())),
    )
}
