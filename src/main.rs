use std::io;
mod utils;
use utils::fs_handler::create_project_structure;


/// Fonction principale pour tester
fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <chemin_readme> <dossier_sortie>", args[0]);
        return Ok(());
    }
    
    let readme_path = &args[1];
    let output_dir = &args[2];
    
    create_project_structure(readme_path, output_dir)
}