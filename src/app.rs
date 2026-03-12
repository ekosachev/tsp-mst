use eframe::{
    App, Frame,
    egui::{self, Color32, Pos2},
};

pub struct TspMstApp {
    nodes: Vec<Pos2>,
}

impl App for TspMstApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_content(ui);
        });
    }
}

impl TspMstApp {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    fn ui_content(&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(
            ui.available_size_before_wrap(),
            egui::Sense::click_and_drag(),
        );

        if let Some(mouse_pos) = response.interact_pointer_pos() && response.clicked() {
            self.nodes.push(mouse_pos);
        }

        self.nodes.iter().for_each(|&pos| {
            painter.circle_filled(pos, 5.0, Color32::from_rgb(200, 100, 100));
        });
    }
}
