use eframe;

mod app;

use app::TspMstApp;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "TSP MST",
        native_options,
        Box::new(|_cc| Ok(Box::new(TspMstApp::new()))),
    );
}
