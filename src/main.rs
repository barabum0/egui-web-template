#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

struct MyApp;

impl Default for MyApp {
    fn default() -> Self {
        Self
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, egui on the web!");
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Run natively
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My Egui Web App",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    ).unwrap();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), eframe::wasm_bindgen::JsValue> {
    // Run in the web browser
    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "the_canvas_id", // Canvas ID in the HTML
        web_options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
    .await?;
    Ok(())
}