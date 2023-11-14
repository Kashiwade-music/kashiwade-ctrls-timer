// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use device_query::{DeviceQuery, DeviceState, Keycode};
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::thread;
use tauri::{LogicalSize, Manager, Size};
use window_shadows::set_shadow;
mod startup_checker;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AlertConfig {
    time_sec: u32,
    color: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    dark_mode: bool,
    width: u32,
    height: u32,
    alert_config: Vec<AlertConfig>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn set_is_dark_mode(is_dark_mode: bool) {
    let mut startup_checker = startup_checker::StartupChecker::new();
    startup_checker.check();
    startup_checker.config.as_mut().unwrap().dark_mode = is_dark_mode;
    startup_checker.save_config();
}

#[tauri::command]
fn get_is_dark_mode() -> bool {
    let mut startup_checker = startup_checker::StartupChecker::new();
    startup_checker.check();
    startup_checker.config.as_ref().unwrap().dark_mode
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut startup_checker = startup_checker::StartupChecker::new();
    startup_checker.check();

    tauri::Builder::default()
        .setup(move |app| {
            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();

            let window = app.get_window("main").unwrap();
            window
                .set_size(Size::Logical(LogicalSize {
                    width: startup_checker.config.as_ref().unwrap().width.into(),
                    height: startup_checker.config.as_ref().unwrap().height.into(),
                }))
                .unwrap();
            window.set_title("KCtrlSTimer").unwrap();

            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(window, true).unwrap();

            let app_handle = app.handle();
            app_handle
                .emit_all(
                    "is-dark-mode",
                    Some(startup_checker.config.as_ref().unwrap().dark_mode),
                )
                .expect("failed to emit");
            println!(
                "config: {:?}",
                startup_checker.config.as_ref().unwrap().dark_mode
            );
            thread::spawn(move || {
                let mut counter = 0;
                loop {
                    thread::sleep(std::time::Duration::from_secs(1));
                    match rx.try_recv() {
                        Ok(command) => {
                            if command == "reset" {
                                counter = 0;
                                app_handle
                                    .emit_all("set-timer", Some("00:00".to_string()))
                                    .expect("failed to emit");
                            }
                        }
                        Err(_) => {}
                    }
                    counter += 1;
                    app_handle
                        .emit_all(
                            "set-timer",
                            Some(format!("{:02}:{:02}", counter / 60, counter % 60)),
                        )
                        .expect("failed to emit");
                }
            });

            let app_handle = app.handle();
            thread::spawn(move || {
                let device_state = DeviceState::new();
                let mut keys = device_state.get_keys();
                loop {
                    let keys_now = device_state.get_keys();
                    if keys_now != keys {
                        keys = keys_now;
                        // detect ctrl+s
                        if keys.contains(&Keycode::LControl) && keys.contains(&Keycode::S) {
                            tx.send("reset").unwrap();
                            app_handle
                                .emit_all("should-timer-reset", Some(true))
                                .expect("failed to emit");
                        }
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![set_is_dark_mode, get_is_dark_mode])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
