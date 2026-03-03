use super::*;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use tempfile::tempdir;


#[test]
fn test_get_dir_and_stub() {
    let (dir, stub) = get_dir_and_stub("C:\\Windows\\");
    assert_eq!(dir, PathBuf::from("C:\\Windows\\"));
    assert_eq!(stub, "");

    let (dir, stub) = get_dir_and_stub("C:\\Windows\\System32");
    assert_eq!(dir, PathBuf::from("C:\\Windows\\"));
    assert_eq!(stub, "System32");
}


#[test]
fn test_build_new_path() {
    let base_dir = Path::new("C:\\Data");
    
    let file_path = build_new_path(base_dir, "test.txt", false);
    assert_eq!(file_path, "C:\\Data\\test.txt");

    let dir_path = build_new_path(base_dir, "projects", true);
    assert_eq!(dir_path, "C:\\Data\\projects\\");
}


#[test]
fn test_fetch_suggestions_sorting() {
    let tmp = tempdir().unwrap();
    let path = tmp.path();

    File::create(path.join("b_file.txt")).unwrap();
    fs::create_dir(path.join("m_folder")).unwrap();

    let suggestions = fetch_suggestions(path, "").unwrap(); // Added unwrap()

    assert_eq!(suggestions.len(), 2);
    assert_eq!(suggestions[0].name, "m_folder");
}


#[test]
fn test_fetch_suggestions_error() {
    let path = Path::new("C:\\non_existent_drive_123");
    let result = fetch_suggestions(path, "");
    assert!(result.is_err());
}
