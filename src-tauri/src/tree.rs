use image::{Rgba, imageops::FilterType, RgbaImage, Pixel};
use rand::Rng;

use crate::{sets::{FUNCTION_SET, FUNCTION, ETerminal, RESIZE_FILTER_SET, EFunction}, functions::*};

#[derive(Clone, Debug)]
enum NodeType {
    Function,
    Terminal,
}

#[derive(Clone, Debug)]
struct NodeValue {
    int32: Option<i32>,
    float32: Option<f32>,
    coordinate: Option<[i64; 2]>,
    rgba8: Option<Rgba<u8>>,
    image: Option<RgbaImage>,
    resize_filter: Option<FilterType>,
}

#[derive(Clone, Debug)]
pub struct Node {
    node_type: NodeType,
    function: Option<FUNCTION>,
    terminal: Option<ETerminal>,
    value: Option<NodeValue>,
    args: Vec<Node>,
}

fn random_function() -> FUNCTION {
    FUNCTION_SET[rand::thread_rng().gen_range(0..FUNCTION_SET.len())].clone()
}

pub fn grow(depth: u32, max_depth: u32) -> Node {
    // initial node
    let mut root = Node {
        node_type: NodeType::Function,
        function: Some(random_function()),
        terminal: None,
        value: None,
        args: vec![]
    };

    for arg in root.clone().function.unwrap().args {
        match arg {
            ETerminal::Image => {
                // if d == D return random terminal
                let f_or_t = rand::thread_rng().gen_range(0..=10);
                // 100% probability of creating a branching function node
                if depth <= 2 {
                    root.args.append(&mut vec![grow(depth + 1, max_depth)]);
                    continue;
                }
                // 90% probability of creating a branching function node
                if f_or_t > 1 && depth < max_depth {
                    root.args.append(&mut vec![grow(depth + 1, max_depth)]);
                    continue;
                } else {
                // 10% probability of creating a final image terminal
                    root.args.append(&mut vec![Node {
                        node_type: NodeType::Terminal,
                        function: None,
                        terminal: Some(ETerminal::Image),
                        value: Some(NodeValue {
                            int32: None,
                            float32: None,
                            coordinate: None,
                            rgba8: None,
                            image: Some(RgbaImage::new(500, 500)),
                            resize_filter: None,
                        }),
                        args: vec![]
                    }]);
                }
            },
            ETerminal::Int32 => {
                root.args.append(&mut vec![Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Int32),
                    value: Some(NodeValue {
                        int32: Some(rand::thread_rng().gen_range(1..255)),
                        float32: None,
                        coordinate: None,
                        rgba8: None,
                        image: None,
                        resize_filter: None,
                    }),
                    args: vec![]
                }]);
            },
            ETerminal::Float32 => {
                root.args.append(&mut vec![Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Float32),
                    value: Some(NodeValue {
                        int32: None,
                        float32: Some(rand::thread_rng().gen_range(1..100) as f32 / 100.0),
                        coordinate: None,
                        rgba8: None,
                        image: None,
                        resize_filter: None,
                    }),
                    args: vec![]
                }]);
            },
            ETerminal::Coordinate => {
                root.args.append(&mut vec![Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Coordinate),
                    value: Some(NodeValue {
                        int32: None,
                        float32: None,
                        coordinate: Some([
                            rand::thread_rng().gen_range(0..980),
                            rand::thread_rng().gen_range(0..980)
                        ]),
                        rgba8: None,
                        image: None,
                        resize_filter: None,
                    }),
                    args: vec![]
                }]);
            },
            ETerminal::Rgba8 => {
                root.args.append(&mut vec![Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Rgba8),
                    value: Some(NodeValue {
                        int32: None,
                        float32: None,
                        coordinate: None,
                        rgba8: Some(*Rgba::from_slice(&[
                            rand::thread_rng().gen_range(0..255) as u8,
                            rand::thread_rng().gen_range(0..255) as u8,
                            rand::thread_rng().gen_range(0..255) as u8,
                            rand::thread_rng().gen_range(1..255) as u8
                        ])),
                        image: None,
                        resize_filter: None,
                    }),
                    args: vec![]
                }]);
            },
            ETerminal::ResizeFilter => {
                root.args.append(&mut vec![Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::ResizeFilter),
                    value: Some(NodeValue {
                        int32: None,
                        float32: None,
                        coordinate: None,
                        rgba8: None,
                        image: None,
                        resize_filter: Some(RESIZE_FILTER_SET.clone()[rand::thread_rng().gen_range(0..RESIZE_FILTER_SET.len())]),
                    }),
                    args: vec![]
                }]);
            },
        }
    }
    root
}

pub fn interpret(node: Node) -> RgbaImage {
    let mut arguments: Vec<NodeValue> = vec![];
    for child in node.args {
        match child.node_type {
            NodeType::Function => {
                arguments.append(&mut vec![NodeValue {
                    image: Some(interpret(child)),
                    int32: None,
                    float32: None,
                    coordinate: None,
                    rgba8: None,
                    resize_filter: None,
                }])
            },
            NodeType::Terminal => {
                arguments.append(&mut vec![child.value.unwrap()])
            }
        }
    }
    let image: RgbaImage = 
        match node.function.unwrap().function {
            // Overlay
            EFunction::Add => {
                add(arguments[0].clone().image.unwrap(), arguments[1].clone().image.unwrap())
            },
            EFunction::Stamp => {
                stamp(arguments[0].clone().image.unwrap(), arguments[1].clone().image.unwrap(), Some(arguments[2].coordinate.unwrap()))
            },
            EFunction::Tile => {
                tile(arguments[0].clone().image.unwrap(), arguments[1].clone().float32.unwrap(), arguments[2].resize_filter.unwrap())
            },
            // Color
            EFunction::Brighten => {
                brighten(arguments[0].clone().image.unwrap(), arguments[1].clone().int32.unwrap())
            },
            EFunction::Contrast => {
                contrast(arguments[0].clone().image.unwrap(), arguments[1].clone().float32.unwrap())
            },
            EFunction::Hue => {
                hue(arguments[0].clone().image.unwrap(), arguments[1].clone().int32.unwrap())
            },
            EFunction::Invert => {
                invert(arguments[0].clone().image.unwrap())
            },
            // Transforms
            EFunction::FlipHorizontal => {
                flip_horizontal(arguments[0].clone().image.unwrap())
            },
            EFunction::FlipVertical => {
                flip_vertical(arguments[0].clone().image.unwrap())
            },
            // Draw
            EFunction::Gradient => {
                gradient(arguments[0].clone().image.unwrap(), arguments[1].clone().rgba8.unwrap(), arguments[2].rgba8.unwrap())
            }
        };
    image
    // args = []
    // for arg in node.args:
    //     if arg.function:
    //         args.append(interpret(arg))
    //     else:
    //         args.append(arg.terminal)
    // image = node.function(*args)
    // return image
}

// match arg.function.unwrap().function {
//     // Overlay
//     EFunction::Add => {
//         add(interpret(node.args[0]), )
//     },
//     EFunction::Stamp => {

//     },
//     EFunction::Tile => {

//     },
//     // Color
//     EFunction::Brighten => {

//     },
//     EFunction::Contrast => {

//     },
//     EFunction::Hue => {

//     },
//     EFunction::Invert => {

//     },
//     // Transforms
//     EFunction::FlipHorizontal => {

//     },
//     EFunction::FlipVertical => {

//     },
//     // Draw
//     EFunction::Gradient => {

//     }
// }