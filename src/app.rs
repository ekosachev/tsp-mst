use std::{time};

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
    mst_edges: Vec<Pos2>,
    depth_first_traversal: Vec<Pos2>,
    solution: Vec<Pos2>,
    render_edges: bool,
    render_mst: bool,
    render_dft: bool,
    render_solution: bool,
    mst_duration: f32,
    dft_duration: f32,
    solution_duration: f32,
}

impl App for TspMstApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_content(ui);
        });

        egui::Window::new("Параметры")
            .anchor(Align2::LEFT_TOP, (10.0, 10.0))
            .resizable(false)
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
            mst_edges: Vec::new(),
            depth_first_traversal: Vec::new(),
            solution: Vec::new(),
            render_edges: false,
            render_dft: true,
            render_mst: true,
            render_solution: true,
            mst_duration: 0.0,
            dft_duration: 0.0,
            solution_duration: 0.0,
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

            let mut start = time::Instant::now();

            self.mst_adjacency_list = prim::prim_algorithm(self.nodes.clone());
            self.mst_duration = start.elapsed().as_secs_f32();

            self.mst_edges.clear();

            for (u, neighbors) in self.mst_adjacency_list.iter().enumerate() {
                for neighbor in neighbors {
                    let node = self.nodes[u];

                    if self.mst_edges.contains(&(node, *neighbor))
                        || self.mst_edges.contains(&(*neighbor, node))
                    {
                        continue;
                    }

                    self.mst_edges.push((self.nodes[u], *neighbor));
                }
            }


            start = time::Instant::now();

            self.depth_first_traversal = crate::depth_first_traversal::depth_first_search(
                self.nodes.clone(),
                self.mst_adjacency_list.clone(),
            );
            self.dft_duration = start.elapsed().as_secs_f32();
            start = time::Instant::now();

            self.solution = crate::euler_tour::build_euler_tour(self.depth_first_traversal.clone());
            self.solution_duration = start.elapsed().as_secs_f32();
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
            painter.line(self.solution.clone(), (4.0, Color32::GREEN));
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
            } else {
                self.nodes.iter_mut().for_each(|pos| {
                    *pos += response.drag_delta();
                });
                self.is_dirty = true;
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
        let mst_weight = self
            .mst_edges
            .iter()
            .map(|(u, v)| u.distance(*v))
            .sum::<f32>();

        let solution_length = self
            .solution
            .windows(2)
            .map(|w| w[0].distance(w[1]))
            .sum::<f32>();

        ui.label("Видимость:");
        ui.checkbox(&mut self.render_edges, "Ребра");
        ui.checkbox(&mut self.render_mst, "Минимальное остовное дерево");
        ui.checkbox(&mut self.render_dft, "Обход в глубину");
        ui.checkbox(&mut self.render_solution, "Решение TSP");
        ui.label("Информация о решении:");
        ui.label(format!("Количество вершин: {}", self.nodes.len()));
        ui.label(format!("Вес MST: {:.2}", mst_weight));
        ui.label(format!("Длина решения: {:.2}", solution_length));
        ui.label(format!(
            "Качество решения: {:.2}",
            solution_length / (2.0 * mst_weight)
        ));

        let total_time = self.mst_duration + self.dft_duration + self.solution_duration;
        ui.label(format!(
            "Время построения MST: {:.2} μs ({:.2}%)",
            self.mst_duration * 1000000.0,
            (self.mst_duration / total_time) * 100.0
        ));
        ui.label(format!(
            "Время обхода в глубину: {:.2} μs ({:.2}%)",
            self.dft_duration * 1000000.0,
            (self.dft_duration / total_time) * 100.0
        ));
        ui.label(format!(
            "Время построения решения: {:.2} μs ({:.2}%)",
            self.solution_duration * 1000000.0,
            (self.solution_duration / total_time) * 100.0
        ));
        ui.label(format!("Общее время: {:.2} μs", total_time * 1000000.0));
        if ui.button("Сбросить").clicked() {
            self.nodes.clear();
            self.mst_edges.clear();
            self.depth_first_traversal.clear();
            self.solution.clear();
            self.is_dirty = false;
        }
    }
}
