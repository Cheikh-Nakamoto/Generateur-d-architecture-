use regex::Regex;
use crate::utils::helper::{extract_path_from_line,split_on_first_letter,pipe_count};


pub fn extract_directory_structure(content: &str) -> Vec<String> {
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


pub fn remake_root_directory(prev_line: String, line: String, root_directory: &mut String, current_dir: &str) {
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
