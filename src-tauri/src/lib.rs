use tauri::Manager;
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::{CommandEvent, CommandChild};
use std::sync::Mutex;

struct ChatServerChild(Mutex<Option<CommandChild>>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let handle = app.handle().clone();
            let sidecar_command = handle
                .shell()
                .sidecar("chat-server")
                .expect("Failed to create chat-server sidecar command");
            let (mut rx, child) = sidecar_command
                .spawn()
                .expect("Failed to spawn chat-server");

            // child를 app state에 저장 → drop 방지
            app.manage(ChatServerChild(Mutex::new(Some(child))));

            tauri::async_runtime::spawn(async move {
                while let Some(event) = rx.recv().await {
                    match event {
                        CommandEvent::Stdout(line) => {
                            let line = String::from_utf8_lossy(&line);
                            println!("[chat-server] {}", line);
                        }
                        CommandEvent::Stderr(line) => {
                            let line = String::from_utf8_lossy(&line);
                            eprintln!("[chat-server ERR] {}", line);
                        }
                        CommandEvent::Error(e) => {
                            eprintln!("[chat-server] Error: {}", e);
                        }
                        CommandEvent::Terminated(status) => {
                            println!("[chat-server] Terminated: {:?}", status);
                            break;
                        }
                        _ => {}
                    }
                }
            });
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                // child를 꺼내서 명시적으로 kill
                if let Some(state) = window.try_state::<ChatServerChild>() {
                    if let Ok(mut guard) = state.0.lock() {
                        if let Some(child) = guard.take() {
                            let _ = child.kill();
                            println!("[chat-server] Killed on window destroy");
                        }
                    }
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
