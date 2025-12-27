use crate::models::SortBar;
use crate::sorting::Operation;
use eframe::egui::Color32;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn counting_sort_visual(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    let n = bars.len();
    if n <= 1 {
        return;
    }

    // Find the maximum value with visual feedback
    let mut max_val = 0;
    for i in 0..n {
        let _ = tx.send(Operation::SetColor(i, Color32::LIGHT_BLUE));
        thread::sleep(Duration::from_millis(30));

        if bars[i].value > max_val {
            max_val = bars[i].value;
            let _ = tx.send(Operation::SetColor(i, Color32::RED));
            thread::sleep(Duration::from_millis(50));
        }

        let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
    }

    // Safety check: prevent memory issues with very large max values
    const MAX_SAFE_SIZE: usize = 10000; // Reasonable limit for visualization
    if max_val > MAX_SAFE_SIZE {
        // Fall back to a different sorting algorithm or skip
        // For now, we'll just reset colors and return
        for i in 0..n {
            let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
        }
        return;
    }

    // Create count array
    let mut count = vec![0; max_val + 1];

    // Count occurrences with visual feedback
    for i in 0..n {
        let _ = tx.send(Operation::SetColor(i, Color32::YELLOW));
        thread::sleep(Duration::from_millis(60));

        count[bars[i].value] += 1;

        let _ = tx.send(Operation::SetColor(i, Color32::LIGHT_GREEN));
        thread::sleep(Duration::from_millis(40));
    }

    // Convert count array to cumulative count
    for i in 1..count.len() {
        count[i] += count[i - 1];
    }

    // Create output array
    let mut output = vec![0; n];

    // Build the output array from right to left to maintain stability
    for i in (0..n).rev() {
        let val = bars[i].value;
        let _ = tx.send(Operation::SetColor(i, Color32::BLUE));
        thread::sleep(Duration::from_millis(50));

        count[val] -= 1;
        output[count[val]] = val;

        let _ = tx.send(Operation::SetColor(i, Color32::GRAY));
        thread::sleep(Duration::from_millis(30));
    }

    // Copy the sorted elements back to original array with visual feedback
    for i in 0..n {
        let _ = tx.send(Operation::SetColor(i, Color32::GREEN));
        bars[i].value = output[i];
        thread::sleep(Duration::from_millis(80));

        let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
        thread::sleep(Duration::from_millis(20));
    }

    // Final pass to show completion
    for i in 0..n {
        let _ = tx.send(Operation::SetColor(i, Color32::LIGHT_GREEN));
        thread::sleep(Duration::from_millis(20));
        let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
    }
}
