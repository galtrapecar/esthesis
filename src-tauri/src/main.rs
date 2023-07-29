// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod functions;
mod tree;
mod sets;
mod mutations;
mod random;
mod display;

use std::{collections::HashMap, path::PathBuf, sync::{Mutex, OnceLock}, fs, io::Cursor, thread, borrow::BorrowMut, vec};

use image::io::Reader;
use rand::{distributions::Alphanumeric, Rng};
use random::_random_image;
use regex::Regex;
use sets::IMAGE_TERMINAL_SET;
use tauri::{Window, Manager};
use tree::{interpret, Genotype};

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
    pub static ref BEST_OF_POPULATION: Mutex<HashMap<String, Mutex<Genotype>>> = Mutex::new(HashMap::new());
}

lazy_static! {
    pub static ref NEW_POPULATION: Mutex<Vec<Mutex<Genotype>>>  = Mutex::new(vec![]);
}

lazy_static! {
    pub static ref POPULATION_COUNTER: Mutex<i32> = Mutex::new(0);
}

lazy_static! {
    pub static ref BEST_OF_POPULATION_COUNTER: Mutex<i32> = Mutex::new(0);
}

static WINDOW: OnceLock<Window> = OnceLock::new();

fn update_best_of_population_counter(population_size: i32) {
    let mut best_of_population_counter = BEST_OF_POPULATION_COUNTER.lock().unwrap();
    *best_of_population_counter += 1;
    if best_of_population_counter.clone() == population_size {
        let _ = WINDOW.get().unwrap().emit("loading", "");
        *best_of_population_counter = 0;

        let mut population = POPULATION.lock().unwrap();
        let mut best_of_population = BEST_OF_POPULATION.lock().unwrap();

        population.clear();
        for (k, v) in &*best_of_population {
            let v = v.lock().unwrap();
            population.insert(k.clone(), Mutex::new(v.clone()));
            drop(v);
        }
        best_of_population.clear();
    }
}

fn update_population_counter(population_size: i32) {
    let mut population_counter = POPULATION_COUNTER.lock().unwrap();
    *population_counter += 1;
    if population_counter.clone() == population_size {
        let _ = WINDOW.get().unwrap().emit("loading", "");
        *population_counter = 0;
    }
}

#[tauri::command(async)]
fn generate_population() {
    let population_size = 8;

    let mut threads = vec![];

    let mut borrow = POPULATION.lock().unwrap();
    *borrow = HashMap::new();
    drop(borrow);

    let borrow = PATHS.lock().unwrap();
    let path = borrow.get("data").unwrap();

    fs::remove_dir_all(path).unwrap();
    fs::create_dir(path).unwrap();

    for i in 0..1 {
        let handle = thread::spawn(move || {
            let mut population = POPULATION.lock().unwrap();
            let str = random_string();
            let genotype = Genotype::new();
            population.insert(str.clone(), Mutex::new(genotype.clone()));

            let out: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = interpret(genotype.clone().get_root());
            let _ = out.save(PATHS.lock().unwrap().get("data").unwrap().join(format!("{}.png", str.clone())));

            update_population_counter(population_size);
            println!("created population {}", i);
            println!("{}", genotype.get_root());
        });
        threads.push(handle);
    }
}

// Thanks to https://stackoverflow.com/a/52367953
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn random_string() -> String {
    rand::thread_rng().sample_iter(&Alphanumeric).take(7).map(char::from).collect()
}

#[tauri::command(async)]
fn evolve_population(selection: [String; 2]) {
    let _ = WINDOW.get().unwrap().emit("evolving", "");

    let population_size = 8;

    let mut threads = vec![];

    let path_borrow = PATHS.lock().unwrap();
    let path = path_borrow.get("data").unwrap();

    fs::remove_dir_all(path).unwrap();
    fs::create_dir(path).unwrap();

    let clone = selection.clone();
    let selection = [string_to_static_str(clone[0].clone()), string_to_static_str(clone[1].clone())];

    let mut population = POPULATION.lock().unwrap();
    let mut old_population = vec![];
    for old_genotype in population.values() {
        old_population.push(Mutex::new(old_genotype.lock().unwrap().clone()));
    }
    let mut new_population = NEW_POPULATION.lock().unwrap();

    for i in 0..population_size {
        let index = if i % 2 == 0 { 0 } else { 1 };
        let genotype = population.get(selection.clone()[index]).unwrap();
        let mut genotype = genotype.lock().unwrap().clone();

        let partner = old_population.get(rand::thread_rng().gen_range(0..old_population.len())).unwrap().lock().unwrap().clone();
        println!("{}", genotype.size());
        genotype.crossover(partner);
        println!("{}", genotype.size());
        new_population.push(Mutex::new(genotype));
    }

    population.clear();
    
    // for (k, v) in &new_population {
    //     let v = v.lock().unwrap();
    //     population.insert(k.clone(), Mutex::new(v.clone()));
    //     drop(v);
    // }

    for i in 0..population_size {
        let handle = thread::spawn(move || {
            // let population = POPULATION.lock().unwrap();
            let mut best_of_population = BEST_OF_POPULATION.lock().unwrap();
            let mut new_population = NEW_POPULATION.lock().unwrap();
            let genotype = new_population.pop().unwrap();
            let genotype = genotype.lock().unwrap().clone();

            println!("{:p}", &genotype);

            let str = random_string();
            let mut new_genotype = genotype.clone();
            new_genotype.mutate(); 

            best_of_population.insert(str.clone(), Mutex::new(new_genotype.clone()));

            let out: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = interpret(new_genotype.clone().get_root());
            let _ = out.save(PATHS.lock().unwrap().get("data").unwrap().join(format!("{}.png", str.clone())));

            drop(new_population);
            drop(best_of_population);

            update_best_of_population_counter(population_size);
            println!("evolved population {}", i);
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

            // let out: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = IMAGE_TERMINAL_SET[4].clone();
            // let _ = out.save(PATHS.lock().unwrap().get("data").unwrap().join(format!("{}.png", "test")));

            generate_population();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_phenotypes, generate_population, evolve_population])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
