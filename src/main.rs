#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::epaint::vec2;
use tools::{APP_ICON, APP_NAME,App};


#[tokio::main]
async fn main() {
    let icon = image::load_from_memory(APP_ICON).unwrap();
    let native_options = eframe::NativeOptions {
        maximized: true,
        // initial_window_size: Some(vec2(1920.0, 1080.0)),
        max_window_size: Some(vec2(1960.0, 1080.0)),
        min_window_size: Some(vec2(1366.0, 768.0)),
        // fullscreen:true,
        icon_data: Some(eframe::IconData {
            width: icon.width(),
            height: icon.height(),
            rgba: icon.into_rgba8().into_raw(),
        }),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        APP_NAME,
        native_options,
        
        Box::new(|cc|{
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(App::new(cc))
        } 
    ))
    .unwrap();
}
