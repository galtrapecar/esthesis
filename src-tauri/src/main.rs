// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod functions;
mod tree;
mod sets;

use std::{collections::HashMap, path::PathBuf, sync::Mutex};

use tree::{grow, interpret};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref PATHS: Mutex<HashMap<String, PathBuf>> = Mutex::new(HashMap::new());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            let data = app.handle().path_resolver().app_data_dir().unwrap();
            let assets = app.path_resolver().resolve_resource("assets/").expect("failed to resolve resource");

            PATHS.lock().unwrap().insert("data".to_string(), data.clone());
            PATHS.lock().unwrap().insert("assets".to_string(), assets.clone());

            // let inn = Reader::open(path.join("img2.png"))?.decode()?.to_rgba8();

            for i in 0..10 {
                let root = grow(2, 20);

                let out: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = interpret(root);
                out.save(data.join(format!("out{}.png", i)))?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
