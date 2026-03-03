mod engine;
mod view;
mod events;

use eframe::egui;
use std::io;


pub fn render_path_input(ui: &mut egui::Ui, label: &str, path_str: &mut String) {
    ui.horizontal(|ui| {
        ui.label(label);
        
        let text_edit = egui::TextEdit::singleline(path_str)
            .hint_text("Enter path...")
            .desired_width(ui.available_width() - 20.0)
            .lock_focus(true);

        let response = ui.add(text_edit);

        if response.has_focus() {
            process_autocomplete(ui, &response, path_str, label);
        }
    });
}


fn process_autocomplete(ui: &mut egui::Ui, response: &egui::Response, path_str: &mut String, label: &str) {
    let keys = events::consume_navigation_keys(ui);
    let (dir_to_scan, stub) = engine::get_dir_and_stub(path_str);
    
    match engine::fetch_suggestions(&dir_to_scan, &stub) {
        Ok(suggestions) if !suggestions.is_empty() => {
            let mut idx = ui.data_mut(|d| *d.get_temp_mut_or_default::<isize>(response.id));

            if keys.up { idx -= 1; }
            if keys.down { idx += 1; }
            
            let current_idx = idx.rem_euclid(suggestions.len() as isize) as usize;
            ui.data_mut(|d| d.insert_temp(response.id, idx));

            if keys.tab_or_enter {
                let item = &suggestions[current_idx];
                *path_str = engine::build_new_path(&dir_to_scan, &item.name, item.is_dir);
                events::move_cursor_to_end(ui.ctx(), response.id, path_str);
            }

            if keys.tab_or_enter || keys.up || keys.down {
                events::apply_focus_lock(ui, response.id);
            }

            view::render_suggestions_window(ui, response.rect, path_str, &dir_to_scan, &suggestions, current_idx, label);
        }
        Err(e) if dir_to_scan.exists() => {
            render_error_tooltip(response, &e); // UI removed from args as response has access to ctx
        }
        _ => {} 
    }
}


fn render_error_tooltip(response: &egui::Response, err: &io::Error) {
    // Clone is necessary because on_hover_ui consumes self
    response.clone().on_hover_ui(|ui| {
        ui.label(egui::RichText::new(format!("⚠ {}", err)).color(ui.visuals().error_fg_color));
    });
}
