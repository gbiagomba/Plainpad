mod app;
mod document;
mod editor;
mod shortcuts;
mod ui;
mod updater;

fn main() -> eframe::Result<()> {
    // Initialize Velopack - must run first as it may terminate/restart the process
    velopack::VelopackApp::build().run();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "plainpad",
        options,
        Box::new(|_cc| Box::new(app::PlainpadApp::new())),
    )
}
