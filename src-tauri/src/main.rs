// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod functions;
mod tree;
mod sets;
mod mutations;
mod random;

use std::{collections::HashMap, path::PathBuf, sync::{Mutex, OnceLock}, fs, io::Cursor, thread};

use image::{io::Reader, RgbaImage};
use rand::{distributions::Alphanumeric, Rng};
use regex::Regex;
use tauri::{Window, Manager};
use tree::{interpret, Genotype, NodeRef};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref PATHS: Mutex<HashMap<String, PathBuf>> = Mutex::new(HashMap::new());
}

#[tauri::command(async)]
fn get_phenotypes() -> Vec<[String; 2]> {
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
        let _ = image.unwrap().write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png);
        let base64 = base64::encode(&buf);

        let re = Regex::new(r"/(?<path>\w+).png").unwrap();
        let path_string = &path.unwrap().path().display().to_string();
        let capture = re.captures(path_string);
        match capture {
            Some(capture) => {
                images.push([format!("data:image/png;base64,{}", base64), capture["path"].to_owned()]);
            },
            None => images.push([format!("data:image/png;base64,{}", base64), "".to_string()])
        };
    }

    return images;
}

lazy_static! {
    pub static ref POPULATION: Mutex<HashMap<String, Mutex<Genotype>>> = Mutex::new(HashMap::new());
}

lazy_static! {
    pub static ref BEST_OF_POPULATION: Mutex<Vec<Mutex<Genotype>>> = Mutex::new(vec![]);
}

lazy_static! {
    pub static ref POPULATION_COUNTER: Mutex<i32> = Mutex::new(0);
}

static WINDOW: OnceLock<Window> = OnceLock::new();

fn update_population_counter(population_size: i32) {
    let mut population_counter = POPULATION_COUNTER.lock().unwrap();
    *population_counter += 1;
    if population_counter.clone() == population_size {
        let _ =WINDOW.get().unwrap().emit("loading", "");
    }
}

#[tauri::command(async)]
fn generate_population() {
    let population_size = 8;
    let grow_depth = 2;
    let grow_max_depth = 6;

    let mut threads = vec![];

    let mut borrow = POPULATION.lock().unwrap();
    *borrow = HashMap::new();
    drop(borrow);

    let borrow = PATHS.lock().unwrap();
    let path = borrow.get("data").unwrap();

    fs::remove_dir_all(path).unwrap();
    fs::create_dir(path).unwrap();

    for i in 0..population_size {
        let handle = thread::spawn(move || {
            let mut population = POPULATION.lock().unwrap();
            let str: String = rand::thread_rng().sample_iter(&Alphanumeric).take(7).map(char::from).collect();
            let genotype = Genotype::new();
            population.insert(str.clone(), Mutex::new(genotype.clone()));

            let out: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = interpret(genotype.clone().get_root());
            let _ = out.save(PATHS.lock().unwrap().get("data").unwrap().join(format!("{}.png", str.clone())));

            update_population_counter(population_size);
            println!("created population {}", i);
        });
        threads.push(handle);
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .setup(|app: &mut tauri::App| {
            let data = app.handle().path_resolver().app_data_dir().unwrap();
            let assets = app.path_resolver().resolve_resource("assets/").expect("failed to resolve resource");

            let window = app.get_window("main").unwrap();
            _ = WINDOW.set(window);

            PATHS.lock().unwrap().insert("data".to_string(), data.clone());
            PATHS.lock().unwrap().insert("assets".to_string(), assets.clone());

            // let inn = Reader::open(path.join("img2.png"))?.decode()?.to_rgba8();

            generate_population();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_phenotypes, generate_population])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
