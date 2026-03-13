use eframe::{
    App, Frame,
    egui::{self, Align2, Color32, FontId, Pos2},
};

use crate::prim;

pub struct TspMstApp {
    nodes: Vec<Pos2>,
    hovered_node: Option<usize>,
    dragged_node: Option<usize>,
    is_dirty: bool,
    mst_adjacency_list: Vec<Vec<Pos2>>,
    depth_first_traversal: Vec<Pos2>,
    euler_tour: Vec<Pos2>,
    render_edges: bool,
    render_mst: bool,
    render_dft: bool,
    render_solution: bool,
}

impl App for TspMstApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_content(ui);
        });

        egui::Window::new("Параметры")
            .anchor(Align2::LEFT_TOP, (10.0, 10.0))
            .show(ctx, |ui| {
                self.ui_parameters(ui);
            });
    }
}

impl TspMstApp {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            hovered_node: None,
            dragged_node: None,
            is_dirty: false,
            mst_adjacency_list: Vec::new(),
            depth_first_traversal: Vec::new(),
            euler_tour: Vec::new(),
            render_edges: true,
            render_dft: true,
            render_mst: true,
            render_solution: true,
        }
    }

    fn ui_content(&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(
            ui.available_size_before_wrap(),
            egui::Sense::click_and_drag(),
        );

        self.process_mouse_input(&response);
        if self.render_edges {
            for i in 0..self.nodes.len() {
                for j in (i + 1)..self.nodes.len() {
                    painter.line_segment([self.nodes[i], self.nodes[j]], (1.0, Color32::DARK_GRAY));
                }
            }
        }

        if self.is_dirty {
            self.is_dirty = false;
            self.mst_adjacency_list = prim::prim_algorithm(self.nodes.clone());
            self.depth_first_traversal = crate::depth_first_traversal::depth_first_search(
                self.nodes.clone(),
                self.mst_adjacency_list.clone(),
            );
            self.euler_tour =
                crate::euler_tour::build_euler_tour(self.depth_first_traversal.clone());
        }

        if self.render_mst {
            for (i, neighbors) in self.mst_adjacency_list.iter().enumerate() {
                for &neighbor in neighbors {
                    painter.line_segment([self.nodes[i], neighbor], (2.0, Color32::RED));
                }
            }
        }

        if self.render_dft {
            painter.line(self.depth_first_traversal.clone(), (3.0, Color32::YELLOW));
        }

        if self.render_solution {
            painter.line(self.euler_tour.clone(), (4.0, Color32::GREEN));
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
                self.is_dirty = true;
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
                    self.is_dirty = true;
                }
            }
        }

        if response.double_clicked() {
            if let Some(hovered_node) = self.hovered_node {
                self.nodes.remove(hovered_node);

                self.hovered_node = None;
                self.dragged_node = None;
                self.is_dirty = true;
            }
        }
    }

    fn ui_parameters(&mut self, ui: &mut egui::Ui) {
        ui.label("Видимость:");
        ui.checkbox(&mut self.render_edges, "Ребра");
        ui.checkbox(&mut self.render_mst, "Минимальное остовное дерево");
        ui.checkbox(&mut self.render_dft, "Обход в глубину");
        ui.checkbox(&mut self.render_solution, "Решение TSP");
    }
}
