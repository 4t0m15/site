use crate::models::SortBar;
use eframe::egui::Color32;
use std::sync::mpsc;
use std::time::Duration;

/// Enhanced Visual Insertion Sort implementation
pub fn insertion_sort(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<crate::sorting::Operation>) {
    let n = bars.len();

    // Mark the first element as sorted (green)
    let _ = tx.send(crate::sorting::Operation::SetColor(0, Color32::LIGHT_GREEN));
    std::thread::sleep(Duration::from_millis(100));

    for i in 1..n {
        // Highlight the current element being inserted (red)
        let _ = tx.send(crate::sorting::Operation::SetColor(i, Color32::RED));
        std::thread::sleep(Duration::from_millis(150));

        let mut j = i;

        // Show the sorted portion (light green) and unsorted portion (gray)
        for k in 0..i {
            let _ = tx.send(crate::sorting::Operation::SetColor(k, Color32::LIGHT_GREEN));
        }
        for k in i + 1..n {
            let _ = tx.send(crate::sorting::Operation::SetColor(k, Color32::GRAY));
        }
        std::thread::sleep(Duration::from_millis(100));

        // Find the correct position for the current element
        while j > 0 {
            // Highlight comparison elements
            let _ = tx.send(crate::sorting::Operation::SetColor(j - 1, Color32::YELLOW));
            let _ = tx.send(crate::sorting::Operation::Compare(j - 1, j));
            std::thread::sleep(Duration::from_millis(80));

            if bars[j - 1].value > bars[j].value {
                // Show the swap with distinct colors
                let _ = tx.send(crate::sorting::Operation::SetColor(j - 1, Color32::BLUE));
                let _ = tx.send(crate::sorting::Operation::SetColor(j, Color32::BLUE));
                std::thread::sleep(Duration::from_millis(60));

                let _ = tx.send(crate::sorting::Operation::Swap(j - 1, j));
                bars.swap(j - 1, j);
                std::thread::sleep(Duration::from_millis(80));

                j -= 1;
            } else {
                // Found correct position, reset comparison color
                let _ = tx.send(crate::sorting::Operation::SetColor(
                    j - 1,
                    Color32::LIGHT_GREEN,
                ));
                break;
            }
        }

        // Mark the inserted element as part of sorted array
        let _ = tx.send(crate::sorting::Operation::SetColor(j, Color32::LIGHT_GREEN));
        std::thread::sleep(Duration::from_millis(100));
    }

    // Final pass: mark all elements as sorted (white)
    for i in 0..n {
        let _ = tx.send(crate::sorting::Operation::SetColor(i, Color32::WHITE));
        std::thread::sleep(Duration::from_millis(30));
    }
}
