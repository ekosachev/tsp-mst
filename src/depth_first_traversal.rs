use eframe::egui;

pub fn depth_first_search(
    vertices: Vec<egui::Pos2>,
    adjacency_list: Vec<Vec<egui::Pos2>>,
) -> Vec<egui::Pos2> {
    if vertices.is_empty() || adjacency_list.is_empty() {
        return Vec::new();
    }
    let mut visited = vec![false; adjacency_list.len()];
    let mut frontier = Vec::<usize>::new();
    let mut result = Vec::<egui::Pos2>::new();

    frontier.push(0); // start from the first vertex
    while let Some(vertex_idx) = frontier.pop() {
        if visited[vertex_idx] {
            continue;
        }

        visited[vertex_idx] = true;
        result.push(vertices[vertex_idx]);
        for &neighbor in &adjacency_list[vertex_idx] {
            let neighbor_idx = vertices
                .iter()
                .position(|&v| v == neighbor)
                .expect("Neighbor vertex not found in vertices list");
            if !visited[neighbor_idx] {
                frontier.push(neighbor_idx);
            }
        }
    }
    result
}