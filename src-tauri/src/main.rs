// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod functions;
mod tree;
mod sets;

use image::io::Reader;
use tree::grow;
use crate::functions::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            let path = app.handle().path_resolver().app_data_dir().unwrap();
            let mut out = Reader::open(path.join("img2.png"))?.decode()?.to_rgba8();
            out = tile(out.clone(), 0.5, image::imageops::FilterType::Nearest);
            out.save(path.join("out.png"))?;

            let root = grow(2, 6);
            println!("{:?}", root);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
