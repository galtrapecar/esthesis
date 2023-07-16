// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod functions;
mod tree;
mod sets;

use image::io::Reader;
use crate::functions::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            let path = app.handle().path_resolver().app_data_dir().unwrap();
            let mut out = Reader::open(path.join("img.webp"))?.decode()?.to_rgba8();
            out = haar(out);
            out = flip_horizontal(out);
            out.save(path.join("out.png"))?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
