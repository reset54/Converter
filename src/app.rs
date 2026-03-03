use eframe::{egui, App, CreationContext, Frame};
use std::path::PathBuf;
use std::thread;
use std::fs;

use crate::config::ConverterConfig;
use crate::cache::AppCache;
use crate::converter::process_single_file;
use crate::log::{Logger, LogHandle};
use crate::ui::render_ui;
use crate::utils::get_output_path;


pub struct ConverterApp {
    pub config: ConverterConfig,
    pub cache: AppCache,
    pub is_running: bool,
    pub log_handle: LogHandle,
}


impl ConverterApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        egui_extras::install_image_loaders(&cc.egui_ctx);

        Self {
            config: ConverterConfig::default(),
            cache: AppCache::load(),
            is_running: false,
            log_handle: Logger::new_handle(),
        }
    }


    pub fn check_completion(&mut self) {
        if let Ok(lg) = self.log_handle.lock() {
            if lg.last().map_or(false, |s| s.contains("--- Finished ---") || s.contains("Error:")) {
                self.is_running = false;
                self.cache.save();
            }
        }
    }


    pub fn start_conversion(&mut self, ctx: egui::Context) {
        let input_raw = self.cache.last_input.trim().to_string();
        let output_raw = self.cache.last_output.trim().to_string();
        
        if input_raw.is_empty() { return; }

        let source = PathBuf::from(&input_raw);
        let out_folder = if output_raw.is_empty() { None } else { Some(PathBuf::from(&output_raw)) };
        
        let log = self.log_handle.clone();
        let in_ext = self.config.input_format.to_lowercase();
        let out_ext = self.config.output_format.to_lowercase();
        let width = self.config.width.parse::<u32>().unwrap_or(570);
        let height = self.config.height.parse::<u32>().unwrap_or(342);
        
        self.is_running = true;

        thread::spawn(move || {
            let mut tasks = Vec::new();

            if source.is_dir() {
                if let Ok(entries) = fs::read_dir(&source) {
                    for entry in entries.flatten() {
                        tasks.push(entry.path());
                    }
                }
            } else if source.is_file() {
                tasks.push(source);
            } else {
                Logger::log(&log, format!("Error: Path does not exist -> {}", source.display()));
            }

            for path in tasks {
                if path.is_dir() { continue; }
                let current_ext = path.extension().and_then(|s| s.to_str()).unwrap_or_default().to_lowercase();
                
                if current_ext == in_ext {
                    let dest = get_output_path(&path, &out_folder, &out_ext);
                    match process_single_file(&path, &dest, width, height, &out_ext) {
                        Ok(_) => Logger::log(&log, format!("Done: {}", path.file_name().unwrap().to_string_lossy())),
                        Err(e) => Logger::log(&log, format!("Fail: {} - {}", path.display(), e)),
                    }
                } else {
                    Logger::log(&log, format!("Skip: {} (not {})", path.file_name().unwrap().to_string_lossy(), in_ext));
                }
            }

            Logger::log(&log, "--- Finished ---");
            ctx.request_repaint();
        });
    }
}


impl App for ConverterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        render_ui(self, ctx);
    }
}
