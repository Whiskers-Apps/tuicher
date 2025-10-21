use tuicher_rs::config::{get_config, Config};

#[tauri::command]
pub fn invoke_get_config() -> Result<Config, String> {
    Ok(get_config().map_err(|_| "Failed to get config")?)
}
