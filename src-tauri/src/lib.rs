use std::error::Error;

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use tuicher_rs::config::get_config;

use crate::{
    action::invoke_result_action, apps::setup_apps_indexing, config::invoke_get_config,
    listener::setup_keyboard_listener, search::invoke_search,
};

pub mod action;
pub mod apps;
pub mod config;
pub mod listener;
pub mod plugins;
pub mod search;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn Error>> {
    // Fix crashing when using webview on wayland/nvidia
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    // std::env::set_var("GDK_BACKEND", "x11");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            invoke_get_config,
            invoke_search,
            invoke_result_action
        ])
        .setup(|app| {
            let app_clone = app.app_handle().to_owned();
            let app_thread_clone = app_clone.clone();

            setup_keyboard_listener(app_thread_clone);

            setup_apps_indexing()?;

            let window = app_clone
                .get_webview_window("main")
                .expect("Failed to get window");

            window.close().unwrap();

            let config = get_config().expect("Failed to get config");

            WebviewWindowBuilder::new(&app_clone, "tuicher", WebviewUrl::App("index.html".into()))
                .title("tuicher")
                .center()
                .always_on_top(true)
                .decorations(false)
                .inner_size(config.width as f64, config.height as f64)
                .resizable(false)
                .build()
                .expect("Failed to build window");

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                if window.label() == "tuicher" {
                    api.prevent_close();
                    window.hide().expect("Failed to hide window");
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
