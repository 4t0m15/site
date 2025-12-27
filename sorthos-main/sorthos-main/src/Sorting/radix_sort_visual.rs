use crate::models::SortBar;
use crate::sorting::Operation;
use eframe::egui::Color32;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn radix_sort_visual(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    let n = bars.len();
    if n <= 1 {
        return;
    }

    // Find the maximum value to determine the number of digits
    let mut max_val = 0;
    for i in 0..n {
        let _ = tx.send(Operation::SetColor(i, Color32::LIGHT_BLUE));
        thread::sleep(Duration::from_millis(20));

        if bars[i].value > max_val {
            max_val = bars[i].value;
            let _ = tx.send(Operation::SetColor(i, Color32::RED));
            thread::sleep(Duration::from_millis(50));
        }

        let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
    }

    // Safety check: prevent excessive iterations with very large max values
    const MAX_SAFE_VALUE: usize = 100000; // Reasonable limit for radix sort visualization
    if max_val > MAX_SAFE_VALUE {
        // Fall back by resetting colors and returning early
        for i in 0..n {
            let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
        }
        return;
    }

    // Perform counting sort for every digit (from least significant to most significant)
    let mut exp = 1;
    while max_val / exp > 0 {
        // Highlight the current digit position being processed
        for i in 0..n {
            let digit = (bars[i].value / exp) % 10;
            let color = match digit {
                0 => Color32::from_rgb(255, 200, 200),
                1 => Color32::from_rgb(255, 220, 200),
                2 => Color32::from_rgb(255, 255, 200),
                3 => Color32::from_rgb(220, 255, 200),
                4 => Color32::from_rgb(200, 255, 200),
                5 => Color32::from_rgb(200, 255, 220),
                6 => Color32::from_rgb(200, 255, 255),
                7 => Color32::from_rgb(200, 220, 255),
                8 => Color32::from_rgb(200, 200, 255),
                _ => Color32::from_rgb(220, 200, 255),
            };
            let _ = tx.send(Operation::SetColor(i, color));
        }
        thread::sleep(Duration::from_millis(200));

        counting_sort_by_digit(bars, exp, tx);

        // Brief pause between digit positions
        thread::sleep(Duration::from_millis(100));
        exp *= 10;
    }

    // Final sweep to show completion
    for i in 0..n {
        let _ = tx.send(Operation::SetColor(i, Color32::LIGHT_GREEN));
        thread::sleep(Duration::from_millis(30));
        let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
    }
}

fn counting_sort_by_digit(bars: &mut Vec<SortBar>, exp: usize, tx: &mpsc::Sender<Operation>) {
    let n = bars.len();
    let mut output = vec![0; n];
    let mut count = [0; 10]; // Count array for digits 0-9

    // Count occurrences of each digit
    for i in 0..n {
        let digit = (bars[i].value / exp) % 10;
        count[digit] += 1;

        // Highlight the element being counted
        let _ = tx.send(Operation::SetColor(i, Color32::YELLOW));
        thread::sleep(Duration::from_millis(30));
    }

    // Convert count to cumulative count
    for i in 1..10 {
        count[i] += count[i - 1];
    }

    // Build output array from right to left to maintain stability
    for i in (0..n).rev() {
        let digit = (bars[i].value / exp) % 10;
        let _ = tx.send(Operation::SetColor(i, Color32::BLUE));
        thread::sleep(Duration::from_millis(40));

        count[digit] -= 1;
        output[count[digit]] = bars[i].value;

        let _ = tx.send(Operation::SetColor(i, Color32::GRAY));
        thread::sleep(Duration::from_millis(20));
    }

    // Copy the sorted elements back to original array
    for i in 0..n {
        let old_val = bars[i].value;
        bars[i].value = output[i];

        // Show the movement visually
        if old_val != output[i] {
            let _ = tx.send(Operation::SetColor(i, Color32::GREEN));
            thread::sleep(Duration::from_millis(60));
        } else {
            let _ = tx.send(Operation::SetColor(i, Color32::LIGHT_GRAY));
            thread::sleep(Duration::from_millis(30));
        }
    }

    // Reset colors after this digit pass
    for i in 0..n {
        let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
    }
    thread::sleep(Duration::from_millis(50));
}
