use crate::data_structures::MyPriorityQueue;
use eframe::egui;

struct HeapItem(pub f32, pub usize); // (key, vertex index)

impl PartialEq for HeapItem {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

pub fn prim_algorithm(vertices: Vec<egui::Pos2>) -> Vec<Vec<egui::Pos2>> {
    if vertices.len() < 2 {
        return Vec::new();
    }

    let mut key = vec![f32::INFINITY; vertices.len()]; // min edge weight to connect vertex to MST
    let mut parent = vec![None; vertices.len()]; // parent vertex in MST
    let mut in_mst = vec![false; vertices.len()];

    let mut heap = MyPriorityQueue::<HeapItem>::new();

    // initialization
    key[0] = 0.0;
    parent[0] = None;

    for (key, index) in key.iter().zip(0..) {
        heap.push(HeapItem(*key, index));
    }

    while let Some(HeapItem(_key_u, u)) = heap.pop() {
        if in_mst[u] {
            continue;
        }
        in_mst[u] = true;

        for v in 0..vertices.len() {
            if in_mst[v] || v == 0 {
                continue;
            }

            let edge_length = vertices[u].distance(vertices[v]);
            if edge_length < key[v] {
                key[v] = edge_length;
                parent[v] = Some(u);

                heap.push(HeapItem(key[v], v));
            }
        }
    }

    let parent_count = parent.iter().filter(|p| p.is_some()).count();
    assert_eq!(parent_count, vertices.len() - 1);

    // construct adjacency list of MST based on parent array

    let mut mst_adj_list = vec![Vec::new(); vertices.len()];
    for v in 1..vertices.len() {
        if let Some(u) = parent[v] {
            mst_adj_list[u].push(vertices[v]);
            mst_adj_list[v].push(vertices[u]);
        }
    }

    let edge_count: usize = mst_adj_list.iter().map(|v| v.len()).sum();

    assert_eq!(edge_count / 2, vertices.len() - 1);

    mst_adj_list
}
