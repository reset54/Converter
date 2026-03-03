pub mod components;
pub mod autocomplete;

use eframe::egui;
use crate::app::ConverterApp;


pub fn render_ui(app: &mut ConverterApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Image Converter");
        ui.add_space(15.0);

        components::render_format_settings(ui, app);
        ui.add_space(10.0);

        components::render_dimension_settings(ui, app);
        ui.add_space(10.0);

        components::render_path_settings(ui, app);
        ui.add_space(20.0);

        components::render_controls(ui, app, ctx);
        ui.add_space(20.0);

        components::render_log_area(ui, app);
    });
}
