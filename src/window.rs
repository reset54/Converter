use eframe::egui;


pub const DEFAULT_WIDTH: f32 = 700.0;
pub const DEFAULT_HEIGHT: f32 = 550.0;


pub fn get_native_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Image Converter")
            .with_inner_size([DEFAULT_WIDTH, DEFAULT_HEIGHT])
            .with_min_inner_size([500.0, 400.0]),
        ..Default::default()
    }
}
