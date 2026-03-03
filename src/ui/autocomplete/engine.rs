use std::fs;
use std::io; // Key fix for error E0433
use std::path::{Path, PathBuf};


pub struct SuggestionItem {
    pub name: String,
    pub is_dir: bool,
}


pub fn get_dir_and_stub(path_str: &str) -> (PathBuf, String) {
    let input_path = PathBuf::from(path_str);

    if path_str.ends_with('\\') || path_str.ends_with('/') {
        (input_path, String::new())
    } else {
        let parent = input_path.parent().unwrap_or_else(|| Path::new("")).to_path_buf();
        let file_stub = input_path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        (parent, file_stub)
    }
}


pub fn fetch_suggestions(dir: &Path, stub: &str) -> io::Result<Vec<SuggestionItem>> {
    let entries = fs::read_dir(dir)?;

    let mut items: Vec<SuggestionItem> = entries
        .flatten()
        .map(|e| SuggestionItem {
            name: e.file_name().to_string_lossy().to_string(),
            is_dir: e.path().is_dir(),
        })
        .filter(|s| s.name.to_lowercase().starts_with(&stub.to_lowercase()))
        .collect();

    items.sort_by(|a, b| {
        if a.is_dir == b.is_dir {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        } else {
            b.is_dir.cmp(&a.is_dir)
        }
    });

    Ok(items)
}


pub fn build_new_path(dir: &Path, name: &str, is_dir: bool) -> String {
    let mut new_path = dir.to_path_buf();
    new_path.push(name);

    let mut s = new_path.to_string_lossy().to_string();
    if is_dir && !s.ends_with('\\') && !s.ends_with('/') {
        s.push('\\');
    }
    s
}


#[cfg(test)]
#[path = "engine_tests.rs"]
mod tests;
