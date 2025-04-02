use std::io;

use ui::view::MyApp;
mod utils;
mod ui;



/// Fonction principale pour tester
fn main() -> io::Result<()> { 
    let options = eframe::NativeOptions::default();
    eframe::run_native("Project Structure Generator", options, Box::new(|_cc| Box::new(MyApp::default())));
    Ok(())
}