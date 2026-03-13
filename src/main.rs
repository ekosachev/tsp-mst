use eframe;

mod app;
mod euler_tour;
mod data_structures;
mod depth_first_traversal;
mod prim;

use app::TspMstApp;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "TSP MST",
        native_options,
        Box::new(|_cc| Ok(Box::new(TspMstApp::new()))),
    );
}
