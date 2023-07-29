use std::sync::Arc;

use image::RgbaImage;
use rand::Rng;

use crate::{sets::{FUNCTION, FUNCTION_SET, ETerminal, IMAGE_TERMINAL_SET, STAMP_IMAGE_TERMINAL_SET}, tree::{Node, NodeType, NodeValue, NodeRef, NodeRefs}};

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
        args: NodeRefs(vec![])
    }
}

pub fn random_node(mut random: usize, root: &NodeRef) -> Option<NodeRef> {
    let mut stack: Vec<NodeRef> = vec![];
    stack.push(Arc::clone(root));

    let mut current_node_ref= None;

    while stack.len() > 0 {
        if random == 0 {
            break;
        }
        let current_node = Arc::clone(&stack.pop().unwrap());
        let guard = current_node.lock().unwrap();
        for arg in guard.clone().args.0.iter() {
            current_node_ref = Some(Arc::clone(arg));
            if guard.clone().function.is_some() && random > 0 {
                stack.push(Arc::clone(arg));
                random -= 1;
            }
        }
    }

    return current_node_ref;
}