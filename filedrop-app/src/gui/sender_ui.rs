use egui::{Ui, ProgressBar};
use std::sync::{Arc, Mutex};

use crate::core::{send, types::{TransferState, SharedState}};
use rfd::FileDialog;

static mut SHARED: Option<SharedState> = None;

pub fn render_sender_ui(ui: &mut Ui) {
    let state = unsafe {
        SHARED.get_or_insert_with(|| Arc::new(Mutex::new(TransferState::default()))).clone()
    };

    if ui.button("📁 Select File to Send").clicked() {
        if let Some(path) = FileDialog::new().pick_file() {
            let path_str = path.to_string_lossy().to_string();
            send::send_file(path_str, "127.0.0.1:7878".into(), state.clone());
        }
    }

    let s = state.lock().unwrap();

    if !s.filename.is_empty() {
        ui.label(format!("📦 File: {}", s.filename));
        let progress = if s.filesize > 0 {
            s.transferred as f32 / s.filesize as f32
        } else {
            0.0
        };
        ui.add(ProgressBar::new(progress).text(format!("{:.1}%", progress * 100.0)));
        if s.complete {
            ui.colored_label(egui::Color32::GREEN, "✅ Sent!");
        }
        if let Some(err) = &s.error {
            ui.colored_label(egui::Color32::RED, format!("❌ {}", err));
        }
    }
}
