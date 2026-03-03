use std::path::{Path, PathBuf};


pub fn get_output_path(input_path: &Path, output_folder: &Option<PathBuf>, ext: &str) -> PathBuf {
    let file_name = input_path.file_name().unwrap_or_default();
    
    let base_path = match output_folder {
        Some(folder) => folder.join(file_name),
        None => input_path.to_path_buf(),
    };

    let mut output = base_path;
    output.set_extension(ext);
    output
}
