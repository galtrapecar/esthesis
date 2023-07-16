use std::vec;

use image::{imageops::FilterType, RgbaImage};
use lazy_static::lazy_static;

#[derive(Clone, Debug)]
pub enum EFunction {
    // Overlay
    Add,
    Stamp,
    Tile,
    // Color
    Brighten,
    Contrast,
    Hue,
    Invert,
    // Transforms
    FlipHorizontal,
    FlipVertical,
    // Draw
    Gradient
}

#[derive(Clone, Debug)]
pub enum ETerminal {
    Int32,
    Float32,
    Coordinate,
    Rgba8,
    Image,
    ResizeFilter,
}

#[derive(Clone, Debug)]
pub struct FUNCTION {
    pub name: String,
    pub function: EFunction,
    pub arity: usize,
    pub args: Vec<ETerminal>,
}

lazy_static! {
    pub static ref FUNCTION_SET: Vec<FUNCTION> = Vec::from([
        // Overlay
        FUNCTION {
            name: "Add".to_string(),
            function: EFunction::Add,
            arity: 2,
            args: vec![ETerminal::Image, ETerminal::Image],
        },
        FUNCTION {
            name: "Stamp".to_string(),
            function: EFunction::Stamp,
            arity: 3,
            args: vec![ETerminal::Image, ETerminal::Image, ETerminal::Coordinate],
        },
        FUNCTION {
            name: "Tile".to_string(),
            function: EFunction::Tile,
            arity: 3,
            args: vec![ETerminal::Image, ETerminal::Float32, ETerminal::ResizeFilter],
        },
        // Color
        FUNCTION {
            name: "Brighten".to_string(),
            function: EFunction::Brighten,
            arity: 2,
            args: vec![ETerminal::Image, ETerminal::Int32],
        },
        FUNCTION {
            name: "Contrast".to_string(),
            function: EFunction::Contrast,
            arity: 2,
            args: vec![ETerminal::Image, ETerminal::Float32],
        },
        FUNCTION {
            name: "Hue".to_string(),
            function: EFunction::Hue,
            arity: 2,
            args: vec![ETerminal::Image, ETerminal::Int32],
        },
        FUNCTION {
            name: "Invert".to_string(),
            function: EFunction::Invert,
            arity: 1,
            args: vec![ETerminal::Image],
        },
        // Transforms
        FUNCTION {
            name: "Flip Horizontal".to_string(),
            function: EFunction::FlipHorizontal,
            arity: 1,
            args: vec![ETerminal::Image],
        },
        FUNCTION {
            name: "Flip Vertical".to_string(),
            function: EFunction::FlipVertical,
            arity: 1,
            args: vec![ETerminal::Image],
        },
        // Draw
        FUNCTION {
            name: "Gradient".to_string(),
            function: EFunction::Gradient,
            arity: 3,
            args: vec![ETerminal::Image, ETerminal::Rgba8, ETerminal::Rgba8],
        },
    ]);
}

pub static RESIZE_FILTER_SET: [FilterType; 5] = [
    FilterType::Nearest,
    FilterType::CatmullRom,
    FilterType::Gaussian,
    FilterType::Lanczos3,
    FilterType::Triangle,
];

lazy_static! {
    pub static ref IMAGE_TERMINAL_SET: Vec<RgbaImage> = Vec::from([RgbaImage::new(1024, 1024)]);
}