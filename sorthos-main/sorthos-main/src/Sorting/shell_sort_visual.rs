use crate::models::SortBar;
use crate::sorting::Operation;
use eframe::egui::Color32;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn shell_sort_visual(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    let n = bars.len();
    if n <= 1 {
        return;
    }

    // Start with a big gap, then reduce the gap
    let mut gap = n / 2;

    while gap > 0 {
        // Highlight the current gap size by showing which elements will be compared
        show_gap_groups(bars, gap, tx);

        // Do a gapped insertion sort for this gap size
        // The first gap elements bars[0..gap] are already in gapped order
        for i in gap..n {
            // Save bars[i] in temp and make a hole at position i
            let temp = bars[i].value;

            // Highlight the element being inserted
            let _ = tx.send(Operation::SetColor(i, Color32::RED));
            thread::sleep(Duration::from_millis(100));

            let mut j = i;

            // Shift earlier gap-sorted elements up until the correct location for bars[i] is found
            while j >= gap {
                // Highlight the elements being compared
                let _ = tx.send(Operation::SetColor(j - gap, Color32::YELLOW));
                let _ = tx.send(Operation::SetColor(j, Color32::YELLOW));
                let _ = tx.send(Operation::Compare(j - gap, j));
                thread::sleep(Duration::from_millis(80));

                if bars[j - gap].value > temp {
                    // Show the shift operation
                    let _ = tx.send(Operation::SetColor(j - gap, Color32::BLUE));
                    let _ = tx.send(Operation::SetColor(j, Color32::BLUE));

                    bars[j].value = bars[j - gap].value;
                    let _ = tx.send(Operation::Swap(j - gap, j));
                    thread::sleep(Duration::from_millis(100));

                    j -= gap;
                } else {
                    break;
                }

                // Reset colors after comparison
                let _ = tx.send(Operation::SetColor(j + gap, Color32::WHITE));
                if j >= gap {
                    let _ = tx.send(Operation::SetColor(j - gap, Color32::WHITE));
                }
            }

            // Put temp (the original bars[i]) in its correct location
            bars[j].value = temp;
            let _ = tx.send(Operation::SetColor(j, Color32::GREEN));
            thread::sleep(Duration::from_millis(60));

            // Reset the inserted element color
            let _ = tx.send(Operation::SetColor(j, Color32::WHITE));
        }

        // Show completion of this gap phase
        for i in 0..n {
            let _ = tx.send(Operation::SetColor(i, Color32::LIGHT_GREEN));
            thread::sleep(Duration::from_millis(20));
            let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
        }

        thread::sleep(Duration::from_millis(200));
        gap /= 2;
    }

    // Final sweep to show completion
    for i in 0..n {
        let _ = tx.send(Operation::SetColor(i, Color32::LIGHT_GREEN));
        thread::sleep(Duration::from_millis(40));
        let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
    }
}

fn show_gap_groups(bars: &Vec<SortBar>, gap: usize, tx: &mpsc::Sender<Operation>) {
    let n = bars.len();

    // Color different gap groups with different colors
    let colors = [
        Color32::from_rgb(255, 200, 200), // Light red
        Color32::from_rgb(200, 255, 200), // Light green
        Color32::from_rgb(200, 200, 255), // Light blue
        Color32::from_rgb(255, 255, 200), // Light yellow
        Color32::from_rgb(255, 200, 255), // Light magenta
        Color32::from_rgb(200, 255, 255), // Light cyan
        Color32::from_rgb(255, 230, 200), // Light orange
        Color32::from_rgb(230, 200, 255), // Light purple
    ];

    // Assign colors to show gap groups
    for i in 0..n {
        let group = i % gap;
        let color = colors[group % colors.len()];
        let _ = tx.send(Operation::SetColor(i, color));
    }

    thread::sleep(Duration::from_millis(300));

    // Reset all colors
    for i in 0..n {
        let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
    }
}
