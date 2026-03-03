use image::{DynamicImage, ImageFormat};
use std::path::Path;


pub fn process_single_file(
    input_file: &Path,
    output_file: &Path,
    w: u32,
    h: u32,
    out_ext: &str
) -> Result<(), String> {
    let img: DynamicImage = image::open(input_file)
        .map_err(|e| format!("Failed to open: {}", e))?;

    let resized = img.resize_exact(w, h, image::imageops::FilterType::Lanczos3);

    // Dynamic format detection by extension
    let format = ImageFormat::from_extension(out_ext)
        .ok_or_else(|| format!("Unsupported output format: {}", out_ext))?;

    resized.save_with_format(output_file, format)
        .map_err(|e| format!("Failed to save: {}", e))?;

    Ok(())
}
