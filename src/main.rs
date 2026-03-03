#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod converter;
mod app;
mod ui;
mod utils;
mod config;
mod cache;
mod window;
mod log;

use crate::app::ConverterApp;
use crate::window::get_native_options;


fn main() -> eframe::Result<()> {
    // Initialize env_logger for terminal debug output
    env_logger::init();

    // Start eframe application with settings from window.rs
    eframe::run_native(
        "Image Converter",
        get_native_options(),
        Box::new(|cc| Box::new(ConverterApp::new(cc))),
    )
}
