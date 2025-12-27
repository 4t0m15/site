use crate::models::SortBar;
use crate::gui_backend::gui::Theme;
use eframe::egui::Color32;

/// Reset all bar colors to the default for the given theme
pub fn apply_theme_consistency(bars: &mut [SortBar], theme: Theme) {
    let default_color = match theme {
        Theme::Light => Color32::BLACK,
        Theme::Dark => Color32::WHITE,
    };
    for bar in bars {
        bar.color = default_color;
    }
}
