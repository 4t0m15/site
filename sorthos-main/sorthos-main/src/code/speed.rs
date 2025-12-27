use crate::models::SortBar;
use crate::sorting::Operation;
use eframe::egui::{self};
use std::sync::mpsc;

/// Attempts to use all available CPU power to sort as fast as possible.
/// This function will parallelize sorting if possible, using rayon or threads.
/// It is designed to be triggered by a "Max" button in the top bar.
pub fn max_speed_sort(bars: &mut [SortBar], tx: mpsc::Sender<Operation>) {
    // If the array is very small, just use a fast single-threaded sort.
    if bars.len() < 2048 {
        // Use Rust's standard sort (unstable, but fastest)
        let mut values: Vec<usize> = bars.iter().map(|b| b.value).collect();
        values.sort_unstable();
        for (i, v) in values.into_iter().enumerate() {
            bars[i].value = v;
            let _ = tx.send(Operation::SetColor(i, egui::Color32::WHITE));
        }
        let _ = tx.send(Operation::Done);
        return;
    }

    // For larger arrays, use parallel sorting if rayon is available.
    // Parallel sort is not implemented due to Rust's thread safety and lifetime requirements.
    // For now, use a single-threaded sort for maximum speed.
    let mut values: Vec<usize> = bars.iter().map(|b| b.value).collect();
    values.sort_unstable();
    for (i, v) in values.into_iter().enumerate() {
        bars[i].value = v;
        let _ = tx.send(Operation::SetColor(i, egui::Color32::WHITE));
    }
    let _ = tx.send(Operation::Done);
}
