use image::*;
use image::imageops::*;
use image::imageops::colorops::*;
use imageproc::map::map_pixels;
use imageproc::noise::{gaussian_noise_mut, salt_and_pepper_noise_mut};

// Overlay

pub fn add(mut i: RgbaImage, mut j: RgbaImage) -> RgbaImage {
    j = map_pixels(&j, |_x, _y, p| {
        Rgba([p[0], p[1], p[2], 128])
    });
    overlay(&mut i, &j, 0, 0);
    i
}

pub fn stamp(mut i: RgbaImage, j: RgbaImage, xy: Option<[i64; 2]>, scale: f32) -> RgbaImage {
    let j = resize(j, scale, FilterType::Nearest);

    if xy.is_some() {
        overlay(&mut i, &j, xy.unwrap()[0], xy.unwrap()[1]);
    } else {
        overlay(&mut i, &j, 0, 0);
    }
    i
}

pub fn tile(mut i: RgbaImage, scale: f32) -> RgbaImage {
    let j = resize(i.clone(), scale, FilterType::Nearest);
    imageops::tile(&mut i, &j);
    i
}

// pub fn haar(mut i: RgbaImage) -> RgbaImage {
//     let haar_features = enumerate_haar_features(20, 20);
//     println!("{:?}", haar_features);
//     for haar_feature in haar_features {
//         draw_haar_feature_mut(&mut i, haar_feature);
//     }
//     i
// }

// Color

pub fn brighten(mut i: RgbaImage, value: i32) -> RgbaImage {
    brighten_in_place(&mut i, value);
    i
}

pub fn contrast(mut i: RgbaImage, value: f32) -> RgbaImage {
    contrast_in_place(&mut i, value);
    i
}

pub fn hue(mut i: RgbaImage, value: i32) -> RgbaImage {
    huerotate_in_place(&mut i, value);
    i
}

pub fn invert(mut i: RgbaImage) -> RgbaImage {
    colorops::invert(&mut i);
    i
}

// pub fn dither(mut i: RgbaImage, color_map: &Map) -> RgbaImage {
//     image::imageops::dither(&mut i, color_map);
//     i
// }

// Transforms

pub fn flip_horizontal(mut i: RgbaImage) -> RgbaImage {
    flip_horizontal_in_place( &mut i);
    i
}

pub fn flip_vertical(mut i: RgbaImage) -> RgbaImage {
    flip_vertical_in_place( &mut i);
    i
}

pub fn resize(i: RgbaImage, scale: f32, filter: FilterType) -> RgbaImage {
    imageops::resize(&i, (i.width() as f32 * scale) as u32, (i.height() as f32 * scale) as u32, filter)
}

// Draw

pub fn gradient(mut i: RgbaImage, start: Rgba<u8>, stop: Rgba<u8>) -> RgbaImage {
    let j = i.clone();
    horizontal_gradient(&mut i, &start, &stop);
    i = add(j, i);
    i
}

// Noise

#[derive(Clone, Copy, Debug)]
pub enum NoiseType {
    Gaussian,
    SaltPepper
}

pub fn noise(mut i: RgbaImage, noise_type: NoiseType, a: f32, b: u32) -> RgbaImage {
    match noise_type {
        NoiseType::Gaussian => {
            gaussian_noise_mut(&mut i, ((a as f64 * b as f64) % 100.0) / 100.0, a as f64, b.clone() as u64);
        },
        NoiseType::SaltPepper => {
            salt_and_pepper_noise_mut(&mut i, a as f64, b.clone() as u64);
        }
    }
    i
}