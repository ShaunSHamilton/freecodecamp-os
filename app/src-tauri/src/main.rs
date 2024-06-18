// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{AppHandle, Manager, Window};
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

#[tauri::command]
fn start_server(app: AppHandle, _window: Window) -> String {
    println!("Starting server");
    let test_command = app.shell().sidecar("test_bin").unwrap();
    let (mut rx, mut _child) = test_command.spawn().expect("Failed to spawn test");

    tauri::async_runtime::spawn(async move {
        // read events such as stdout
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line) = event {
                // window
                //     .emit("message", Some(format!("'{:?}'", line)))
                //     .expect("failed to emit event");
                // // write to stdin
                // child.write("message\n".as_bytes()).unwrap();
                println!("{:?}", line);
            }
        }
    });

    let code_server_command = app.shell().sidecar("code-server").unwrap();
    let (mut _rx, mut _child) = code_server_command
        .spawn()
        .expect("Failed to spawn code-server");
    "Server started".to_string()
}

#[tauri::command]
async fn check_server() -> Result<(), String> {
    let res = reqwest::get("http://localhost:8080/healthz").await;
    match res {
        Ok(response) => {
            if response.status().is_success() {
                Ok(())
            } else {
                Err("Server is not running".to_string())
            }
        }
        Err(_) => Err("Server is not running".to_string()),
    }
}

#[tauri::command]
async fn stop_server(app: AppHandle, _window: Window) -> String {
    todo!()
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            start_server,
            stop_server,
            check_server
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
