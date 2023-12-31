use std::{vec, fs};

use image::{RgbaImage, io::Reader};
use lazy_static::lazy_static;

use crate::PATHS;

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
    Gradient,
    // Noise
    Noise,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ETerminal {
    Int32,
    Float32,
    Coordinate,
    Rgba8,
    Image,
    Stamp,
    // ResizeFilter,
    NoiseType,
}

// #[derive(Clone, Debug)]
// pub enum EImage {
//     Carnation,
//     Tape,
//     Statue,
//     Paper,
//     Empty,
// }

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
            arity: 4,
            args: vec![ETerminal::Image, ETerminal::Image, ETerminal::Coordinate, ETerminal::Float32],
        },
        FUNCTION {
            name: "Tile".to_string(),
            function: EFunction::Tile,
            arity: 2,
            // args: vec![ETerminal::Image, ETerminal::Float32, ETerminal::ResizeFilter],
            args: vec![ETerminal::Image, ETerminal::Float32],
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
        // Noise
        FUNCTION {
            name: "Noise".to_string(),
            function: EFunction::Noise,
            arity: 4,
            args: vec![ETerminal::Image, ETerminal::NoiseType, ETerminal::Float32, ETerminal::Int32],
        },
    ]);
}

// pub static RESIZE_FILTER_SET: [FilterType; 5] = [
//     FilterType::Nearest,
//     FilterType::CatmullRom,
//     FilterType::Gaussian,
//     FilterType::Lanczos3,
//     FilterType::Triangle,
// ];

//RgbaImage::new(1024, 1024)

fn clamp(x: i32) -> u8 {
    if x < 0 { return 0 as u8; }
    if x > 255 { return 255 as u8; }
    return x as u8;
}

lazy_static! {
    pub static ref IMAGE_TERMINAL_SET: [RgbaImage; 7] = [
        // X looping
        RgbaImage::from_fn(1024, 1024, |x, _y| {
            image::Rgba([x as u8, x as u8, x as u8, 255])
        }),
        // Y looping
        RgbaImage::from_fn(1024, 1024, |_x, y| {
            image::Rgba([y as u8, y as u8, y as u8, 255])
        }),
         // X normalized
         RgbaImage::from_fn(1024, 1024, |x, _y| {
            let xt = clamp(x as i32 - 512);
            image::Rgba([xt, xt, xt, 255])
        }),
         // Y normalized
         RgbaImage::from_fn(1024, 1024, |_x, y| {
            let yt = clamp(y as i32 - 512);
            image::Rgba([yt, yt, yt, 255])
        }),
        // abs X
        RgbaImage::from_fn(1024, 1024, |x, _y| {
            let xt = x as i32 - 512;
            let xt = clamp(xt.abs());
            image::Rgba([xt, xt, xt, 255])
        }),
        // abs Y
        RgbaImage::from_fn(1024, 1024, |_x, y| {
            let yt = y as i32 - 512;
            let yt = clamp(yt.abs());
            image::Rgba([yt, yt, yt, 255])
        }),
        // mod X abs Y
        RgbaImage::from_fn(1024, 1024, |x, y| {
            let yt = y as i32 - 512;
            let xt = x as i32 - 512;
            if yt != 0 {
                let a = clamp((xt % yt).abs());
                image::Rgba([a, a, a, 255])
            } else {
                image::Rgba([0, 0, 0, 255])
            }
        }),
    ];
}

lazy_static! {
    pub static ref STAMP_IMAGE_TERMINAL_SET: Vec<RgbaImage> = {
        let path = PATHS.lock().unwrap().get("assets").unwrap().clone();
        let files = fs::read_dir(path).unwrap();
        let mut images = vec![];
        for file in files {
            images.push(Reader::open(file.unwrap().path()).unwrap().decode().unwrap().to_rgba8() as RgbaImage);
        }
        images
    };
}