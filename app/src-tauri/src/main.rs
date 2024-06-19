// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::time::Duration;
use tauri::async_runtime::Receiver;
use tauri::{AppHandle, Manager, Window};
use tauri_plugin_shell::process::CommandEvent;
// use tauri_plugin_shell::ShellExt;
use tokio::time::timeout;

#[tauri::command]
async fn start_server(_app: AppHandle, _window: Window) -> Result<(), String> {
    println!("Starting server");
    Ok(())
}

async fn _process_events(rx: &mut Receiver<CommandEvent>) -> Result<(), String> {
    let result = timeout(Duration::from_secs(3), async {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    let mes = line.escape_ascii().to_string();
                    println!("{}", mes);
                }
                CommandEvent::Stderr(line) => {
                    let mes = line.escape_ascii().to_string();
                    return Err(mes);
                }
                _ => {}
            }
        }
        Ok(())
    })
    .await;

    match result {
        Ok(res) => res,
        Err(_) => Ok(()), // Timeout occurred
    }
}

#[tauri::command]
async fn check_server() -> Result<(), String> {
    todo!()
}

#[tauri::command]
async fn stop_server(_app: AppHandle, _window: Window) -> String {
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
