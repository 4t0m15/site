use crate::models::SortBar;
use crate::sorting::Operation;
use eframe::egui::Color32;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn merge_sort_visual(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    let len = bars.len();
    if len > 1 {
        merge_sort_recursive(bars, 0, len - 1, tx);
    }

    // Reset all colors to white at the end
    for i in 0..bars.len() {
        let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
    }
}

fn merge_sort_recursive(
    bars: &mut Vec<SortBar>,
    left: usize,
    right: usize,
    tx: &mpsc::Sender<Operation>,
) {
    if left < right {
        let mid = left + (right - left) / 2;

        // Highlight the current section being divided
        for i in left..=right {
            let _ = tx.send(Operation::SetColor(i, Color32::LIGHT_BLUE));
        }
        thread::sleep(Duration::from_millis(100));

        // Recursively sort left half
        merge_sort_recursive(bars, left, mid, tx);

        // Recursively sort right half
        merge_sort_recursive(bars, mid + 1, right, tx);

        // Merge the sorted halves
        merge_visual(bars, left, mid, right, tx);
    }
}

fn merge_visual(
    bars: &mut Vec<SortBar>,
    left: usize,
    mid: usize,
    right: usize,
    tx: &mpsc::Sender<Operation>,
) {
    // Create temporary arrays for left and right subarrays (store full SortBar objects)
    let left_arr: Vec<SortBar> = bars[left..=mid].to_vec();
    let right_arr: Vec<SortBar> = bars[mid + 1..=right].to_vec();

    let left_size = left_arr.len();
    let right_size = right_arr.len();

    // Highlight the sections being merged
    for i in left..=mid {
        let _ = tx.send(Operation::SetColor(i, Color32::GREEN));
    }
    for i in mid + 1..=right {
        let _ = tx.send(Operation::SetColor(i, Color32::YELLOW));
    }
    thread::sleep(Duration::from_millis(100));

    let mut i = 0; // Index for left subarray
    let mut j = 0; // Index for right subarray
    let mut k = left; // Index for merged array

    // Merge the arrays
    while i < left_size && j < right_size {
        // Highlight elements being compared
        let left_idx = left + i;
        let right_idx = mid + 1 + j;

        let _ = tx.send(Operation::Compare(left_idx, right_idx));
        thread::sleep(Duration::from_millis(80));

        if left_arr[i].value <= right_arr[j].value {
            // Take from left array - update local bars AND send to visualization
            bars[k] = left_arr[i].clone();
            let _ = tx.send(Operation::Overwrite(k, left_arr[i].clone()));
            let _ = tx.send(Operation::SetColor(k, Color32::LIGHT_GREEN));
            i += 1;
        } else {
            // Take from right array - update local bars AND send to visualization
            bars[k] = right_arr[j].clone();
            let _ = tx.send(Operation::Overwrite(k, right_arr[j].clone()));
            let _ = tx.send(Operation::SetColor(k, Color32::LIGHT_YELLOW));
            j += 1;
        }

        thread::sleep(Duration::from_millis(60));
        k += 1;
    }

    // Copy remaining elements from left array
    while i < left_size {
        bars[k] = left_arr[i].clone();
        let _ = tx.send(Operation::Overwrite(k, left_arr[i].clone()));
        let _ = tx.send(Operation::SetColor(k, Color32::LIGHT_GREEN));
        thread::sleep(Duration::from_millis(40));
        i += 1;
        k += 1;
    }

    // Copy remaining elements from right array
    while j < right_size {
        bars[k] = right_arr[j].clone();
        let _ = tx.send(Operation::Overwrite(k, right_arr[j].clone()));
        let _ = tx.send(Operation::SetColor(k, Color32::LIGHT_YELLOW));
        thread::sleep(Duration::from_millis(40));
        j += 1;
        k += 1;
    }

    // Reset colors for the merged section
    for idx in left..=right {
        let _ = tx.send(Operation::SetColor(idx, Color32::WHITE));
    }
    thread::sleep(Duration::from_millis(50));
}
