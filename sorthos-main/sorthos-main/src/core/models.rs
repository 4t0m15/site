use eframe::egui::Color32;

#[derive(Debug, Clone)]
pub struct SortBar {
    pub value: usize,
    pub color: Color32,
}

impl SortBar {
    pub fn new(value: usize) -> Self {
        Self {
            value,
            color: Color32::WHITE,
        }
    }
}
