use std::f32::consts::E;

use eframe::{
    App, Frame,
    egui::{self, Align2, Color32, FontId, Pos2, Sense},
};

pub struct TspMstApp {
    nodes: Vec<Pos2>,
    hovered_node: Option<usize>,
    drawing_edge_from: Option<usize>,
    dragged_node: Option<usize>,
    edges: Vec<(usize, usize)>,
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
            drawing_edge_from: None,
            dragged_node: None,
            edges: Vec::new(),
        }
    }

    fn ui_content(&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(
            ui.available_size_before_wrap(),
            egui::Sense::click_and_drag(),
        );

        self.process_mouse_input(&response);

        self.edges.iter().for_each(|&(from, to)| {
            let from_pos = self.nodes[from];
            let to_pos = self.nodes[to];
            painter.line_segment([from_pos, to_pos], (2.0, Color32::GREEN));
        });

        if let Some(drawing_from) = self.drawing_edge_from {
            let from_pos = self.nodes[drawing_from];
            if let Some(mouse_pos) = response.hover_pos() {
                painter.line_segment([from_pos, mouse_pos], (2.0, Color32::LIGHT_GREEN));
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
            let target_node: usize;
            if self.hovered_node.is_none() {
                self.nodes.push(click_pos);
                target_node = self.nodes.len() - 1;
            } else {
                target_node = self.hovered_node.unwrap();
            }

            if let Some(drawing_from) = self.drawing_edge_from {
                self.edges.push((drawing_from, target_node));
                self.drawing_edge_from = None;
            }
        }

        if response.clicked_by(egui::PointerButton::Secondary) {
            if let Some(hovered_node) = self.hovered_node {
                if self.drawing_edge_from.is_none() {
                    self.drawing_edge_from = Some(hovered_node);
                } else {
                    self.edges
                        .push((self.drawing_edge_from.unwrap(), hovered_node));
                    self.drawing_edge_from = None;
                }
            } else {
                self.drawing_edge_from = None;
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

                self.edges.retain(|&(from, to)| from != hovered_node && to != hovered_node);
                for edge in &mut self.edges {
                    if edge.0 > hovered_node {
                        edge.0 -= 1;
                    }
                    if edge.1 > hovered_node {
                        edge.1 -= 1;
                    }
                }

                self.hovered_node = None;
                self.dragged_node = None;
                self.drawing_edge_from = None;
            }
        }
    }
}
