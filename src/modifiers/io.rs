
use image::{ImageBuffer, Rgba};

pub fn load_texture(path: &str) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    match image::open(path) {
        Ok(img) => {
            return img.to_rgba8();
        },
        Err(e) => panic!("Could not load texture: {}", e),
    }
}