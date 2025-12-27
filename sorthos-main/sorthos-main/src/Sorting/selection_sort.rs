use crate::models::SortBar;
use eframe::egui::Color32;
use std::sync::mpsc;
use std::time::Duration;

/// Enhanced Visual Selection Sort implementation
pub fn selection_sort(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<crate::sorting::Operation>) {
    let n = bars.len();

    for i in 0..n {
        // Show sorted portion (light green) and unsorted portion (gray)
        for k in 0..i {
            let _ = tx.send(crate::sorting::Operation::SetColor(k, Color32::LIGHT_GREEN));
        }
        for k in i..n {
            let _ = tx.send(crate::sorting::Operation::SetColor(k, Color32::GRAY));
        }
        std::thread::sleep(Duration::from_millis(100));

        // Highlight current position being filled (blue)
        let _ = tx.send(crate::sorting::Operation::SetColor(i, Color32::BLUE));
        std::thread::sleep(Duration::from_millis(150));

        let mut min_idx = i;

        // Find the minimum element in the remaining unsorted array
        for j in i + 1..n {
            // Highlight current element being examined (yellow)
            let _ = tx.send(crate::sorting::Operation::SetColor(j, Color32::YELLOW));

            // Highlight current minimum candidate (red)
            let _ = tx.send(crate::sorting::Operation::SetColor(min_idx, Color32::RED));

            let _ = tx.send(crate::sorting::Operation::Compare(min_idx, j));
            std::thread::sleep(Duration::from_millis(80));

            if bars[j].value < bars[min_idx].value {
                // Reset previous minimum
                let _ = tx.send(crate::sorting::Operation::SetColor(min_idx, Color32::GRAY));
                // New minimum found
                min_idx = j;
                let _ = tx.send(crate::sorting::Operation::SetColor(min_idx, Color32::RED));
                std::thread::sleep(Duration::from_millis(60));
            }

            // Reset examined element color
            let _ = tx.send(crate::sorting::Operation::SetColor(j, Color32::GRAY));
        }

        // Show the final selection
        if min_idx != i {
            let _ = tx.send(crate::sorting::Operation::SetColor(i, Color32::GREEN));
            let _ = tx.send(crate::sorting::Operation::SetColor(min_idx, Color32::GREEN));
            std::thread::sleep(Duration::from_millis(100));

            let _ = tx.send(crate::sorting::Operation::Swap(i, min_idx));
            bars.swap(i, min_idx);
            std::thread::sleep(Duration::from_millis(120));
        }

        // Mark the element as sorted
        let _ = tx.send(crate::sorting::Operation::SetColor(i, Color32::LIGHT_GREEN));
        std::thread::sleep(Duration::from_millis(80));
    }

    // Final sweep: mark all elements as sorted (white)
    for i in 0..n {
        let _ = tx.send(crate::sorting::Operation::SetColor(i, Color32::WHITE));
        std::thread::sleep(Duration::from_millis(30));
    }
}
