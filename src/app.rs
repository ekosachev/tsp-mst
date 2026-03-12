use eframe::{
    App, Frame,
    egui::{self, Align2, Color32, FontId, Pos2},
};

pub struct TspMstApp {
    nodes: Vec<Pos2>,
    hovered_node: Option<usize>,
    dragged_node: Option<usize>,
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
        Self {
            nodes: Vec::new(),
            hovered_node: None,
            dragged_node: None,
        }
    }

    fn ui_content(&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(
            ui.available_size_before_wrap(),
            egui::Sense::click_and_drag(),
        );

        self.process_mouse_input(&response);

        for i in 0..self.nodes.len() {
            for j in (i + 1)..self.nodes.len() {
                painter.line_segment(
                    [self.nodes[i], self.nodes[j]],
                    (2.0, Color32::DARK_GRAY),
                );
            }
        }

        self.nodes.iter().enumerate().for_each(|(i, &pos)| {
            painter.circle_filled(pos, 12.0, Color32::GRAY);
            painter.text(
                pos,
                Align2::CENTER_CENTER,
                i.to_string(),
                FontId::monospace(12.0),
                Color32::WHITE,
            );
        });
    }

    fn process_mouse_input(&mut self, response: &egui::Response) {
        if let Some(hover_pos) = response.hover_pos() {
            self.hovered_node = self
                .nodes
                .iter()
                .position(|&pos| (pos - hover_pos).length() < 20.0);
        }

        if let Some(click_pos) = response.interact_pointer_pos()
            && response.clicked()
        {
            if self.hovered_node.is_none() {
                self.nodes.push(click_pos);
            }
        }

        if response.drag_started()
            && let Some(hovered_node) = self.hovered_node
        {
            self.dragged_node = Some(hovered_node);
        }

        if response.drag_stopped() {
            self.dragged_node = None;
        }

        if response.dragged() {
            if let Some(dragged_node) = self.dragged_node {
                if let Some(mouse_pos) = response.hover_pos() {
                    self.nodes[dragged_node] = mouse_pos;
                }
            }
        }

        if response.double_clicked() {
            if let Some(hovered_node) = self.hovered_node {
                self.nodes.remove(hovered_node);

                self.hovered_node = None;
                self.dragged_node = None;
            }
        }
    }
}
