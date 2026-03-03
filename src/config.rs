pub struct ConverterConfig {
    pub width: String,
    pub height: String,
    pub input_format: String,
    pub output_format: String,
}


impl ConverterConfig {
    pub fn available_formats() -> Vec<&'static str> {
        vec!["png", "jpg", "jpeg", "bmp", "webp", "tiff", "ico", "tga"]
    }
}


impl Default for ConverterConfig {
    fn default() -> Self {
        Self {
            width: "570".to_string(),
            height: "342".to_string(),
            input_format: "png".to_string(),
            output_format: "bmp".to_string(),
        }
    }
}
