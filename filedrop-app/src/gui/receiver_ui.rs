use egui::{Ui, ProgressBar};
use std::sync::{Arc, Mutex};
use crate::core::{receive, types::{TransferState, SharedState}};

static mut SHARED: Option<SharedState> = None;

pub fn render_receiver_ui(ui: &mut Ui) {
    let state = unsafe {
        SHARED.get_or_insert_with(|| Arc::new(Mutex::new(TransferState::default()))).clone()
    };

    if ui.button("🔄 Start Receiver (port 7878)").clicked() {
        receive::receive_file(7878, state.clone());
    }

    let s = state.lock().unwrap();

    if !s.filename.is_empty() {
        ui.label(format!("📥 Receiving: {}", s.filename));
        let progress = if s.filesize > 0 {
            s.transferred as f32 / s.filesize as f32
        } else {
            0.0
        };
        ui.add(ProgressBar::new(progress).text(format!("{:.1}%", progress * 100.0)));
        if s.complete {
            ui.colored_label(egui::Color32::GREEN, "✅ File Received!");
        }
        if let Some(err) = &s.error {
            ui.colored_label(egui::Color32::RED, format!("❌ {}", err));
        }
    }
}
