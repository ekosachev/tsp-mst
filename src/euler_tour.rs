use eframe::egui;

pub fn build_euler_tour(mut traversal: Vec<egui::Pos2>) -> Vec<egui::Pos2> {
    if traversal.is_empty() {
        return Vec::new();
    }
    // remove duplicates while preserving order
    let mut seen = Vec::<egui::Pos2>::new();

    traversal.retain(|&pos| {
        if seen.contains(&pos) {
            false
        } else {
            seen.push(pos);
            true
        }
    });

    traversal.push(traversal[0]); // return to the starting point

    traversal
}