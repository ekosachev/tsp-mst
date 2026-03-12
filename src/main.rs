use eframe::{App, Frame, egui};

struct HelloApp;

impl App for HelloApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello world");
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Hello world",
        native_options,
        Box::new(|_cc| Ok(Box::new(HelloApp))),
    );
}
