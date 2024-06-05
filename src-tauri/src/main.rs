// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rodio::{Decoder, Source};
use std::sync::{Arc, Mutex};
use tauri::Manager;

#[tauri::command]
fn frontend_ready(app_state: tauri::State<'_, AppState>) {
    // TODO: Read filepath/filename from config (settings)
    let window = app_state.window.clone();
    std::thread::spawn(move || {
        if let Err(e) = play_audio("assets/audio/okaeri.mp3", window) {
            println!("Error: {}", e);
        }
    });
}

#[tauri::command]
fn get_image() -> Result<Vec<u8>, String> {
    std::fs::read("assets/images/catto.png").map_err(|e| e.to_string())
}

fn play_audio(
    file_path: &str,
    window: Arc<Mutex<tauri::Window>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Wait 200ms for nicer audio transition after frontend
    // has loaded
    std::thread::sleep(std::time::Duration::from_millis(200));

    // Create an output stream
    let (_stream, stream_handle) = rodio::OutputStream::try_default()?;

    let file = std::fs::File::open(file_path)?;
    let source = Decoder::new(std::io::BufReader::new(file))?.convert_samples();

    // TODO: Read f32 from config (settings)
    let source_with_volume = source.amplify(1.0);

    // Play the audio
    stream_handle.play_raw(source_with_volume)?;

    // Wait until the audio finishes playing
    // TODO: Read duration from config (settings)
    std::thread::sleep(std::time::Duration::from_secs(3));

    // Close app after playing audio
    close_app(window);

    Ok(())
}

fn close_app(window: Arc<Mutex<tauri::Window>>) {
    let window = window.lock().unwrap();
    window.close().unwrap();
}

#[derive(Clone)]
struct AppState {
    window: Arc<Mutex<tauri::Window>>,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();

            let app_state = AppState {
                window: Arc::new(Mutex::new(window)),
            };

            app.manage(app_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_image, frontend_ready])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/*
  ∧,,,∧
(  ̳• · • ̳)
/    づ♡ read if cute
*/
