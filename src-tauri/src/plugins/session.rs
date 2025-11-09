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
            TUIResult::new("Shutdown", "shutdown").set_action(Action::Session(Session::Shutdown)),
        );
    }

    if sniffer.matches("restart/reboot", search_text) {
        results.push(
            TUIResult::new("Restart", "restart").set_action(Action::Session(Session::Restart)),
        );
    }

    if sniffer.matches("suspend", search_text) {
        results.push(
            TUIResult::new("Suspend", "suspend").set_action(Action::Session(Session::Suspend)),
        );
    }

    if sniffer.matches("logout", search_text) {
        results
            .push(TUIResult::new("Logout", "logout").set_action(Action::Session(Session::Logout)));
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
                        .arg("Missing Config for This Environment")
                        .spawn()?;
                }
            }
        }
    }

    window.close()?;
    Ok(())
}
