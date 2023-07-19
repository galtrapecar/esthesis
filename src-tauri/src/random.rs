use std::fs;

use image::{RgbaImage, io::Reader};
use rand::{Rng, seq::IteratorRandom};

use crate::{sets::{FUNCTION, FUNCTION_SET, ETerminal}, PATHS, tree::{Node, NodeType, NodeValue}};

pub fn random_function() -> FUNCTION {
    FUNCTION_SET[rand::thread_rng().gen_range(0..FUNCTION_SET.len())].clone()
}

pub fn random_stamp() -> RgbaImage {
    let path = PATHS.lock().unwrap().get("assets").unwrap().clone();
    let mut rng = rand::thread_rng();
    let files = fs::read_dir(path).unwrap();
    let file = files.choose(&mut rng).unwrap().unwrap();
    Reader::open(file.path()).unwrap().decode().unwrap().to_rgba8() as RgbaImage
}

pub fn random_image() -> Node {
    Node {
        node_type: NodeType::Terminal,
        function: None,
        terminal: Some(ETerminal::Image),
        value: Some(NodeValue::from_image(
            RgbaImage::from_fn(1024, 1024, |x, _y| {
                image::Rgba([x as u8, x as u8, x as u8, 255])
            })
        )),
        args: vec![]
    }
}