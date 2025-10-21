use std::{env, error::Error, process::Command};

use sniffer_rs::sniffer::Sniffer;
use tauri::Window;
use tuicher_rs::result::{Action, Session, TUIResult};

pub fn get_session_results(
    sniffer: &Sniffer,
    search_text: &str,
) -> Result<Vec<TUIResult>, Box<dyn Error>> {
    let mut results: Vec<TUIResult> = vec![];

    if sniffer.matches("shutdown/poweroff", search_text) {
        results.push(
            TUIResult::new("Shutdown", "session-manager")
                .set_action(Action::Session(Session::Shutdown))
                .set_secondary_text("Shutdown the computer"),
        );
    }

    if sniffer.matches("restart/reboot", search_text) {
        results.push(
            TUIResult::new("Restart", "session-manager")
                .set_action(Action::Session(Session::Restart))
                .set_secondary_text("Restart the computer"),
        );
    }

    if sniffer.matches("suspend", search_text) {
        results.push(
            TUIResult::new("Suspend", "session-manager")
                .set_action(Action::Session(Session::Suspend))
                .set_secondary_text("Suspend the computer"),
        );
    }

    if sniffer.matches("logout", search_text) {
        results.push(
            TUIResult::new("Logout", "session-manager")
                .set_action(Action::Session(Session::Logout))
                .set_secondary_text("Logout of your desktop environment/window manager"),
        );
    }

    Ok(results)
}

pub fn on_session_action(session: Session, window: Window) -> Result<(), Box<dyn Error>> {
    match session {
        Session::Shutdown => {
            Command::new("systemctl").arg("poweroff").spawn()?;
        }
        Session::Restart => {
            Command::new("systemctl").arg("reboot").spawn()?;
        }
        Session::Suspend => {
            Command::new("systemctl").arg("suspend").spawn()?;
        }
        Session::Logout => {
            let desktop_env = env::var("XDG_CURRENT_DESKTOP")?.to_lowercase();

            match desktop_env.as_str() {
                "hyprland" => {
                    Command::new("hyprctl")
                        .arg("dispatch")
                        .arg("exit")
                        .spawn()?;
                }
                "kde" => {
                    Command::new("qdbus6")
                        .args(["org.kde.Shutdown", "/Shutdown", "org.kde.Shutdown.logout"])
                        .spawn()?;
                }
                _ => {
                    Command::new("notify-send")
                        .arg("'Missing Desktop Environment. Please Report'")
                        .spawn()?;
                }
            }
        }
    }

    window.close()?;
    Ok(())
}
