use crate::gif_handler::GifHandler;
use crate::gui::bars_render::SortVisualizerApp;
use eframe::egui::{self, pos2, vec2, Sense, Style, Visuals};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}
impl Theme {
    pub fn default_visuals(self) -> Visuals {
        match self {
            Self::Dark => Visuals::dark(),
            Self::Light => Visuals::light(),
        }
    }
    pub fn default_style(self) -> Style {
        Style {
            visuals: self.default_visuals(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum SortingAlgorithm {
    #[default]
    About,
    Controls,
    Info,
    Duck,
}

pub struct Sorthos {
    selected_algorithm: SortingAlgorithm,
    theme: Theme,
    duck_gif: GifHandler,
    sort_app: SortVisualizerApp,
    max_speed: bool,
}

impl Default for Sorthos {
    fn default() -> Self {
        Self {
            selected_algorithm: SortingAlgorithm::default(),
            theme: Theme::default(),
            duck_gif: GifHandler::default(),
            sort_app: SortVisualizerApp::new(100, crate::sorting::SortingAlgorithm::QuickVisual),
            max_speed: false,
        }
    }
}

impl Sorthos {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();
        const DUCK_GIF: &[u8] = include_bytes!("../assets/spinning-duck.gif");
        let _ = app
            .duck_gif
            .load_gif_from_bytes(&cc.egui_ctx, DUCK_GIF, "duck");
        app.sort_app.apply_theme(app.theme);
        app
    }

    fn show_info_page(&self, ui: &mut egui::Ui) {
        ui.heading("Algorithm Information");
        ui.separator();

        ui.label("Below is a list of how to and where sorting algorithms are used:");

        ui.separator();
        ui.label("- Timsort: O(n log n) worst case, O(n) best case (adaptive), Stable");
        ui.label("- Heapsort: O(n log n) all cases, Unstable");
        ui.label("- Merge Sort: O(n log n) all cases, Stable");
        ui.label("- Quicksort: O(n log n) average, O(n²) worst case, Unstable");

        ui.separator();
        ui.label("- Counting Sort: O(n + k), Stable");
        ui.label("- Radix Sort: O(d × (n + k)), Stable");
        ui.label("- Shell Sort: O(n^1.25) to O(n^1.5), Unstable");

        ui.separator();
        ui.label("- Bubble Sort: O(n²) average and worst case, O(n) best case (adaptive), Stable");
        ui.label("- Insertion Sort: O(n²) average and worst case, O(n) best case (adaptive), Stable",);
        ui.label("- Selection Sort: O(n²) all cases, Unstable");
        ui.label("- Cocktail Sort: O(n²) average and worst case, Stable");
        ui.label("- Gnome Sort: O(n²) worst case, O(n) best case, Stable");

        ui.separator();
        ui.label("- Bogo Sort: O((n+1)!) worst case, O(n) best case, Not guaranteed to finish, but there is a universe where it is the fastest");
    }

    fn show_duck_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("Duck");
        ui.separator();
        ui.vertical_centered(|ui| {
            self.duck_gif.render(ui, [128.0, 128.0]);
        });
    }

    fn show_controls_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("Sort Controls");
        ui.separator();

        ui.label("Controls:");

        ui.horizontal(|ui| {
            if ui.button("Shuffle").clicked() && !self.sort_app.sorting {
                self.sort_app.shuffle_bars();
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Remove Duplicates").clicked() && !self.sort_app.sorting {
                self.sort_app.remove_duplicates();
            }
            if ui.button("Generate Duplicates").clicked() && !self.sort_app.sorting {
                self.sort_app.generate_with_duplicates();
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Reset").clicked() && !self.sort_app.sorting {
                self.sort_app.reset_bars();
            }
        });

        ui.add(egui::Slider::new(&mut self.sort_app.num_bars, 16..=315).text("bars"))
            .on_hover_text("Change number of bars");

        ui.separator();

        ui.label("Status:");
        if !self.sort_app.status_message.is_empty() {
            ui.label(&self.sort_app.status_message);
        }
        let duplicate_count = self.sort_app.count_duplicates();
        if duplicate_count > 0 {
            ui.colored_label(
                egui::Color32::ORANGE,
                format!("⚠ {} duplicates detected", duplicate_count),
            );
        } else {
            ui.colored_label(egui::Color32::GREEN, "No duplicates");
        }
    }
}

fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let size = ui.spacing().interact_size.y * vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(size, Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Checkbox, ui.is_enabled(), *on, "")
    });
    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool_responsive(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter().rect(
            rect,
            radius,
            visuals.bg_fill,
            visuals.bg_stroke,
            egui::StrokeKind::Inside,
        );
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }
    response
}

pub fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| toggle_ui(ui, on)
}

impl eframe::App for Sorthos {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_style(self.theme.default_style());
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Sorthos");
                ui.separator();
                let mut max_speed = self.max_speed;
                let toggle_resp = ui.add_enabled(
                    !self.sort_app.sorting,
                    egui::widgets::Checkbox::new(&mut max_speed, "Max Speed"),
                );
                if toggle_resp.changed() {
                    self.max_speed = max_speed;
                }
            });
        });
        egui::SidePanel::left("algorithm_selector").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Dark/Light");
                let mut is_light = self.theme == Theme::Light;
                if ui.add(toggle(&mut is_light)).changed() {
                    self.theme = if is_light { Theme::Light } else { Theme::Dark };
                    self.sort_app.reset_bars();
                    self.sort_app.apply_theme(self.theme);
                }
            });
            ui.separator();
            if ui
                .selectable_label(
                    self.selected_algorithm == SortingAlgorithm::About,
                    "sorting",
                )
                .clicked()
            {
                self.selected_algorithm = SortingAlgorithm::About;
            }
            if ui
                .selectable_label(
                    self.selected_algorithm == SortingAlgorithm::Controls,
                    "controls",
                )
                .clicked()
            {
                self.selected_algorithm = SortingAlgorithm::Controls;
            }
            if ui
                .selectable_label(self.selected_algorithm == SortingAlgorithm::Duck, "duck")
                .clicked()
            {
                self.selected_algorithm = SortingAlgorithm::Duck;
            }
            if ui
                .selectable_label(self.selected_algorithm == SortingAlgorithm::Info, "info")
                .clicked()
            {
                self.selected_algorithm = SortingAlgorithm::Info;
            }
        });
        match self.selected_algorithm {
            SortingAlgorithm::Info => {
                egui::CentralPanel::default().show(ctx, |ui| self.show_info_page(ui));
            }
            SortingAlgorithm::About => {
                egui::SidePanel::left("sorting_controls").show(ctx, |ui| {
                    ui.label("Algorithm:");
                    for &alg in crate::sorting::SortingAlgorithm::all() {
                        if ui
                            .selectable_label(self.sort_app.algorithm == alg, format!("{alg}"))
                            .clicked()
                        {
                            self.sort_app.algorithm = alg;
                        }
                    }
                    ui.separator();
                    if ui.button("Sort").clicked() && !self.sort_app.sorting {
                        self.sort_app.start_sorting(self.max_speed);
                    }
                });
                self.sort_app.update(ctx, frame);
            }
            SortingAlgorithm::Controls => {
                egui::CentralPanel::default().show(ctx, |ui| self.show_controls_page(ui));
            }
            SortingAlgorithm::Duck => {
                egui::CentralPanel::default().show(ctx, |ui| self.show_duck_page(ui));
            }
        }
    }
}

pub fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Sorthos",
        options,
        Box::new(|cc| Ok(Box::new(Sorthos::new(cc)) as Box<dyn eframe::App>)),
    )
}
