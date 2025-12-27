mod code;
#[path = "gif/gif_handler.rs"]
mod gif_handler;
mod gui;
mod gui_backend;
#[path = "core/models.rs"]
mod models;
#[path = "core/sorting.rs"]
mod sorting;

#[cfg(test)]
mod algorithm_tests;
#[cfg(test)]
mod tests;

fn main() -> Result<(), eframe::Error> {
    gui_backend::gui::run_gui()
}
