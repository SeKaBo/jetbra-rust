#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use anyhow::Result;
use eframe::egui::{ComboBox, Ui};
use eframe::{egui, Theme};

use jetbra::app::{App, Apps};
use jetbra::install::{InstallArgs, Installer};
use jetbra::jetbrains;
use jetbra::uninstall::{UninstallArgs, Uninstaller};

fn main() -> Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        resizable: false,
        default_theme: Theme::Light,
        ..Default::default()
    };
    let app = Jetbra::new()?;
    eframe::run_native("Jetbra", options, Box::new(|_cc| Box::from(app))).unwrap();
    Ok(())
}

struct Jetbra {
    apps: Vec<Apps>,
    installer: Installer,
    uninstaller: Uninstaller,

    selected_app: Option<Apps>,
}

impl Jetbra {
    fn new() -> Result<Self> {
        let jetbrains_dir = jetbrains::path()?;
        Ok(Self {
            apps: Apps::all(),
            installer: Installer::new(jetbrains_dir.clone()),
            uninstaller: Uninstaller::new(jetbrains_dir),
            selected_app: None,
        })
    }

    fn select_app(&mut self, ui: &mut Ui) {
        ComboBox::from_label("")
            .selected_text(match &self.selected_app {
                None => "All".to_owned(),
                Some(app) => App::from(*app).name,
            })
            .show_ui(ui, |ui| {
                // All
                ui.selectable_value(&mut self.selected_app, None, "All");
                // Apps
                self.apps.iter().for_each(|app| {
                    ui.selectable_value(&mut self.selected_app, Some(*app), App::from(*app).name);
                });
            });
    }

    fn install(&self, ui: &mut egui::Ui) -> Result<()> {
        if ui.button("Install").clicked() {
            self.installer.install(&InstallArgs {
                app: self.selected_app.map(|app| vec![app]),
            })?;
        }
        Ok(())
    }

    fn uninstall(&self, ui: &mut Ui) -> Result<()> {
        if ui.button("Uninstall").clicked() {
            self.uninstaller.uninstall(&UninstallArgs {
                app: self.selected_app.map(|app| vec![app]),
            })?;
        }
        Ok(())
    }
}

impl eframe::App for Jetbra {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Select app
            self.select_app(ui);
            // Install
            if let Err(err) = self.install(ui) {
                ui.label(err.to_string());
            }
            // Uninstall
            if let Err(err) = self.uninstall(ui) {
                ui.label(err.to_string());
            }
        });
    }
}