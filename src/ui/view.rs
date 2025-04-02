use crate::utils::fs_handler::create_project_structure;
use eframe::{egui, epaint::Color32};
use rfd::FileDialog;
use std::path::PathBuf;

pub struct MyApp {
    readme_path: Option<PathBuf>,
    output_dir: Option<PathBuf>,
    message: String,
    success: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            readme_path: None,
            output_dir: None,
            message: String::new(),
            success: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        let accent_color = Color32::from_rgb(63, 81, 181);
        let success_color = Color32::from_rgb(76, 175, 80);
        let error_color = Color32::from_rgb(244, 67, 54);
        let warning_color = Color32::from_rgb(255, 152, 0);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("🚀 Générateur de Structure de Projet");
                ui.separator();
                ui.add_space(20.0);

                // Sélection du fichier README
                if ui.button("📄 Sélectionner README.md").clicked() {
                    if let Some(path) = FileDialog::new().pick_file() {
                        self.readme_path = Some(path);
                        self.message.clear();
                    }
                }
                if let Some(path) = &self.readme_path {
                    ui.label(format!("Fichier: {}", path.display()));
                }
                ui.add_space(10.0);

                // Sélection du dossier de sortie
                if ui.button("📂 Sélectionner Dossier de Sortie").clicked() {
                    if let Some(path) = FileDialog::new().pick_folder() {
                        self.output_dir = Some(path);
                        self.message.clear();
                    }
                }
                if let Some(path) = &self.output_dir {
                    ui.label(format!("Dossier: {}", path.display()));
                }
                ui.add_space(20.0);

                // Bouton de génération
                let can_generate = self.readme_path.is_some() && self.output_dir.is_some();
                if ui.add_enabled(can_generate, egui::Button::new("🚀 Générer Structure")).clicked() {
                    if let (Some(readme), Some(output)) = (&self.readme_path, &self.output_dir) {
                        match create_project_structure(&readme.display().to_string(), &output.display().to_string()) {
                            Ok(_) => {
                                self.message = "Structure créée avec succès !".to_string();
                                self.success = true;
                            },
                            Err(e) => {
                                self.message = format!("Erreur: {}", e);
                                self.success = false;
                            },
                        }
                    }
                }
                ui.add_space(20.0);

                // Message d'état
                if !self.message.is_empty() {
                    let (icon, message_color) = if self.success {
                        ("✅ ", success_color)
                    } else if self.message.contains("Veuillez") {
                        ("⚠️ ", warning_color)
                    } else {
                        ("❌ ", error_color)
                    };

                    ui.colored_label(message_color, format!("{}{}", icon, self.message));
                }
            });
        });
    }
}