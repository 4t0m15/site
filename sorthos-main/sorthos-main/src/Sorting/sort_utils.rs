use crate::models::SortBar;
use crate::sorting::Operation;
use eframe::egui::Color32;
use std::sync::mpsc;
use std::time::Duration;

/// Visual insertion sort for SortBar with range support
pub fn insertion_sort_range_visual(
    bars: &mut Vec<SortBar>,
    start: usize,
    end: usize,
    tx: &mpsc::Sender<Operation>,
) {
    for i in (start + 1)..end {
        let mut j = i;

        // Highlight the element being inserted
        let _ = tx.send(Operation::SetColor(i, Color32::LIGHT_BLUE));
        std::thread::sleep(Duration::from_millis(50));

        while j > start {
            let _ = tx.send(Operation::Compare(j - 1, j));
            std::thread::sleep(Duration::from_millis(30));

            if bars[j - 1].value > bars[j].value {
                let _ = tx.send(Operation::Swap(j - 1, j));
                bars.swap(j - 1, j);
                std::thread::sleep(Duration::from_millis(40));
                j -= 1;
            } else {
                break;
            }
        }

        // Reset color
        let _ = tx.send(Operation::SetColor(j, Color32::WHITE));
    }
}
