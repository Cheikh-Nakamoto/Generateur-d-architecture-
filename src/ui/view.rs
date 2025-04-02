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
        // Configurez un style global
        let mut style = (*ctx.style()).clone();
        style.text_styles.get_mut(&egui::TextStyle::Body).unwrap().size = 18.0;
        style.text_styles.get_mut(&egui::TextStyle::Button).unwrap().size = 20.0;
        style.text_styles.get_mut(&egui::TextStyle::Heading).unwrap().size = 32.0;
        ctx.set_style(style);

        // Configurer les couleurs personnalisées
        let accent_color = Color32::from_rgb(63, 81, 181); // Bleu indigo
        let success_color = Color32::from_rgb(76, 175, 80); // Vert
        let error_color = Color32::from_rgb(244, 67, 54); // Rouge
        let warning_color = Color32::from_rgb(255, 152, 0); // Orange

        egui::CentralPanel::default().show(ctx, |ui| {
           // ui.image(egui::include_image!("./assets/ferris.png"));
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                
                // Titre principal avec une couleur personnalisée
                ui.heading("Générateur de Structure de Projet");
                ui.add_space(30.0);

                // Conteneur pour les sélecteurs de fichiers
                egui::Frame::none().fill(Color32::from_rgb(240, 240, 245)).rounding(10.0).show(ui, |ui| {
                    ui.add_space(20.0);
                    
                    // Section README
                    ui.horizontal(|ui| {
                        if ui.add(egui::Button::new("📄 Sélectionner README.md").fill(accent_color).min_size(egui::vec2(250.0, 40.0))).clicked() {
                            if let Some(path) = FileDialog::new().pick_file() {
                                self.readme_path = Some(path);
                                self.message = String::new();
                            }
                        }
                        
                        if let Some(path) = &self.readme_path {
                            ui.label(format!("Fichier: {}", path.display()));
                        }
                    });

                    ui.add_space(15.0);

                    // Section dossier de sortie
                    ui.horizontal(|ui| {
                        if ui.add(egui::Button::new("📂 Sélectionner Dossier de Sortie").fill(accent_color).min_size(egui::vec2(250.0, 40.0))).clicked() {
                            if let Some(path) = FileDialog::new().pick_folder() {
                                self.output_dir = Some(path);
                                self.message = String::new();
                            }
                        }
                        
                        if let Some(path) = &self.output_dir {
                            ui.label(format!("Dossier: {}", path.display()));
                        }
                    });
                    
                    ui.add_space(20.0);
                });

                ui.add_space(30.0);

                // Bouton de génération avec style
                let generate_button = egui::Button::new("🚀 Générer Structure")
                    .min_size(egui::vec2(300.0, 50.0))
                    .fill(if self.readme_path.is_some() && self.output_dir.is_some() { 
                        accent_color 
                    } else { 
                        Color32::from_white_alpha(180)
                    });

                if ui.add(generate_button).clicked() {
                    if let (Some(readme), Some(output)) = (&self.readme_path, &self.output_dir) {
                        match create_project_structure(
                            &readme.display().to_string(),
                            &output.display().to_string(),
                        ) {
                            Ok(_) => {
                                self.message = "Structure créée avec succès !".to_string();
                                self.success = true;
                            },
                            Err(e) => {
                                self.message = format!("Erreur: {}", e);
                                self.success = false;
                            },
                        }
                    } else {
                        self.message = "Veuillez sélectionner un fichier README et un dossier de sortie.".to_string();
                        self.success = false;
                    }
                }

                ui.add_space(20.0);

                // Affichage du message avec un style adapté
                if !self.message.is_empty() {
                    let message_color = if self.success {
                        success_color
                    } else if self.message.contains("Veuillez") {
                        warning_color
                    } else if self.message.contains("Erreur") {
                        error_color
                    } else {
                        ui.style().visuals.text_color()
                    };

                    let icon = if self.success {
                        "✅ "
                    } else if self.message.contains("Veuillez") {
                        "⚠️ "
                    } else if self.message.contains("Erreur") {
                        "❌ "
                    } else {
                        ""
                    };

                    egui::Frame::none()
                        .fill(Color32::from_rgba_premultiplied(message_color.r(), message_color.g(), message_color.b(), 25))
                        .rounding(8.0)
                        .stroke((2.0, message_color))
                        .show(ui, |ui| {
                            ui.add_space(10.0);
                            ui.colored_label(message_color, format!("{}{}", icon, self.message));
                            ui.add_space(10.0);
                        });
                }
                
                ui.add_space(20.0);
            });
        });
    }
}