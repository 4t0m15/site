use crate::models::SortBar;
use eframe::egui::Color32;
use std::sync::mpsc;

/// Visual Bubble Sort implementation
pub fn bubble_sort(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<crate::sorting::Operation>) {
    let n = bars.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            let _ = tx.send(crate::sorting::Operation::Compare(j, j + 1));
            std::thread::sleep(std::time::Duration::from_millis(10));

            if bars[j].value > bars[j + 1].value {
                let _ = tx.send(crate::sorting::Operation::Swap(j, j + 1));
                bars.swap(j, j + 1);
                std::thread::sleep(std::time::Duration::from_millis(10));
            }

            let _ = tx.send(crate::sorting::Operation::SetColor(j, Color32::WHITE));
            let _ = tx.send(crate::sorting::Operation::SetColor(j + 1, Color32::WHITE));
        }
    }
}
