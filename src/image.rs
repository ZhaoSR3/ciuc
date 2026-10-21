use std::path::Path;

use image::{Rgba, RgbaImage, imageops::FilterType};

use crate::models::SpriteFrame;

pub fn load_rgba_image<P>(path: &P) -> Result<RgbaImage, Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    let rgba_image = image::open(path)?.to_rgba8();

    Ok(rgba_image)
}

pub fn get_pixel(image: &RgbaImage, x: u32, y: u32) -> &Rgba<u8> {
    image.get_pixel(x, y)
}

pub fn crop_image(image: &RgbaImage, sprite_frame: &SpriteFrame) -> RgbaImage {
    image::imageops::crop_imm(
        image,
        sprite_frame.x,
        sprite_frame.y,
        sprite_frame.width,
        sprite_frame.height,
    )
    .to_image()
}

pub fn resize_image(image: &RgbaImage, width: u32, height: u32) -> RgbaImage {
    image::imageops::resize(image, width, height, FilterType::Lanczos3)
}
