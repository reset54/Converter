use eframe::egui;
use crate::app::ConverterApp;
use crate::config::ConverterConfig;
use crate::ui::autocomplete;


fn render_path_row(ui: &mut egui::Ui, label: &str, path: &mut String, is_folder_only: bool) {
    ui.horizontal(|ui| {
        let btn_text = if is_folder_only { "Folder…" } else { "File…" };

        if ui.button(btn_text).clicked() {
            let dialog = rfd::FileDialog::new();
            let result = if is_folder_only {
                dialog.pick_folder()
            } else {
                dialog.pick_file()
            };

            if let Some(p) = result {
                *path = p.display().to_string();
            }
        }

        autocomplete::render_path_input(ui, label, path);
    });
}


pub fn render_path_settings(ui: &mut egui::Ui, app: &mut ConverterApp) {
    ui.group(|ui| {
        ui.label("Path Settings:");
        ui.add_space(5.0);

        render_path_row(ui, "In:", &mut app.cache.last_input, false);
        ui.add_space(8.0);
        render_path_row(ui, "Out:", &mut app.cache.last_output, true);
    });
}


pub fn render_controls(ui: &mut egui::Ui, app: &mut ConverterApp, ctx: &egui::Context) {
    ui.horizontal(|ui| {
        let has_input = !app.cache.last_input.trim().is_empty();
        let can_run = has_input && !app.is_running;

        let run_btn = egui::Button::new("Process All").min_size([140.0, 40.0].into());

        if ui.add_enabled(can_run, run_btn).clicked() {
            app.start_conversion(ctx.clone());
        }

        if ui.button("Reset Paths").clicked() {
            app.cache.last_input.clear();
            app.cache.last_output.clear();
        }
    });

    if app.is_running {
        app.check_completion();
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.spinner();
            ui.label("Processing images...");
        });
    }
}


pub fn render_format_settings(ui: &mut egui::Ui, app: &mut ConverterApp) {
    ui.group(|ui| {
        ui.label("Format Settings:");
        ui.horizontal(|ui| {
            ui.label("From:");
            render_combobox(ui, "src_fmt", &mut app.config.input_format);
            ui.add_space(30.0);
            ui.label("To:");
            render_combobox(ui, "dst_fmt", &mut app.config.output_format);
        });
    });
}


fn render_combobox(ui: &mut egui::Ui, id: &str, selected: &mut String) {
    egui::ComboBox::from_id_source(id)
        .selected_text(selected.as_str())
        .width(100.0)
        .show_ui(ui, |ui| {
            for fmt in ConverterConfig::available_formats() {
                ui.selectable_value(selected, fmt.to_string(), fmt);
            }
        });
}


pub fn render_dimension_settings(ui: &mut egui::Ui, app: &mut ConverterApp) {
    ui.group(|ui| {
        ui.label("Target Dimensions (px):");
        ui.horizontal(|ui| {
            ui.label("W:");
            ui.add(egui::TextEdit::singleline(&mut app.config.width).desired_width(70.0));
            ui.add_space(20.0);
            ui.label("H:");
            ui.add(egui::TextEdit::singleline(&mut app.config.height).desired_width(70.0));
        });
    });
}


pub fn render_log_area(ui: &mut egui::Ui, app: &mut ConverterApp) {
    ui.separator();
    ui.label("Process Log:");

    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .show(ui, |ui| {
            ui.set_min_height(150.0);
            if let Ok(lg) = app.log_handle.lock() {
                for line in lg.iter() {
                    ui.monospace(line);
                }
            }
        });
}
