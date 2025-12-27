use crate::models::SortBar;
use crate::sorting::Operation;
use eframe::egui::Color32;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn quick_sort_visual(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    if !bars.is_empty() {
        quick_sort_recursive(bars, 0, bars.len() - 1, tx);
    }

    // Reset all colors to white at the end
    for i in 0..bars.len() {
        let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
    }
}

fn quick_sort_recursive(
    bars: &mut Vec<SortBar>,
    low: usize,
    high: usize,
    tx: &mpsc::Sender<Operation>,
) {
    if low < high {
        let pi = partition_visual(bars, low, high, tx);

        if pi > 0 {
            quick_sort_recursive(bars, low, pi - 1, tx);
        }
        if pi + 1 <= high {
            quick_sort_recursive(bars, pi + 1, high, tx);
        }
    }
}

fn partition_visual(
    bars: &mut Vec<SortBar>,
    low: usize,
    high: usize,
    tx: &mpsc::Sender<Operation>,
) -> usize {
    let pivot_value = bars[high].value;
    let mut i = low;

    // Highlight pivot in red
    let _ = tx.send(Operation::SetColor(high, Color32::RED));
    thread::sleep(Duration::from_millis(30));

    for j in low..high {
        // Highlight current element being compared in yellow
        let _ = tx.send(Operation::SetColor(j, Color32::YELLOW));
        let _ = tx.send(Operation::Compare(j, high));
        thread::sleep(Duration::from_millis(50));

        if bars[j].value < pivot_value {
            if i != j {
                // Highlight the element to swap with in green
                let _ = tx.send(Operation::SetColor(i, Color32::GREEN));
                thread::sleep(Duration::from_millis(20));

                let _ = tx.send(Operation::Swap(i, j));
                bars.swap(i, j);
                thread::sleep(Duration::from_millis(60));

                // Reset color of swapped element
                let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
            }
            i += 1;
        }

        // Reset color of compared element
        let _ = tx.send(Operation::SetColor(j, Color32::WHITE));
    }

    // Place pivot in correct position
    if i != high {
        let _ = tx.send(Operation::SetColor(i, Color32::GREEN));
        thread::sleep(Duration::from_millis(20));

        let _ = tx.send(Operation::Swap(i, high));
        bars.swap(i, high);
        thread::sleep(Duration::from_millis(60));
    }

    // Reset colors
    let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
    let _ = tx.send(Operation::SetColor(high, Color32::WHITE));

    i
}
