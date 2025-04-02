use std::{fs::{self, File}, io::{self, Read}, path::PathBuf};
use crate::utils::parser::extract_directory_structure;

/// Creer une structure de dossiers et fichiers à partir du contenu d'un README.md
/// 
/// # Arguments
/// 
/// * `readme_path` - Chemin vers le fichier README.md à analyser
/// * `output_dir` - Dossier où la structure de projet sera creee
/// 
/// # Retour
/// 
/// Resultat OK si la structure a ete creee, Err sinon
pub fn create_project_structure(readme_path: &str, output_dir: &str) -> io::Result<()> {
    let mut readme_content = String::new();
    let mut file = File::open(readme_path)?;
    file.read_to_string(&mut readme_content)?;
    
    let paths = extract_directory_structure(&readme_content);
    
    let output_dir = PathBuf::from(output_dir);
    fs::create_dir_all(&output_dir)?;
    
    for path in paths {
        let cleaned_path = path.trim().replace('\\', "/");
        let full_path = output_dir.join(&cleaned_path);
        
        // Détection améliorée des dossiers
        let is_dir = cleaned_path.ends_with('/') 
            || cleaned_path.split('/').last().map_or(false, |s| !s.contains('.'))
            || path.contains("#") // Les commentaires indiquent souvent des dossiers
            || path.ends_with('/');
        
        if is_dir {
            fs::create_dir_all(&full_path)?;
            println!("✓ Dossier créé: {}", full_path.display());
        } else {
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)?;
            }
            File::create(&full_path)?;
            println!("✓ Fichier créé: {}", full_path.display());
        }
    }
    
    println!("\nStructure de projet créée avec succès dans: {}", output_dir.display());
    Ok(())
}