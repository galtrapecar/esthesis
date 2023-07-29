use std::{borrow::BorrowMut, sync::{Arc, Mutex}, ops::DerefMut};

use image::{Rgba, RgbaImage, Pixel};
use rand::Rng;

use crate::{sets::{FUNCTION, ETerminal, EFunction}, functions::*, random::{random_function, random_stamp, random_image, random_node}, mutations::{image_to_function, swap_terminal, swap_image, swap_function, grow_branch}};

#[derive(Clone, Debug)]
pub struct Genotype {
    root: NodeRef
}

impl Genotype {
    pub fn new() -> Self {
        Genotype { root: grow(2, 6) }
    }

    fn count_nodes(tree: Node) -> usize {
        let mut count: usize = 1;
        let mut stack = vec![tree];

        while stack.len() > 0 {
            let current = stack.pop().unwrap();

            for arg in current.args.0 {
                stack.push(arg.lock().unwrap().clone());
                count += 1;
            }
        }
        count
    }

    pub fn size(&self) -> usize {
        Self::count_nodes(self.root.lock().unwrap().clone())
    }

    pub fn get_root(self) -> Node {
        self.root.lock().unwrap().clone()
    }

    pub fn mutate(&mut self) {
        // println!("mutate call");
        let size = self.size();
        let mut random: usize = rand::thread_rng().gen_range(0..size);
        let mut stack: Vec<Arc<Mutex<Node>>> = vec![];
        stack.push(Arc::clone(&self.root));

        let mut current_node_ref= None;
        
        while stack.len() > 0 {
            if random == 0 {
                break;
            }
            let current_node = Arc::clone(&stack.pop().unwrap());
            let guard = current_node.lock().unwrap();
            for arg in guard.clone().args.0.iter() {
                current_node_ref = Some(Arc::clone(arg));
                if random == 0 {
                    break;
                }
                if guard.clone().terminal.is_some() && guard.clone().terminal.unwrap() != ETerminal::Image {
                    if rand::thread_rng().gen_range(0..=100) > 20 { return; }
                    stack.push(Arc::clone(arg));
                    random -= 1;
                } else {
                    stack.push(Arc::clone(arg));
                    random -= 1;
                }
            }
        }

        if current_node_ref.is_none() { return; }
        let lock = current_node_ref.clone().unwrap().lock().unwrap().clone();
        match lock.clone().node_type {
            NodeType::Function => {
                println!("swapping function");
                swap_function(current_node_ref.unwrap().borrow_mut());
            },
            NodeType::Terminal => {
                match lock.clone().terminal.unwrap() {
                    ETerminal::Image => {
                        let coinflip = rand::thread_rng().gen_range(0..=100);
                        if coinflip > 66 {
                            println!("image to function");
                            image_to_function(current_node_ref.unwrap().borrow_mut());
                        } else if coinflip > 33 {
                            println!("swapping image");
                            swap_image(current_node_ref.unwrap().borrow_mut());
                        } else {
                            println!("growing branch");
                            grow_branch(current_node_ref.unwrap().borrow_mut());
                        }
                    },
                    _ => {
                        println!("swapping terminal");
                        swap_terminal(current_node_ref.unwrap().borrow_mut());
                    }
                }
            },
        }
    }

    pub fn crossover(&mut self, partner: Genotype) {
        let my_size = self.size();
        let partner_size = partner.size();
        let my_node = random_node(rand::thread_rng().gen_range(1..my_size), &self.root).unwrap();
        let mut my_node = my_node.lock().unwrap();
        let mut _borrow = my_node.deref_mut();

        let partner_node = random_node(rand::thread_rng().gen_range(1..partner_size), &partner.root).unwrap();
        let mut partner_node = partner_node.lock().unwrap().clone();

        _borrow = &mut partner_node;

        drop(partner_node);
        drop(my_node);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NodeType {
    Function,
    Terminal,
}

#[derive(Clone, Debug)]
pub struct NodeValue {
    pub int32: Option<i32>,
    pub float32: Option<f32>,
    pub coordinate: Option<[i64; 2]>,
    pub rgba8: Option<Rgba<u8>>,
    pub image: Option<RgbaImage>,
    pub stamp: Option<RgbaImage>,
    // pub resize_filter: Option<FilterType>,
    pub noise: Option<NoiseType>
}

impl NodeValue {
    pub fn from_image(i: RgbaImage) -> Self {
        NodeValue { 
            int32: None, 
            float32: None, 
            coordinate: None, 
            rgba8: None, 
            image: Some(i),
            stamp: None,
            // // resize_filter: None,
            noise: None,
        }
    }
    pub fn from_int32(i: i32) -> Self {
        NodeValue { 
            int32: Some(i), 
            float32: None, 
            coordinate: None, 
            rgba8: None, 
            image: None,
            stamp: None,
            // resize_filter: None,
            noise: None,
        }
    }
    pub fn from_float32(i: f32) -> Self {
        NodeValue { 
            int32: None, 
            float32: Some(i), 
            coordinate: None, 
            rgba8: None, 
            image: None, 
            stamp: None,
            // resize_filter: None,
            noise: None,
        }
    }
    pub fn from_coordinate(i: [i64; 2]) -> Self {
        NodeValue { 
            int32: None, 
            float32: None, 
            coordinate: Some(i), 
            rgba8: None, 
            image: None, 
            stamp: None,
            // resize_filter: None,
            noise: None,
        }
    }
    pub fn from_rgba8(i: Rgba<u8>) -> Self {
        NodeValue { 
            int32: None, 
            float32: None, 
            coordinate: None, 
            rgba8: Some(i), 
            image: None, 
            stamp: None,
            // resize_filter: None,
            noise: None,
        }
    }
    // pub fn from_resize_filter(i: FilterType) -> Self {
    //     NodeValue { 
    //         int32: None, 
    //         float32: None, 
    //         coordinate: None, 
    //         rgba8: None, 
    //         image: None, 
    //         stamp: None,
    //         resize_filter: Some(i),
    //         noise: None,
    //     }
    // }
    pub fn from_noise_type(i: NoiseType) -> Self {
        NodeValue { 
            int32: None, 
            float32: None, 
            coordinate: None, 
            rgba8: None, 
            image: None, 
            stamp: None,
            // resize_filter: None,
            noise: Some(i),
        }
    }
    pub fn from_stamp(i: RgbaImage) -> Self {
        NodeValue { 
            int32: None, 
            float32: None, 
            coordinate: None, 
            rgba8: None, 
            image: Some(i), 
            stamp: None,
            // resize_filter: None,
            noise: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub function: Option<FUNCTION>,
    pub terminal: Option<ETerminal>,
    pub value: Option<NodeValue>,
    pub args: NodeRefs,
}

pub type NodeRef = Arc<Mutex<Node>>;

#[derive(Clone, Debug)]
pub struct NodeRefs(pub Vec<NodeRef>);

pub fn grow(depth: u32, max_depth: u32) -> NodeRef {
    // initial node
    let mut root = Node {
        node_type: NodeType::Function,
        function: Some(random_function()),
        terminal: None,
        value: None,
        args: NodeRefs(vec![])
    };

    for arg in root.clone().function.unwrap().args {
        match arg {
            ETerminal::Image => {
                // if d == D return random terminal
                let f_or_t = rand::thread_rng().gen_range(0..=10);
                // 100% probability of creating a branching function node
                if depth <= 2 {
                    root.args.0.append(&mut vec![grow(depth + 1, max_depth)]);
                    continue;
                }
                // 90% probability of creating a branching function node
                if f_or_t > 1 && depth < max_depth {
                    root.args.0.append(&mut vec![grow(depth + 1, max_depth)]);
                    continue;
                } else {
                // 10% probability of creating a final image terminal
                    root.args.0.append(&mut vec![Arc::new(Mutex::new(random_image()))]);
                }
            },
            ETerminal::Int32 => {
                root.args.0.append(&mut vec! [Arc::new(Mutex::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Int32),
                    value: Some(NodeValue::from_int32(rand::thread_rng().gen_range(1..255))),
                    args: NodeRefs(vec![])
                }))]);
            },
            ETerminal::Float32 => {
                root.args.0.append(&mut vec! [Arc::new(Mutex::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Float32),
                    value: Some(NodeValue::from_float32(rand::thread_rng().gen_range(1..100) as f32 / 100.0)),
                    args: NodeRefs(vec![])
                }))]);
            },
            ETerminal::Coordinate => {
                root.args.0.append(&mut vec! [Arc::new(Mutex::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Coordinate),
                    value: Some(NodeValue::from_coordinate([
                        rand::thread_rng().gen_range(0..980),
                        rand::thread_rng().gen_range(0..980)
                    ])),
                    args: NodeRefs(vec![])
                }))]);
            },
            ETerminal::Rgba8 => {
                root.args.0.append(&mut vec! [Arc::new(Mutex::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Rgba8),
                    value: Some(NodeValue::from_rgba8(*Rgba::from_slice(&[
                        rand::thread_rng().gen_range(0..255) as u8,
                        rand::thread_rng().gen_range(0..255) as u8,
                        rand::thread_rng().gen_range(0..255) as u8,
                        rand::thread_rng().gen_range(1..255) as u8
                    ]))),
                    args: NodeRefs(vec![])
                }))]);
            },
            ETerminal::Stamp => {
                // println!("stamp");
                root.args.0.append(&mut vec! [Arc::new(Mutex::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Image),
                    value: Some(NodeValue::from_stamp(random_stamp())),
                    args: NodeRefs(vec![])
                }))]);
            },
            // ETerminal::ResizeFilter => {
            //     root.args.0.append(&mut vec! [Arc::new(Mutex::new(Node {
            //         node_type: NodeType::Terminal,
            //         function: None,
            //         terminal: Some(ETerminal::ResizeFilter),
            //         value: Some(NodeValue::from_resize_filter(RESIZE_FILTER_SET.clone()[rand::thread_rng().gen_range(0..RESIZE_FILTER_SET.len())])),
            //         args: NodeRefs(vec![])
            //     }))]);
            // },
            ETerminal::NoiseType => {
                root.args.0.append(&mut vec! [Arc::new(Mutex::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::NoiseType),
                    value: Some(NodeValue::from_noise_type([NoiseType::Gaussian, NoiseType::SaltPepper][rand::thread_rng().gen_range(0..2)])),
                    args: NodeRefs(vec![])
                }))]);
            },
        }
    }
    Arc::new(Mutex::new(root))
}

pub fn interpret(node: Node) -> RgbaImage {
    let mut arguments: Vec<NodeValue> = vec![];
    for child in node.args.0 {
        let guard = child.lock().unwrap();
        let child = guard.clone();
        match child.node_type {
            NodeType::Function => {
                arguments.append(&mut vec! [NodeValue {
                    image: Some(interpret(child)),
                    int32: None,
                    float32: None,
                    coordinate: None,
                    rgba8: None,
                    stamp: None,
                    // resize_filter: None,
                    noise: None,
                }]);
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
                // println!("stamp");
                stamp(arguments[0].clone().image.unwrap(), arguments[1].clone().image.unwrap(), Some(arguments[2].coordinate.unwrap()), arguments[3].clone().float32.unwrap())
            },
            EFunction::Tile => {
                tile(arguments[0].clone().image.unwrap(), arguments[1].clone().float32.unwrap())
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
            },
            // Noise
            EFunction::Noise => {
                noise(arguments[0].clone().image.unwrap(), arguments[1].clone().noise.unwrap(), arguments[2].clone().float32.unwrap(), arguments[3].clone().int32.unwrap() as u32)
            }
        };
    image
}