use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{PathBuf};
use regex::Regex;

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
/// Extrait la structure de dossiers d'un contenu de README
fn extract_directory_structure(content: &str) -> Vec<String> {
    let mut paths = Vec::new();
    
    // Rechercher les blocs de code qui contiennent des structures de dossiers
    let code_block_regex = Regex::new(r"```(?:plaintext|bash|sh|txt|)\s*([\s\S]*?)```").unwrap();
    
    for captures in code_block_regex.captures_iter(content) {
        if let Some(code_block) = captures.get(1) {
            let block_content = code_block.as_str();
            
            // Verifier si c'est une structure de dossier
            if block_content.contains("/") || block_content.contains("\\") {
                // Extraire les chemins
                for line in block_content.lines() {
                    // Nettoyer la ligne
                    let cleaned_line = line.trim();
                    
                    // Ignorer les lignes vides
                    if cleaned_line.is_empty() {
                        continue;
                    }
                    
                    // Extraire le chemin de la ligne
                    let path = extract_path_from_line(cleaned_line);
                    if let Some(path) = path {
                        paths.push(path);
                    }
                }
            }
        }
    }
    
    // Extraire egalement la structure à partir des indentations
    let indentation_regex = Regex::new(r"(?m)^[ \t]*[├└]─+\s+([^/\n]+)(/?)$").unwrap();
    
    let mut current_path = Vec::new();
    let mut previous_indent = 0;
    
    for line in content.lines() {
        if line.contains("├─") || line.contains("└─") || line.contains("│") {
            // Calculer l'indentation
            let indent = line.chars().take_while(|&c| c == ' ' || c == '│' || c == '\t').count();
            
            // Ajuster le chemin actuel en fonction de l'indentation
            if indent < previous_indent {
                let levels_to_pop = (previous_indent - indent) / 2 + 1;
                for _ in 0..levels_to_pop {
                    current_path.pop();
                }
            } else if indent == previous_indent && !current_path.is_empty() {
                current_path.pop();
            }
            
            // Extraire le nom du fichier ou dossier
            if let Some(captures) = indentation_regex.captures(line) {
                let name = captures.get(1).unwrap().as_str().trim();
                let is_dir = captures.get(2).map_or(false, |m| m.as_str() == "/");
                
                current_path.push(name.to_string());
                let path_str = current_path.join("/");
                
                if is_dir {
                    paths.push(format!("{}/", path_str));
                } else {
                    paths.push(path_str);
                }
            }
            
            previous_indent = indent;
        }
    }
    
    // Deduplication et tri
    paths.sort();
    paths.dedup();
    
    paths
}

/// Extrait un chemin de fichier ou de dossier à partir d'une ligne
/// en ignorant les commentaires après le caractère #
fn extract_path_from_line(line: &str) -> Option<String> {
    // Ignorer les lignes qui ne contiennent pas de chemin
    if !line.contains('/') && !line.contains('.') {
        return None;
    }
    
    // Supprimer les commentaires (tout ce qui suit un #)
    let line_without_comments = if let Some(index) = line.find('#') {
        &line[0..index]
    } else {
        line
    };
    
    // Ignorer les lignes qui deviennent vides après suppression des commentaires
    if line_without_comments.trim().is_empty() {
        return None;
    }
    
    // Différents formats possibles pour les lignes de structure
    let patterns = [
        // Format: ├── src/
        Regex::new(r"[├└]─+\s+([./\w-]+(?:/[./\w-]+)*)").ok(),
        // Format: src/
        Regex::new(r"^([./\w-]+(?:/[./\w-]+)*)").ok(),
        // Format avec indentation: src/
        Regex::new(r"^\s+([./\w-]+(?:/[./\w-]+)*)").ok(),
    ];
    
    for pattern in patterns.iter().flatten() {
        if let Some(captures) = pattern.captures(line_without_comments) {
            if let Some(path) = captures.get(1) {
                let path_str = path.as_str().trim();
                
                // Ignorer les lignes qui ne sont pas des chemins
                if path_str.starts_with('+') || path_str.starts_with('-') {
                    continue;
                }
                
                return Some(path_str.to_string());
            }
        }
    }
    
    None
}

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