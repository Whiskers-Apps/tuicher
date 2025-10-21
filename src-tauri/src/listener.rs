use std::{
    fs,
    io::Write,
    os::unix::net::{UnixListener, UnixStream},
    path::Path,
    process::{exit, Command},
    thread,
};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};

const SOCKET_PATH: &str = "/tmp/tuicher.sock";

pub fn setup_keyboard_listener(app: AppHandle) {
    if Path::new(SOCKET_PATH).exists() {
        match UnixStream::connect(SOCKET_PATH) {
            Ok(mut stream) => {
                stream.write_all(b"").expect("Failed to write to stream");
                exit(0);
            }
            Err(_) => {
                fs::remove_file(Path::new(SOCKET_PATH)).expect("Failed to remove socket file");

                Command::new("tuicher")
                    .spawn()
                    .expect("Failed to run launcher");

                exit(0)
            }
        }
    }

    let app_thread_clone = app.clone();

    thread::spawn(move || {
        let path = Path::new(SOCKET_PATH);

        if path.exists() {
            std::fs::remove_file(&path).expect("Failed to remove temp file");
        }

        let listener = UnixListener::bind(SOCKET_PATH).expect("Failed to bind listener");

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    stream
                        .write_all(b"ack")
                        .expect("Failed to send acknowledge");

                    let window = app
                        .get_webview_window("tuicher")
                        .expect("Failed to get window");

                    window.show().expect("Failed to show window");

                    #[derive(Serialize, Deserialize, Clone)]
                    struct Payload {}

                    let _ = app_thread_clone.emit("window-show", Payload {});
                }
                Err(_) => {
                    eprintln!("Failed to connect listener");
                    exit(1);
                }
            }
        }
    });
}
