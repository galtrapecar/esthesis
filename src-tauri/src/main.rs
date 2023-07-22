// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod functions;
mod tree;
mod sets;
mod mutations;
mod random;

use std::{collections::HashMap, path::PathBuf, sync::Mutex};

use tree::{interpret, Genotype};

use lazy_static::lazy_static;

use crate::mutations::image_to_function;

lazy_static! {
    pub static ref PATHS: Mutex<HashMap<String, PathBuf>> = Mutex::new(HashMap::new());
}

fn test() {
    
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
                let mut genotype = Genotype::new();

                println!("{}", genotype.size());

                let out: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = interpret(genotype.clone().get_root());
                out.save(data.join(format!("out1.png")))?;

                genotype.mutate();
                genotype.mutate();
                genotype.mutate();
                genotype.mutate();


                println!("{}", genotype.size());

                let out: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = interpret(genotype.get_root());
                out.save(data.join(format!("out2.png")))?;

                break;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
