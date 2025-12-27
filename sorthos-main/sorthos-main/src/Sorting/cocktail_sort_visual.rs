use crate::models::SortBar;
use crate::sorting::Operation;
use eframe::egui::Color32;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn cocktail_sort_visual(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    let n = bars.len();
    if n <= 1 {
        return;
    }

    let mut left = 0;
    let mut right = n - 1;
    let mut swapped;

    loop {
        swapped = false;

        // Forward pass (left to right) - bubble largest to the right
        // Highlight the forward direction
        for i in left..right {
            let _ = tx.send(Operation::SetColor(i, Color32::LIGHT_BLUE));
            let _ = tx.send(Operation::SetColor(i + 1, Color32::LIGHT_BLUE));
        }
        thread::sleep(Duration::from_millis(100));

        for i in left..right {
            // Highlight elements being compared
            let _ = tx.send(Operation::SetColor(i, Color32::YELLOW));
            let _ = tx.send(Operation::SetColor(i + 1, Color32::YELLOW));
            let _ = tx.send(Operation::Compare(i, i + 1));
            thread::sleep(Duration::from_millis(60));

            if bars[i].value > bars[i + 1].value {
                // Show swap with green color
                let _ = tx.send(Operation::SetColor(i, Color32::GREEN));
                let _ = tx.send(Operation::SetColor(i + 1, Color32::GREEN));
                let _ = tx.send(Operation::Swap(i, i + 1));
                bars.swap(i, i + 1);
                swapped = true;
                thread::sleep(Duration::from_millis(80));
            }

            // Reset colors after comparison
            let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
            let _ = tx.send(Operation::SetColor(i + 1, Color32::WHITE));
        }

        // Mark the rightmost element as sorted (light green)
        let _ = tx.send(Operation::SetColor(right, Color32::LIGHT_GREEN));
        right -= 1;

        if !swapped {
            break;
        }

        swapped = false;

        // Backward pass (right to left) - bubble smallest to the left
        // Highlight the backward direction with different color
        for i in (left + 1..=right).rev() {
            let _ = tx.send(Operation::SetColor(i, Color32::LIGHT_YELLOW));
            let _ = tx.send(Operation::SetColor(i - 1, Color32::LIGHT_YELLOW));
        }
        thread::sleep(Duration::from_millis(100));

        for i in (left + 1..=right).rev() {
            // Highlight elements being compared with orange/red tones for backward pass
            let _ = tx.send(Operation::SetColor(i, Color32::from_rgb(255, 165, 0))); // Orange
            let _ = tx.send(Operation::SetColor(i - 1, Color32::from_rgb(255, 165, 0)));
            let _ = tx.send(Operation::Compare(i - 1, i));
            thread::sleep(Duration::from_millis(60));

            if bars[i - 1].value > bars[i].value {
                // Show swap with red color for backward pass
                let _ = tx.send(Operation::SetColor(i, Color32::RED));
                let _ = tx.send(Operation::SetColor(i - 1, Color32::RED));
                let _ = tx.send(Operation::Swap(i - 1, i));
                bars.swap(i - 1, i);
                swapped = true;
                thread::sleep(Duration::from_millis(80));
            }

            // Reset colors after comparison
            let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
            let _ = tx.send(Operation::SetColor(i - 1, Color32::WHITE));
        }

        // Mark the leftmost element as sorted (light green)
        let _ = tx.send(Operation::SetColor(left, Color32::LIGHT_GREEN));
        left += 1;

        if !swapped {
            break;
        }

        // Brief pause between passes to show the alternating pattern
        thread::sleep(Duration::from_millis(150));
    }

    // Final sweep: show all elements as sorted
    for i in 0..n {
        let _ = tx.send(Operation::SetColor(i, Color32::LIGHT_GREEN));
        thread::sleep(Duration::from_millis(30));
    }

    thread::sleep(Duration::from_millis(200));

    // Reset all colors to white
    for i in 0..n {
        let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
        thread::sleep(Duration::from_millis(20));
    }
}
