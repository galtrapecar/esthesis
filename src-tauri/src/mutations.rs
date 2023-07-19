// Img => fn

use image::{Rgba, Pixel};
use rand::{Rng, seq::SliceRandom};
use std::rc::Rc;

use crate::{tree::{Node, NodeType, NodeValue}, sets::{ETerminal, RESIZE_FILTER_SET, FUNCTION}, random::{random_function, random_stamp, random_image}, functions::NoiseType};

fn populate_args(mut function: FUNCTION, image: Node) -> Vec<Node> {
    let mut args: Vec<Node> = vec![];
    let mut image_token: u8 = 1;
    for arg in function.args {
        match arg {
            ETerminal::Image => {
                if image_token == 1 {
                    args.append(&mut vec![image.clone()]);
                    image_token -= 1;
                } else {
                    args.append(&mut vec![random_image()]);
                }
            },
            ETerminal::Int32 => {
                args.append(&mut vec![Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Int32),
                    value: Some(NodeValue::from_int32(rand::thread_rng().gen_range(1..255))),
                    args: vec![]
                }]);
            },
            ETerminal::Float32 => {
                args.append(&mut vec![Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Float32),
                    value: Some(NodeValue::from_float32(rand::thread_rng().gen_range(1..100) as f32 / 100.0)),
                    args: vec![]
                }]);
            },
            ETerminal::Coordinate => {
                args.append(&mut vec![Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Coordinate),
                    value: Some(NodeValue::from_coordinate([
                        rand::thread_rng().gen_range(0..980),
                        rand::thread_rng().gen_range(0..980)
                    ])),
                    args: vec![]
                }]);
            },
            ETerminal::Rgba8 => {
                args.append(&mut vec![Node {
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
                }]);
            },
            ETerminal::Stamp => {
                args.append(&mut vec![Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::ResizeFilter),
                    value: Some(NodeValue::from_stamp(random_stamp())),
                    args: vec![]
                }]);
            },
            ETerminal::ResizeFilter => {
                args.append(&mut vec![Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::ResizeFilter),
                    value: Some(NodeValue::from_resize_filter(RESIZE_FILTER_SET.clone()[rand::thread_rng().gen_range(0..RESIZE_FILTER_SET.len())])),
                    args: vec![]
                }]);
            },
            ETerminal::NoiseType => {
                args.append(&mut vec![Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::NoiseType),
                    value: Some(NodeValue::from_noise_type([NoiseType::Gaussian, NoiseType::SaltPepper][rand::thread_rng().gen_range(0..2)])),
                    args: vec![]
                }]);
            },
        }
    }
    args
}

pub fn select_random_image_terminal<'a>(genotype: Rc<Node>) -> Option<Rc<Node>> {
    let mut selection: Vec<Rc<Node>> = vec![];

    let mut stack: Vec<Rc<Node>> = vec![genotype];

    while stack.len() > 0 {
        let current = stack.pop();
        for child in current.unwrap().args {
            if child.node_type == NodeType::Terminal {
                if child.terminal.is_some() && child.clone().terminal.unwrap() == ETerminal::Image {
                    selection.push(child);
                }
                continue;
            }
            stack.push(child);
        }
    }
    let selected = selection.choose(&mut rand::thread_rng());
    println!("{:?}", *selected.unwrap());
    if selected.is_none() { return None }
    Some(*selected.unwrap())
}

pub fn image_to_function(node: &mut Node) {
    if node.node_type != NodeType::Terminal || (node.terminal.is_some() && node.clone().terminal.unwrap() != ETerminal::Image) {
        return;
    }

    let function = random_function();

    let new_node = Node {
        node_type: NodeType::Function,
        function: Some(function.clone()),
        terminal: None,
        value: None,
        args: populate_args(function.clone(), node.clone())
    };

    *node = new_node;
}

pub fn mutate(genotype: Node) {
    let mutation_point = select_random_image_terminal(genotype);
    if mutation_point.is_none() { return; }
    println!("{:?}", mutation_point.clone().unwrap().function);
    image_to_function(&mut mutation_point);
    println!("{:?}", mutation_point.clone().unwrap().function);
}