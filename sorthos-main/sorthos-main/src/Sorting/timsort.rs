use super::sort_utils::insertion_sort_range_visual;
use crate::models::SortBar;
use crate::sorting::Operation;
use eframe::egui::Color32;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// A fixed run size simplifies the logic and is more reliable for visualization.
const RUN_SIZE: usize = 32;

/// A corrected and robust hybrid merge sort inspired by Timsort's principles.
pub fn tim_sort(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    let n = bars.len();
    if n <= 1 {
        return; // Already sorted.
    }

    // Step 1: Sort individual chunks (runs) of the array using Insertion Sort.
    for i in (0..n).step_by(RUN_SIZE) {
        let end = std::cmp::min(i + RUN_SIZE, n);
        insertion_sort_range_visual(bars, i, end, tx);
    }

    // Short pause to visually show the initial sorted runs.
    thread::sleep(Duration::from_millis(250));

    // Step 2: Iteratively merge the sorted runs in a bottom-up fashion.
    let mut size = RUN_SIZE;
    while size < n {
        for left_start in (0..n).step_by(2 * size) {
            let mid = std::cmp::min(left_start + size, n);
            let right_end = std::cmp::min(left_start + 2 * size, n);

            if mid < right_end {
                merge_visual(bars, left_start, mid, right_end, tx);
            }
        }
        size *= 2;
    }

    // Final sweep to confirm completion.
    for i in 0..n {
        let _ = tx.send(Operation::SetColor(i, Color32::LIGHT_GREEN));
        thread::sleep(Duration::from_millis(5));
    }
}

/// Merges two adjacent sorted subarrays: `bars[start..mid)` and `bars[mid..end)`.
fn merge_visual(
    bars: &mut Vec<SortBar>,
    start: usize,
    mid: usize,
    end: usize,
    tx: &mpsc::Sender<Operation>,
) {
    let left_len = mid - start;
    let right_len = end - mid;

    // Highlight the two ranges being merged.
    for x in start..mid {
        let _ = tx.send(Operation::SetColor(x, Color32::from_rgb(100, 100, 255)));
    }
    for x in mid..end {
        let _ = tx.send(Operation::SetColor(x, Color32::from_rgb(255, 100, 100)));
    }
    thread::sleep(Duration::from_millis(150));

    let temp = bars[start..end].to_vec();
    let (left, right) = temp.split_at(left_len);

    let mut i = 0;
    let mut j = 0;
    let mut k = start;

    while i < left_len && j < right_len {
        let _ = tx.send(Operation::Compare(start + i, mid + j));
        thread::sleep(Duration::from_millis(10));

        if left[i].value <= right[j].value {
            // THE FIX: Update the local `bars` vector AND send the message.
            bars[k] = left[i].clone();
            let _ = tx.send(Operation::Overwrite(k, left[i].clone()));
            i += 1;
        } else {
            // THE FIX: Update the local `bars` vector AND send the message.
            bars[k] = right[j].clone();
            let _ = tx.send(Operation::Overwrite(k, right[j].clone()));
            j += 1;
        }
        let _ = tx.send(Operation::SetColor(k, Color32::GREEN));
        thread::sleep(Duration::from_millis(10));
        k += 1;
    }

    // Copy remaining elements from the left side.
    while i < left_len {
        bars[k] = left[i].clone();
        let _ = tx.send(Operation::Overwrite(k, left[i].clone()));
        let _ = tx.send(Operation::SetColor(k, Color32::LIGHT_BLUE));
        thread::sleep(Duration::from_millis(5));
        i += 1;
        k += 1;
    }

    // Copy remaining elements from the right side.
    while j < right_len {
        bars[k] = right[j].clone();
        let _ = tx.send(Operation::Overwrite(k, right[j].clone()));
        let _ = tx.send(Operation::SetColor(k, Color32::LIGHT_RED));
        thread::sleep(Duration::from_millis(5));
        j += 1;
        k += 1;
    }

    // Reset colors for the newly merged section.
    for x in start..end {
        let _ = tx.send(Operation::SetColor(x, Color32::WHITE));
    }
}
