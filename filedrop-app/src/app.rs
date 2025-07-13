// src/app.rs
use crate::gui::{receiver_ui::render_receiver_ui, sender_ui::render_sender_ui};

pub fn run() {
    eframe::run_native(
        "FileDrop",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(FileDropApp::default())),
    ).unwrap();
}

#[derive(Default)]
struct FileDropApp {
    tab: Tab,
}

#[derive(PartialEq)]
enum Tab {
    Send,
    Receive,
}

impl Default for Tab {
    fn default() -> Self {
        Tab::Send
    }
}

impl eframe::App for FileDropApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(self.tab == Tab::Send, "📤 Send").clicked() {
                    self.tab = Tab::Send;
                }
                if ui.selectable_label(self.tab == Tab::Receive, "📥 Receive").clicked() {
                    self.tab = Tab::Receive;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.tab {
                Tab::Send => render_sender_ui(ui),
                Tab::Receive => render_receiver_ui(ui),
            }
        });
    }
}
