use std::fs;

use image::{RgbaImage, io::Reader};
use rand::{Rng, seq::IteratorRandom};

use crate::{sets::{FUNCTION, FUNCTION_SET, ETerminal, IMAGE_TERMINAL_SET, STAMP_IMAGE_TERMINAL_SET}, PATHS, tree::{Node, NodeType, NodeValue}};

pub fn random_function() -> FUNCTION {
    FUNCTION_SET[rand::thread_rng().gen_range(0..FUNCTION_SET.len())].clone()
}

pub fn _random_image() -> RgbaImage {
    IMAGE_TERMINAL_SET[rand::thread_rng().gen_range(0..IMAGE_TERMINAL_SET.len())].clone()
}

pub fn random_stamp() -> RgbaImage {
    STAMP_IMAGE_TERMINAL_SET[rand::thread_rng().gen_range(0..STAMP_IMAGE_TERMINAL_SET.len())].clone()
}

pub fn random_image() -> Node {
    Node {
        node_type: NodeType::Terminal,
        function: None,
        terminal: Some(ETerminal::Image),
        value: Some(NodeValue::from_image(_random_image())),
        args: vec![]
    }
}