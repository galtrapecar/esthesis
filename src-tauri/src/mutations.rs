// Img => fn


use std::{sync::{Arc, Mutex}, ops::DerefMut};

use image::{Rgba, Pixel};
use rand::Rng;

use crate::{tree::{Node, NodeType, NodeValue, NodeRef, grow}, sets::{ETerminal, FUNCTION}, random::{random_function, random_stamp, random_image}, functions::NoiseType};

fn populate_args(function: FUNCTION, mut images: Vec<Node>) -> Vec<NodeRef> {
    let mut args: Vec<NodeRef> = vec![];
    for arg in function.args {
        match arg {
            ETerminal::Image => {
                if images.len() != 0 {
                    let image = images.remove(rand::thread_rng().gen_range(0..images.len()));
                    args.append(&mut vec![Arc::new(Mutex::new(image.clone()))]);
                } else {
                    args.append(&mut vec![Arc::new(Mutex::new(random_image()))]);
                }
            },
            ETerminal::Int32 => {
                args.append(&mut vec![Arc::new(Mutex::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Int32),
                    value: Some(NodeValue::from_int32(rand::thread_rng().gen_range(1..255))),
                    args: vec![]
                }))]);
            },
            ETerminal::Float32 => {
                args.append(&mut vec![Arc::new(Mutex::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Float32),
                    value: Some(NodeValue::from_float32(rand::thread_rng().gen_range(1..100) as f32 / 100.0)),
                    args: vec![]
                }))]);
            },
            ETerminal::Coordinate => {
                args.append(&mut vec![Arc::new(Mutex::new(Node {
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
                args.append(&mut vec![Arc::new(Mutex::new(Node {
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
                args.append(&mut vec![Arc::new(Mutex::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Stamp),
                    value: Some(NodeValue::from_stamp(random_stamp())),
                    args: vec![]
                }))]);
            },
            // ETerminal::ResizeFilter => {
            //     args.append(&mut vec![Arc::new(Mutex::new(Node {
            //         node_type: NodeType::Terminal,
            //         function: None,
            //         terminal: Some(ETerminal::ResizeFilter),
            //         value: Some(NodeValue::from_resize_filter(RESIZE_FILTER_SET.clone()[rand::thread_rng().gen_range(0..RESIZE_FILTER_SET.len())])),
            //         args: vec![]
            //     }))]);
            // },
            ETerminal::NoiseType => {
                args.append(&mut vec![Arc::new(Mutex::new(Node {
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
    let mut guard = node.lock().unwrap();
    let mut_node = guard.deref_mut();
    
    if mut_node.clone().node_type != NodeType::Terminal || (mut_node.clone().terminal.is_some() && mut_node.clone().terminal.clone().unwrap() != ETerminal::Image) {
        return;
    }

    let function = random_function();

    let args = populate_args(function.clone(), vec![mut_node.clone()]);

    mut_node.terminal = None;
    mut_node.function = Some(function.clone());
    mut_node.node_type = NodeType::Function;
    mut_node.args = args.clone();

    drop(guard);
}

pub fn swap_image(node: &mut NodeRef) {
    let mut guard = node.lock().unwrap();
    let mut mut_node = guard.deref_mut();

    if mut_node.clone().node_type != NodeType::Terminal || (mut_node.clone().terminal.is_some() && mut_node.clone().terminal.clone().unwrap() != ETerminal::Image) {
        return;
    }

    mut_node = &mut random_image();
}

pub fn swap_terminal(node: &mut NodeRef) {
    let mut guard = node.lock().unwrap();
    let mut_node = guard.deref_mut();

    if mut_node.clone().node_type != NodeType::Terminal {
        return;
    }

    match mut_node.clone().terminal.unwrap() {
        ETerminal::Int32 => mut_node.value = Some(NodeValue::from_int32(rand::thread_rng().gen_range(1..255))),
        ETerminal::Float32 => mut_node.value = Some(NodeValue::from_float32(rand::thread_rng().gen_range(1..100) as f32 / 100.0)),
        ETerminal::Coordinate => mut_node.value = Some(NodeValue::from_coordinate([rand::thread_rng().gen_range(0..980),rand::thread_rng().gen_range(0..980)])),
        ETerminal::Rgba8 => mut_node.value = Some(NodeValue::from_rgba8(*Rgba::from_slice(&[
            rand::thread_rng().gen_range(0..255) as u8,
            rand::thread_rng().gen_range(0..255) as u8,
            rand::thread_rng().gen_range(0..255) as u8,
            rand::thread_rng().gen_range(1..255) as u8
        ]))),
        ETerminal::Image => {},
        ETerminal::Stamp => mut_node.value = Some(NodeValue::from_stamp(random_stamp())),
        // ETerminal::ResizeFilter => mut_node.value = Some(NodeValue::from_resize_filter(RESIZE_FILTER_SET.clone()[rand::thread_rng().gen_range(0..RESIZE_FILTER_SET.len())])),
        ETerminal::NoiseType => mut_node.value = Some(NodeValue::from_noise_type([NoiseType::Gaussian, NoiseType::SaltPepper][rand::thread_rng().gen_range(0..2)])),
    }

    drop(guard);
}

pub fn swap_function(node: &mut NodeRef) {
    let mut guard = node.lock().unwrap();
    let mut_node = guard.deref_mut();

    if mut_node.clone().node_type != NodeType::Function {
        return;
    }

    let function = random_function();

    let mut images: Vec<Node> = vec![];
    
    for arg in mut_node.clone().args {
        let node = arg.lock().unwrap();
        if node.clone().node_type == NodeType::Function {
            images.push(node.clone());
            return;
        }
    };

    mut_node.terminal = None;
    mut_node.function = Some(function.clone());
    mut_node.node_type = NodeType::Function;
    mut_node.args = populate_args(function, images);

    drop(guard);
}

pub fn grow_branch(node: &mut NodeRef) {
    let mut guard = node.lock().unwrap();
    let mut_node = guard.deref_mut();

    // if mut_node.clone().node_type != NodeType::Function || mut_node.clone().terminal.unwrap() != ETerminal::Image {
    //     return;
    // }

    if mut_node.clone().terminal.unwrap() != ETerminal::Image {
        return;
    }

    let node = grow(2, 3);

    *mut_node = node.lock().unwrap().clone();
}