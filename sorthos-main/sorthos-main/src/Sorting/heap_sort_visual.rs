use crate::models::SortBar;
use crate::sorting::Operation;
use eframe::egui::Color32;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn heap_sort_visual(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    let n = bars.len();

    if n <= 1 {
        return;
    }

    // Build max heap
    for i in (0..n / 2).rev() {
        // Highlight the node being heapified
        let _ = tx.send(Operation::SetColor(i, Color32::BLUE));
        thread::sleep(Duration::from_millis(100));

        heapify_visual(bars, n, i, tx);

        // Reset color
        let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
    }

    // Extract elements from heap one by one
    for i in (1..n).rev() {
        // Highlight the elements being swapped (max element to sorted position)
        let _ = tx.send(Operation::SetColor(0, Color32::RED));
        let _ = tx.send(Operation::SetColor(i, Color32::GREEN));
        thread::sleep(Duration::from_millis(150));

        // Move current root (maximum) to end
        let _ = tx.send(Operation::Swap(0, i));
        bars.swap(0, i);
        thread::sleep(Duration::from_millis(80));

        // Mark the sorted element
        let _ = tx.send(Operation::SetColor(i, Color32::LIGHT_GREEN));

        // Reset root color
        let _ = tx.send(Operation::SetColor(0, Color32::WHITE));

        // Call heapify on the reduced heap
        heapify_visual(bars, i, 0, tx);
    }

    // Reset all colors to white at the end
    for i in 0..bars.len() {
        let _ = tx.send(Operation::SetColor(i, Color32::WHITE));
    }
}

fn heapify_visual(
    bars: &mut Vec<SortBar>,
    heap_size: usize,
    root: usize,
    tx: &mpsc::Sender<Operation>,
) {
    let mut largest = root;
    let left_child = 2 * root + 1;
    let right_child = 2 * root + 2;

    // Highlight the current root
    let _ = tx.send(Operation::SetColor(root, Color32::YELLOW));
    thread::sleep(Duration::from_millis(60));

    // Check if left child exists and is greater than root
    if left_child < heap_size {
        let _ = tx.send(Operation::SetColor(left_child, Color32::LIGHT_BLUE));
        let _ = tx.send(Operation::Compare(left_child, largest));
        thread::sleep(Duration::from_millis(80));

        if bars[left_child].value > bars[largest].value {
            largest = left_child;
        }
    }

    // Check if right child exists and is greater than largest so far
    if right_child < heap_size {
        let _ = tx.send(Operation::SetColor(right_child, Color32::LIGHT_BLUE));
        let _ = tx.send(Operation::Compare(right_child, largest));
        thread::sleep(Duration::from_millis(80));

        if bars[right_child].value > bars[largest].value {
            largest = right_child;
        }
    }

    // If largest is not root, swap and continue heapifying
    if largest != root {
        // Highlight the elements being swapped
        let _ = tx.send(Operation::SetColor(root, Color32::RED));
        let _ = tx.send(Operation::SetColor(largest, Color32::RED));
        thread::sleep(Duration::from_millis(100));

        let _ = tx.send(Operation::Swap(root, largest));
        bars.swap(root, largest);
        thread::sleep(Duration::from_millis(80));

        // Reset colors
        let _ = tx.send(Operation::SetColor(root, Color32::WHITE));
        if left_child < heap_size {
            let _ = tx.send(Operation::SetColor(left_child, Color32::WHITE));
        }
        if right_child < heap_size {
            let _ = tx.send(Operation::SetColor(right_child, Color32::WHITE));
        }

        // Recursively heapify the affected sub-tree
        heapify_visual(bars, heap_size, largest, tx);
    } else {
        // Reset colors if no swap occurred
        let _ = tx.send(Operation::SetColor(root, Color32::WHITE));
        if left_child < heap_size {
            let _ = tx.send(Operation::SetColor(left_child, Color32::WHITE));
        }
        if right_child < heap_size {
            let _ = tx.send(Operation::SetColor(right_child, Color32::WHITE));
        }
    }
}
