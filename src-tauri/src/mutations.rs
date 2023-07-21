// Img => fn

use std::{cell::RefCell, borrow::Borrow, rc::Rc};

use image::{Rgba, Pixel};
use rand::Rng;

use crate::{tree::{Node, NodeType, NodeValue, NodeRef}, sets::{ETerminal, RESIZE_FILTER_SET, FUNCTION}, random::{random_function, random_stamp, random_image}, functions::NoiseType};

fn populate_args(function: FUNCTION, image: NodeRef) -> Vec<NodeRef> {
    let mut args: Vec<NodeRef> = vec![];
    let mut image_token: u8 = 1;
    for arg in function.args {
        match arg {
            ETerminal::Image => {
                if image_token == 1 {
                    args.append(&mut vec![image.clone()]);
                    image_token -= 1;
                } else {
                    args.append(&mut vec![Rc::new(RefCell::new(random_image()))]);
                }
            },
            ETerminal::Int32 => {
                args.append(&mut vec![Rc::new(RefCell::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Int32),
                    value: Some(NodeValue::from_int32(rand::thread_rng().gen_range(1..255))),
                    args: vec![]
                }))]);
            },
            ETerminal::Float32 => {
                args.append(&mut vec![Rc::new(RefCell::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Float32),
                    value: Some(NodeValue::from_float32(rand::thread_rng().gen_range(1..100) as f32 / 100.0)),
                    args: vec![]
                }))]);
            },
            ETerminal::Coordinate => {
                args.append(&mut vec![Rc::new(RefCell::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Coordinate),
                    value: Some(NodeValue::from_coordinate([
                        rand::thread_rng().gen_range(0..980),
                        rand::thread_rng().gen_range(0..980)
                    ])),
                    args: vec![]
                }))]);
            },
            ETerminal::Rgba8 => {
                args.append(&mut vec![Rc::new(RefCell::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Rgba8),
                    value: Some(NodeValue::from_rgba8(*Rgba::from_slice(&[
                        rand::thread_rng().gen_range(0..255) as u8,
                        rand::thread_rng().gen_range(0..255) as u8,
                        rand::thread_rng().gen_range(0..255) as u8,
                        rand::thread_rng().gen_range(1..255) as u8
                    ]))),
                    args: vec![]
                }))]);
            },
            ETerminal::Stamp => {
                args.append(&mut vec![Rc::new(RefCell::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::ResizeFilter),
                    value: Some(NodeValue::from_stamp(random_stamp())),
                    args: vec![]
                }))]);
            },
            ETerminal::ResizeFilter => {
                args.append(&mut vec![Rc::new(RefCell::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::ResizeFilter),
                    value: Some(NodeValue::from_resize_filter(RESIZE_FILTER_SET.clone()[rand::thread_rng().gen_range(0..RESIZE_FILTER_SET.len())])),
                    args: vec![]
                }))]);
            },
            ETerminal::NoiseType => {
                args.append(&mut vec![Rc::new(RefCell::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::NoiseType),
                    value: Some(NodeValue::from_noise_type([NoiseType::Gaussian, NoiseType::SaltPepper][rand::thread_rng().gen_range(0..2)])),
                    args: vec![]
                }))]);
            },
        }
    }
    args
}

pub fn image_to_function(node: &mut NodeRef) {
    let clone = Rc::clone(&node);
    if clone.clone().as_ref().borrow().node_type != NodeType::Terminal || (clone.clone().as_ref().borrow().terminal.is_some() && clone.clone().as_ref().borrow().terminal.clone().unwrap() != ETerminal::Image) {
        return;
    }

    println!("image");

    let copy = node.clone();

    let function = random_function();

    node.borrow_mut().terminal = None;
    node.borrow_mut().function = Some(function.clone());
    node.borrow_mut().node_type = NodeType::Function;
    node.borrow_mut().args = populate_args(function.clone(), copy);
}