use eframe::egui;

use crate::data_structures::MyPriorityQueue;

struct HeapItem(pub f32, pub usize);  // (key, vertex index)

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
    let mut key = Vec::<f32>::with_capacity(vertices.len());  // min edge weight to connect vertex to MST
    let mut parent = Vec::<Option<usize>>::with_capacity(vertices.len());  // parent vertex in MST
    let mut in_mst = Vec::<bool>::with_capacity(vertices.len()); 

    let mut heap = MyPriorityQueue::<HeapItem>::new();

    // initialization
    key[0] = 0.0;
    parent[0] = None;
    in_mst[0] = false;

    for i in 1..vertices.len() {
        key[i] = f32::INFINITY;
        parent[i] = None;
        in_mst[i] = false;
    }

    for (key, index) in key.iter().zip(0..) {
        heap.push(HeapItem(*key, index));
    }

    for _ in 0..vertices.len() {
        let u = heap.pop().unwrap().1;  // vertex with smallest key
        in_mst[u] = true;

        for v in 0..vertices.len() {
            if in_mst[v] { continue; }

            let edge_length = vertices[u].distance(vertices[v]);
            if edge_length < key[v] {
                key[v] = edge_length;
                parent[v] = Some(u);

                if let Some(i) = heap.find(&HeapItem(key[v], v)) {
                    heap.get_mut(i).unwrap().0 = key[v];  // update key in heap
                    heap.bubble_up(i);  // restore heap property
                }
            }
        }
    }

    // construct adjacency list of MST based on parent array

    let mut mst_adj_list = vec![Vec::new(); vertices.len()];
    for v in 1..vertices.len() {
        if let Some(u) = parent[v] {
            mst_adj_list[u].push(vertices[v]);
            mst_adj_list[v].push(vertices[u]);
        }
    }
    mst_adj_list

}