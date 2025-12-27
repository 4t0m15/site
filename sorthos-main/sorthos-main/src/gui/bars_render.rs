use crate::gui::check_theme_consistency::apply_theme_consistency;
use crate::gui_backend::gui::Theme;
use crate::models::SortBar;
use crate::sorting::{start_sort, Operation, SortingAlgorithm};
use eframe::egui::{self, Color32};
use rand::seq::SliceRandom;
use std::sync::mpsc;

pub struct SortVisualizerApp {
    pub bars: Vec<SortBar>,
    pub algorithm: SortingAlgorithm,
    pub num_bars: usize,
    pub sorting: bool,
    rx: mpsc::Receiver<Operation>,
    pub tx: mpsc::Sender<Operation>,
    current_theme: Theme,
    pub status_message: String,
}

impl SortVisualizerApp {
    pub fn reset_bars(&mut self) {
        // Abandon any in-progress sort by replacing the channel and clearing sorting flag
        let (new_tx, new_rx) = mpsc::channel();
        self.tx = new_tx;
        self.rx = new_rx;
        self.sorting = false;

        self.bars = (0..self.num_bars).map(SortBar::new).collect();
        // Apply current theme to newly reset bars
        self.apply_theme(self.current_theme);
        // Reset status message
        self.status_message = "Array reset with unique values".to_string();
    }

    pub fn shuffle_bars(&mut self) {
        self.bars.shuffle(&mut rand::thread_rng());
        let duplicate_count = self.count_duplicates();
        if duplicate_count > 0 {
            self.status_message =
                format!("Array shuffled - {} duplicates present", duplicate_count);
        } else {
            self.status_message = "Array shuffled - no duplicates".to_string();
        }
    }

    pub fn count_duplicates(&self) -> usize {
        let mut seen_values = std::collections::HashMap::new();
        let mut duplicate_count = 0;

        for bar in &self.bars {
            let count = seen_values.entry(bar.value).or_insert(0);
            *count += 1;
            if *count == 2 {
                duplicate_count += 1; // First time we see a duplicate
            } else if *count > 2 {
                duplicate_count += 1; // Additional duplicates
            }
        }

        duplicate_count
    }

    pub fn remove_duplicates(&mut self) {
        let original_duplicate_count = self.count_duplicates();

        // Create a map to track seen values and their positions
        let mut seen_values = std::collections::HashMap::new();
        let mut unique_bars = Vec::new();

        for bar in &self.bars {
            if !seen_values.contains_key(&bar.value) {
                seen_values.insert(bar.value, true);
                unique_bars.push(bar.clone());
            }
        }

        // If we removed duplicates, we need to pad the array back to the original size
        // We'll fill with sequential values starting from the max existing value + 1
        if unique_bars.len() < self.bars.len() {
            let max_value = unique_bars.iter().map(|b| b.value).max().unwrap_or(0);
            let mut next_value = max_value + 1;

            while unique_bars.len() < self.bars.len() {
                unique_bars.push(SortBar::new(next_value));
                next_value += 1;
            }
        }

        self.bars = unique_bars;
        // Apply current theme to the deduplicated bars
        apply_theme_consistency(&mut self.bars, self.current_theme);

        // Update status message
        if original_duplicate_count > 0 {
            self.status_message = format!("Removed {} duplicate values", original_duplicate_count);
        } else {
            self.status_message = "No duplicates found".to_string();
        }
    }

    pub fn generate_with_duplicates(&mut self) {
        // Generate an array with intentional duplicates for testing
        let mut bars = Vec::new();
        let unique_values = self.num_bars / 3; // About 1/3 unique values

        // Create bars with repeated values
        for i in 0..self.num_bars {
            let value = i % unique_values;
            bars.push(SortBar::new(value));
        }

        self.bars = bars;
        // Shuffle to make the duplicates more interesting
        self.shuffle_bars();
        // Apply current theme
        apply_theme_consistency(&mut self.bars, self.current_theme);

        // Update status message
        let duplicate_count = self.count_duplicates();
        self.status_message = format!("Generated array with {} duplicates", duplicate_count);
    }

    pub fn start_sorting(&mut self, max_speed: bool) {
        if self.sorting {
            println!("[DEBUG] Sorting already in progress, ignoring start_sorting call.");
            return;
        }
        println!(
            "[DEBUG] Starting sorting with algorithm: {:?}, max_speed: {}",
            self.algorithm, max_speed
        );
        self.sorting = true;
        // Ensure displayed bars have the correct colors before sorting
        apply_theme_consistency(&mut self.bars, self.current_theme);
        // Clone and enforce correct bar colors before starting
        let mut bars_clone = self.bars.clone();
        apply_theme_consistency(&mut bars_clone, self.current_theme);
        let tx = self.tx.clone();
        if max_speed {
            println!("[DEBUG] Calling max_speed_sort...");
            crate::code::speed::max_speed_sort(&mut bars_clone, tx);
        } else {
            let algo = self.algorithm;
            println!("[DEBUG] Calling start_sort with algorithm: {:?}", algo);
            start_sort(algo, bars_clone, tx);
        }
    }

    fn handle_ops(&mut self) {
        let mut op_count = 0;
        while let Ok(op) = self.rx.try_recv() {
            op_count += 1;
            match op {
                Operation::Compare(i, j) => {
                    println!("[DEBUG] Operation::Compare({}, {})", i, j);
                    self.bars[i].color = Color32::YELLOW;
                    self.bars[j].color = Color32::YELLOW;
                }
                Operation::Swap(i, j) => {
                    println!("[DEBUG] Operation::Swap({}, {})", i, j);
                    self.bars.swap(i, j);
                    self.bars[i].color = Color32::GREEN;
                    self.bars[j].color = Color32::GREEN;
                }
                Operation::Overwrite(i, bar) => {
                    println!("[DEBUG] Operation::Overwrite({}, {:?})", i, bar.value);
                    self.bars[i] = bar;
                }
                Operation::SetColor(i, col) => {
                    // remap "WHITE reset" to your theme’s default background color
                    let default = match self.current_theme {
                        Theme::Light => Color32::BLACK,
                        Theme::Dark => Color32::WHITE,
                    };
                    println!("[DEBUG] Operation::SetColor({}, {:?})", i, col);
                    self.bars[i].color = if col == Color32::WHITE { default } else { col };
                }
                Operation::Done => {
                    println!("[DEBUG] Operation::Done received, sorting finished.");
                    self.sorting = false;
                }
            }
        }
        if op_count > 0 {
            println!(
                "[DEBUG] handle_ops processed {} operations this frame.",
                op_count
            );
        }
    }

    /// Apply the current theme to all bar colors.
    pub fn apply_theme(&mut self, theme: Theme) {
        // Store and apply theme
        self.current_theme = theme;
        apply_theme_consistency(&mut self.bars, theme);
    }

    /// Create a new SortVisualizerApp with given number of bars and initial algorithm.
    pub fn new(num_bars: usize, algorithm: SortingAlgorithm) -> Self {
        let (tx, rx) = mpsc::channel();
        let mut app = Self {
            bars: Vec::new(),
            algorithm,
            num_bars,
            sorting: false,
            tx,
            rx,
            current_theme: Theme::Light, // default, will be applied below
            status_message: String::new(),
        };
        // Initialize bars with default values and apply theme
        app.reset_bars();
        app
    }
}

impl eframe::App for SortVisualizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // First, handle incoming sort operations:
        self.handle_ops();

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            let rect = ui.available_rect_before_wrap();
            let n = self.bars.len() as f32;
            let bar_w = rect.width() / n;

            for (i, bar) in self.bars.iter().enumerate() {
                let x = rect.left() + i as f32 * bar_w;
                let h = rect.height() * (bar.value as f32 + 1.0) / n;
                let y = rect.bottom() - h;

                painter.rect_filled(
                    egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(bar_w - 1.0, h)),
                    0.0,
                    bar.color,
                );
            }
        });

        // keep repainting at ~60 fps during sort
        if self.sorting {
            ctx.request_repaint_after(std::time::Duration::from_millis(16));
        }
    }
}
