// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate rocket;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::NamedFile;
use rocket::http::Header;
use rocket::Response;
use rocket::{Request, Rocket};
use rodio::{Decoder, Source};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tauri::Manager;

#[get("/image")]
async fn image() -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/images/catto.png"))
        .await
        .ok()
}

// probably not a great idea
struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, OPTIONS",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        ));
    }
}

fn play(window: Arc<Mutex<tauri::Window>>) {
    // TODO: Read filepath/filename from config (settings)
    if let Err(e) = play_audio("assets/audio/okaeri.mp3", window) {
        println!("Error: {}", e);
    }
}

fn play_audio(
    file_path: &str,
    window: Arc<Mutex<tauri::Window>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // dev is lazy so prob a TODO here
    // wait for the DOM to load via seconds
    // instead of telling tauri from the frontend
    // that it has been loaded
    // lmao, suck my dick uwu
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Create an output stream
    let (_stream, stream_handle) = rodio::OutputStream::try_default()?;

    // Load the sound file
    let file = std::fs::File::open(file_path)?;
    let source = Decoder::new(std::io::BufReader::new(file))?.convert_samples();

    // Lower the volume
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

#[rocket::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::spawn(async move {
                let _rocket = rocket::build() // .configure(rocket::Config::figment().merge(("port", 133769)))
                    .attach(CORS)
                    .mount("/", routes![image])
                    .launch()
                    .await;
            });

            let window = app.get_window("main").unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();

            let app_state = AppState {
                window: Arc::new(Mutex::new(window)),
            };

            {
                let app_state = app_state.clone();
                std::thread::spawn(move || {
                    play(app_state.window);
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/*
  ∧,,,∧
(  ̳• · • ̳)
/    づ♡ read if cute
*/
