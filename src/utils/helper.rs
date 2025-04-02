use regex::Regex;

pub fn pipe_count(s:String) -> usize{
    return s.matches("│   ").count();
}

pub fn split_on_first_letter(s: String) -> Option<(String, String)> {
    let re = Regex::new(r"([A-Za-z].*)").unwrap();
    if let Some(cap) = re.captures(&s) {
        let start = cap.get(1).unwrap().start();
        return Some((s[..start].to_owned(), s[start..].to_owned()));
    }
    None
}

/// Extrait un chemin de fichier ou de dossier à partir d'une ligne
/// en ignorant les commentaires après le caractère #
pub fn extract_path_from_line(line: &str) -> Option<String> {
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