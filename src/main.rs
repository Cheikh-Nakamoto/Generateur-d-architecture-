use std::io;
use eframe::egui;
use ui::view::MyApp;

mod utils;
mod ui;

fn main() -> io::Result<()> { 
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([150.0, 200.0])  // Modifié de [200.0, 100.0] à [150.0, 200.0]
            .with_resizable(false)
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("./assets/ferris.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    
    let _ = eframe::run_native(
        "Project Structure Generator",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    );
    
    Ok(())
}
