use std::fmt;

use crate::{tree::{Node, NodeType, NodeRefs}, sets::ETerminal};

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self.node_type {
            NodeType::Function => {
                write!(f, "{:?} ( {})", self.function.clone().unwrap().name, self.args)
            },
            NodeType::Terminal => {
                match self.terminal.clone().unwrap() {
                    ETerminal::Image => write!(f, "{:?} ( {:?} )", self.node_type, self.terminal.clone().unwrap()),
                    ETerminal::Coordinate => write!(f, "{:?} ( {:?} )", self.node_type, self.value.clone().unwrap().coordinate.unwrap()),
                    ETerminal::Int32 => write!(f, "{:?} ( {} )", self.node_type, self.value.clone().unwrap().int32.unwrap()),
                    ETerminal::Float32 => write!(f, "{:?} ( {} )", self.node_type, self.value.clone().unwrap().float32.unwrap()),
                    ETerminal::Rgba8 => write!(f, "{:?} ( {:?} )", self.node_type, self.value.clone().unwrap().rgba8.unwrap()),
                    ETerminal::Stamp => write!(f, "{:?} ( {:?} )", self.node_type, self.terminal.clone().unwrap()),
                    ETerminal::NoiseType => write!(f, "{:?} ( {:?} )", self.node_type, self.value.clone().unwrap().noise.unwrap()),
                }
            }
        }
    }
}

impl fmt::Display for NodeRefs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = "".to_string();
        for node in self.0.iter() {
            string += &format!("{} ", node.lock().unwrap().clone());
        }
        write!(f, "{}", string)
    }
}