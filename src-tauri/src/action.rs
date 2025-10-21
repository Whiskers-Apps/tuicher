use std::{
    error::Error,
    process::{Command, Stdio},
    thread,
};

use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder, Window};
use tuicher_rs::result::{Action, CopyImage, CopyText, OpenApp, OpenURL};

use crate::plugins::{bookmarks::on_bookmark_action, session::on_session_action};

#[tauri::command]
pub fn invoke_result_action(action: Action, app: AppHandle, window: Window) -> Result<(), String> {
    match action {
        Action::OpenApp(open_app) => {
            on_open_app(open_app, window.clone()).map_err(|e| e.to_string())?;
        }
        Action::OpenFile(_open_file) => {}
        Action::OpenURL(open_url) => {
            on_open_url(open_url, window.clone()).map_err(|e| e.to_string())?;
        }
        Action::CopyText(copy_text) => {
            on_copy_text(copy_text, window.clone()).map_err(|e| e.to_string())?;
        }
        Action::CopyImage(copy_image) => {
            on_copy_image(copy_image, window.clone()).map_err(|e| e.to_string())?;
        }
        Action::ShowResults(_show_results) => {}
        Action::OpenSettings => {
            on_open_settings(app.clone(), window.clone()).map_err(|e| e.to_string())?;
        }
        Action::Session(session) => {
            on_session_action(session, window.clone()).map_err(|e| e.to_string())?;
        }
        Action::Bookmark(bookmark) => {
            on_bookmark_action(bookmark, window.clone()).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

fn on_open_app(action: OpenApp, window: Window) -> Result<(), Box<dyn Error>> {
    let file_name = action
        .path
        .clone()
        .file_name()
        .ok_or_else(|| "Failed to get file name")?
        .to_owned();

    thread::spawn(move || {
        Command::new("gtk-launch")
            .arg(file_name)
            .spawn()
            .expect("Failed to open app");
    });

    window.close()?;

    Ok(())
}

fn on_open_url(action: OpenURL, window: Window) -> Result<(), Box<dyn Error>> {
    open::that_detached(&action.url)?;
    window.close()?;
    Ok(())
}

fn on_copy_text(action: CopyText, window: Window) -> Result<(), Box<dyn Error>> {
    thread::spawn(move || {
        let text_parts: Vec<&str> = action.text.split(" ").collect();

        Command::new("wl-copy")
            .args(text_parts)
            .spawn()
            .expect("Error copying to clipboard");
    });

    window.close()?;
    Ok(())
}

fn on_copy_image(copy_image: CopyImage, window: Window) -> Result<(), Box<dyn Error>> {
    thread::spawn(move || {
        Command::new("cat")
            .arg(format!("'{}'", copy_image.path.display()))
            .args(["|", "wl-copy", "-t", "image/png"])
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .expect("Error copying image");
    });

    window.close()?;
    Ok(())
}

fn on_open_settings(app: AppHandle, window: Window) -> Result<(), Box<dyn Error>> {
    WebviewWindowBuilder::new(
        &app,
        "tuicher-settings",
        WebviewUrl::App("/settings".into()),
    )
    .title("Settings")
    .center()
    .build()?;

    window.close()?;

    Ok(())
}
