// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod functions;
mod tree;
mod sets;
mod mutations;
mod random;

use std::{collections::HashMap, path::PathBuf, sync::Mutex, fs, io::Cursor};

use image::{io::Reader, RgbaImage};
use tree::{interpret, Genotype, NodeRef};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref PATHS: Mutex<HashMap<String, PathBuf>> = Mutex::new(HashMap::new());
}

#[tauri::command(async)]
fn get_phenotypes() -> Vec<String> {
    let mut images = vec![];
    let paths = fs::read_dir(PATHS.lock().unwrap().get("data").unwrap()).unwrap();

    for path in paths {
        let image = match Reader::open(path.as_ref().unwrap().path()) {
            Ok(file) => {
                match file.decode() {
                    Ok(decode) => Some(decode.to_rgba8()),
                    Err(_err) => None
                }
            },
            Err(_err) => None
        };

        if image.is_none() { continue; };

        let mut buf: Vec<u8> = vec![];
        image.unwrap().write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png);
        let base64 = base64::encode(&buf);

        images.push(format!("data:image/png;base64,{}", base64));
        println!("Name: {}", path.unwrap().path().display());
    }

    return images;
}

lazy_static! {
    pub static ref POPULATION: Mutex<Vec<NodeRef>> = Mutex::new(vec![]);
}

#[tauri::command(async)]
fn evolution_run() {
    let population_size = 6;
    let grow_depth = 2;
    let grow_max_depth = 6;
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

            for i in 0..4 {
                let mut genotype = Genotype::new();

                println!("{}", genotype.size());

                let out: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = interpret(genotype.clone().get_root());
                out.save(data.join(format!("out{}.png", i)))?;

                genotype.mutate();
                genotype.mutate();


                println!("{}", genotype.size());

                let out: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = interpret(genotype.get_root());
                out.save(data.join(format!("out{}m.png", i)))?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_phenotypes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
