use eframe::egui;
use eframe::egui::text::{CCursor, CCursorRange};


pub struct InputResult {
    pub tab_or_enter: bool,
    pub up: bool,
    pub down: bool,
}


pub fn consume_navigation_keys(ui: &mut egui::Ui) -> InputResult {
    InputResult {
        tab_or_enter: ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Tab))
                    || ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Enter)),
        up: ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp)),
        down: ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown)),
    }
}


pub fn move_cursor_to_end(ui: &egui::Context, id: egui::Id, text: &str) {
    if let Some(mut state) = egui::TextEdit::load_state(ui, id) {
        let char_count = text.chars().count();
        let ccursor = CCursor::new(char_count);
        state.cursor.set_char_range(Some(CCursorRange::one(ccursor)));
        state.store(ui, id);
    }
}


pub fn apply_focus_lock(ui: &mut egui::Ui, id: egui::Id) {
    ui.memory_mut(|m| {
        m.request_focus(id);
        m.set_focus_lock_filter(id, egui::EventFilter {
            tab: true,
            horizontal_arrows: false,
            vertical_arrows: true,
            escape: false,
        });
    });
}
