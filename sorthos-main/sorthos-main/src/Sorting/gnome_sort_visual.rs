use crate::models::SortBar;
use crate::sorting::Operation;
use eframe::egui::Color32;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn gnome_sort_visual(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    let n = bars.len();
    if n <= 1 {
        return;
    }

    let mut index = 0;

    while index < n {
        // Highlight current position being examined (blue)
        let _ = tx.send(Operation::SetColor(index, Color32::BLUE));
        thread::sleep(Duration::from_millis(100));

        // Show sorted portion (light green) and unsorted portion (gray)
        for i in 0..index {
            let _ = tx.send(Operation::SetColor(i, Color32::LIGHT_GREEN));
        }
        for i in index + 1..n {
            let _ = tx.send(Operation::SetColor(i, Color32::GRAY));
        }

        if index == 0 {
            // At the beginning, just move forward
            let _ = tx.send(Operation::SetColor(index, Color32::LIGHT_GREEN));
            index += 1;
            thread::sleep(Duration::from_millis(80));
        } else {
            // Compare current element with previous element
            let _ = tx.send(Operation::SetColor(index - 1, Color32::YELLOW));
            let _ = tx.send(Operation::SetColor(index, Color32::YELLOW));
            let _ = tx.send(Operation::Compare(index - 1, index));
            thread::sleep(Duration::from_millis(120));

            if bars[index - 1].value <= bars[index].value {
                // Elements are in correct order, move forward
                let _ = tx.send(Operation::SetColor(index - 1, Color32::LIGHT_GREEN));
                let _ = tx.send(Operation::SetColor(index, Color32::BLUE));
                thread::sleep(Duration::from_millis(60));
                index += 1;
            } else {
                // Elements are out of order, swap and move backward
                let _ = tx.send(Operation::SetColor(index - 1, Color32::RED));
                let _ = tx.send(Operation::SetColor(index, Color32::RED));
                thread::sleep(Duration::from_millis(80));

                let _ = tx.send(Operation::Swap(index - 1, index));
                bars.swap(index - 1, index);
                thread::sleep(Duration::from_millis(100));

                // Show the gnome moving backward (characteristic of gnome sort)
                let _ = tx.send(Operation::SetColor(
                    index - 1,
                    Color32::from_rgb(255, 165, 0),
                )); // Orange
                let _ = tx.send(Operation::SetColor(index, Color32::GRAY));
                thread::sleep(Duration::from_millis(80));

                index -= 1;
            }
        }

        // Brief pause to show the gnome's movement pattern
        thread::sleep(Duration::from_millis(40));
    }

    // Final sweep: show completion with a wave effect
    for i in 0..n {
        let _ = tx.send(Operation::SetColor(i, Color32::LIGHT_GREEN));
        thread::sleep(Duration::from_millis(50));
    }

    thread::sleep(Duration::from_millis(200));

    // Reset all colors to white
    for i in 0..n {
        let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
        thread::sleep(Duration::from_millis(20));
    }
}
