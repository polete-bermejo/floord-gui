use crate::{services::{configuration::{Configuration, generate_config, load_config}, installation::install_if_not}, views::organizer};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Menu {
    config: Configuration,
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            config: Configuration::default(),
        }
    }
}

impl Menu {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        install_if_not().unwrap();
        generate_config().unwrap();
        
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        Menu { config: load_config().unwrap() }
    }
}

impl eframe::App for Menu {
    /// Called each time the UI needs repainting, which may be many times per second.

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::Panel::top("top_panel").show(ui, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.collapsing("Organizer", |ui| {
                organizer::show(ui);
            })
        });
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
