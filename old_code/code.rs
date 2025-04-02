
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::PathBuf;

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
    let mut paths =vec![];
    let mut root_directory = String::new();
    let mut prev_line = String::new();
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
                    // if i == 0 {
                    //     root_directory = path.clone().unwrap_or_default();
                    // }
                   if !path.clone().unwrap_or_default().contains("."){
                    remake_root_directory(prev_line, cleaned_line.to_string(), &mut root_directory, &path.clone().unwrap_or_default());
                    prev_line = cleaned_line.to_owned();
                    paths.push(root_directory.clone());
                   }else{
                        let filepah = format!("{}/{}",root_directory,path.unwrap_or_default());
                        paths.push(filepah);
                   }
                   
                }
            }
        }
    }
    paths.sort();
    paths.dedup();
    
    paths
}

fn remake_root_directory(prev_line: String, line: String, root_directory: &mut String, current_dir: &str) {
    let prev_indent = pipe_count(split_on_first_letter(prev_line.clone()).unwrap_or_default().0);
    let curent_indent = pipe_count(split_on_first_letter(line).unwrap_or_default().0);
    // Calculer la différence d'indentation
   
    if curent_indent < prev_indent {
        let  diff_index = (curent_indent)as i8 -( prev_indent)as i8;
        let parts: Vec<&str> = root_directory.split('/').collect();
        
        let elem = format!("{}/{}", parts[..(parts.len() as i8-1 + diff_index) as usize].join("/"), current_dir);
        let mut parts: Vec<&str> = elem.split('/').collect();
        parts.retain(|a| !a.is_empty());
        *root_directory = parts.join("/");
    } else  {
        let mut parts: Vec<&str> = root_directory.split('/').collect();
        if curent_indent == prev_indent &&  !split_on_first_letter(prev_line.clone()).unwrap_or_default().0.is_empty() {
            parts.pop();
        }
        let elem = format!("{}/{}", parts.join("/"), current_dir);
        let mut parts: Vec<&str> = elem.split('/').collect();
        parts.retain(|a| !a.is_empty());
        *root_directory = parts.join("/");
    }
    
}

fn pipe_count(s:String) -> usize{
    return s.matches("│   ").count();
}

fn split_on_first_letter(s: String) -> Option<(String, String)> {
    let re = Regex::new(r"([A-Za-z].*)").unwrap();
    if let Some(cap) = re.captures(&s) {
        let start = cap.get(1).unwrap().start();
        return Some((s[..start].to_owned(), s[start..].to_owned()));
    }
    None
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