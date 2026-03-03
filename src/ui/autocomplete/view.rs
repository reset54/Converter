use eframe::egui;
use std::path::Path;
use super::engine::{self, SuggestionItem};


pub fn render_suggestions_window(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    path_str: &mut String,
    dir_to_scan: &Path,
    suggestions: &[SuggestionItem],
    current_idx: usize,
    label: &str
) {
    let max_height = 300.0;
    
    egui::Window::new(format!("Suggestions for {}", label))
        .fixed_pos(rect.left_bottom())
        .title_bar(false)
        .resizable(false)
        .constrain(true)
        .show(ui.ctx(), |ui| {
            egui::ScrollArea::vertical()
                .max_height(max_height)
                .show(ui, |ui| {
                    for (i, item) in suggestions.iter().enumerate() {
                        let is_selected = i == (current_idx % suggestions.len());
                        
                        let response = ui.selectable_label(is_selected, &item.name);
                        
                        // Vertical sync: scroll to active element
                        if is_selected {
                            response.scroll_to_me(Some(egui::Align::Center));
                        }
                        
                        if response.clicked() {
                            *path_str = engine::build_new_path(dir_to_scan, &item.name, item.is_dir);
                        }
                    }
                });
        });
}
