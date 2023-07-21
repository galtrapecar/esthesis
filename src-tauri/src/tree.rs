use std::{cell::{RefCell, RefMut}, rc::Rc, borrow::BorrowMut};

use image::{Rgba, imageops::FilterType, RgbaImage, Pixel};
use rand::Rng;

use crate::{sets::{FUNCTION, ETerminal, RESIZE_FILTER_SET, EFunction}, functions::*, random::{random_function, random_stamp, random_image}, mutations::image_to_function, refstack::Stack};

#[derive(Clone, Debug)]
pub struct Genotype {
    root: NodeRef
}

impl Genotype {
    pub fn new() -> Self {
        Genotype { root: grow(2, 6) }
    }

    fn count_nodes(tree: Node) -> usize {
        if tree.args.len() == 0 {
            return 1;
        }
        let mut count: usize = tree.args.len();
        for node in tree.args.iter() {
            if node.borrow().terminal.is_none() {
                count += Self::count_nodes(node.borrow().clone());
                continue;
            }
            if node.borrow().terminal.is_some() && node.borrow().terminal.clone().unwrap() == ETerminal::Image {
                count += Self::count_nodes(node.borrow().clone());
                continue;
            }
        }
        return count;
    }

    pub fn size(&self) -> usize {
        Self::count_nodes(self.root.borrow().clone())
    }

    pub fn get_root(self) -> Node {
        self.root.borrow().clone()
    }

    pub fn mutate(&mut self) {
        let size = self.size();
        let mut random: usize = rand::thread_rng().gen_range(0..size);
        let mut stack: Stack<NodeRef> = Stack::new(&self.root);

        let mut current_node_ref= None;
         
        while stack.element.is_some() {
            if random == 0 {
                break;
            }
            let current_node = Rc::clone(stack.pop().unwrap());
            for arg in current_node.borrow().args.iter() {
                current_node_ref = Some(Rc::clone(arg));
                if current_node.clone().borrow().terminal.is_none() || *current_node.clone().borrow().terminal.as_ref().unwrap() == ETerminal::Image {
                    stack.push(arg);
                    random -= 1;
                }
            }
        }
        image_to_function(current_node_ref.unwrap().borrow_mut());
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
    pub resize_filter: Option<FilterType>,
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
            resize_filter: None,
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
            resize_filter: None,
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
            resize_filter: None,
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
            resize_filter: None,
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
            resize_filter: None,
            noise: None,
        }
    }
    pub fn from_resize_filter(i: FilterType) -> Self {
        NodeValue { 
            int32: None, 
            float32: None, 
            coordinate: None, 
            rgba8: None, 
            image: None, 
            stamp: None,
            resize_filter: Some(i),
            noise: None,
        }
    }
    pub fn from_noise_type(i: NoiseType) -> Self {
        NodeValue { 
            int32: None, 
            float32: None, 
            coordinate: None, 
            rgba8: None, 
            image: None, 
            stamp: None,
            resize_filter: None,
            noise: Some(i),
        }
    }
    pub fn from_stamp(i: RgbaImage) -> Self {
        NodeValue { 
            int32: None, 
            float32: None, 
            coordinate: None, 
            rgba8: None, 
            image: None, 
            stamp: Some(i),
            resize_filter: None,
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
    pub args: Vec<NodeRef>,
}

pub type NodeRef = Rc<RefCell<Node>>;

pub fn grow(depth: u32, max_depth: u32) -> NodeRef {
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
                    root.args.append(&mut vec![Rc::new(RefCell::new(random_image()))]);
                }
            },
            ETerminal::Int32 => {
                root.args.append(&mut vec! [Rc::new(RefCell::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Int32),
                    value: Some(NodeValue::from_int32(rand::thread_rng().gen_range(1..255))),
                    args: vec![]
                }))]);
            },
            ETerminal::Float32 => {
                root.args.append(&mut vec! [Rc::new(RefCell::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::Float32),
                    value: Some(NodeValue::from_float32(rand::thread_rng().gen_range(1..100) as f32 / 100.0)),
                    args: vec![]
                }))]);
            },
            ETerminal::Coordinate => {
                root.args.append(&mut vec! [Rc::new(RefCell::new(Node {
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
                root.args.append(&mut vec! [Rc::new(RefCell::new(Node {
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
                root.args.append(&mut vec! [Rc::new(RefCell::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::ResizeFilter),
                    value: Some(NodeValue::from_stamp(random_stamp())),
                    args: vec![]
                }))]);
            },
            ETerminal::ResizeFilter => {
                root.args.append(&mut vec! [Rc::new(RefCell::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::ResizeFilter),
                    value: Some(NodeValue::from_resize_filter(RESIZE_FILTER_SET.clone()[rand::thread_rng().gen_range(0..RESIZE_FILTER_SET.len())])),
                    args: vec![]
                }))]);
            },
            ETerminal::NoiseType => {
                root.args.append(&mut vec! [Rc::new(RefCell::new(Node {
                    node_type: NodeType::Terminal,
                    function: None,
                    terminal: Some(ETerminal::NoiseType),
                    value: Some(NodeValue::from_noise_type([NoiseType::Gaussian, NoiseType::SaltPepper][rand::thread_rng().gen_range(0..2)])),
                    args: vec![]
                }))]);
            },
        }
    }
    Rc::new(RefCell::new(root))
}

pub fn interpret(node: Node) -> RgbaImage {
    let mut arguments: Vec<NodeValue> = vec![];
    for child in node.args {
        match child.clone().borrow().node_type {
            NodeType::Function => {
                arguments.append(&mut vec! [NodeValue {
                    image: Some(interpret(child.borrow().clone())),
                    int32: None,
                    float32: None,
                    coordinate: None,
                    rgba8: None,
                    stamp: None,
                    resize_filter: None,
                    noise: None,
                }]);
            },
            NodeType::Terminal => {
                arguments.append(&mut vec![child.as_ref().borrow().value.clone().unwrap()])
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
            },
            // Noise
            EFunction::Noise => {
                noise(arguments[0].clone().image.unwrap(), arguments[1].clone().noise.unwrap(), arguments[2].clone().float32.unwrap(), arguments[3].clone().int32.unwrap() as u32)
            }
        };
    image
}