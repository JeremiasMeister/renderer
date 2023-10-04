use super::super::renderer::render;
use render::Object3D;
use image::{ImageBuffer, Rgba, imageops::FilterType};
use std::error::Error;


fn rgba_to_u32(rgba: image::Rgba<u8>) -> u32 {
    let [r, g, b, a] = rgba.0;
    ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

pub fn displace_plane(plane: &mut Object3D, heightmap: &ImageBuffer<Rgba<u8>, Vec<u8>>, scale: f32) {
    let dimensions = heightmap.dimensions();
    let width = dimensions.0;
    let height = dimensions.1;
    for x in 0..width{
        for y in 0..height{
            match plane.vertices.get_mut((x + y * width) as usize) {
                Some(vertex) => {
                    let pixel = heightmap.get_pixel(x, y);
                    let height = (1.0 - (pixel[0] as f32 / 255.0)) * scale;
                    vertex[1] = height;
                    plane.vertices[(x + y * width) as usize] = vertex.clone();
                },
                None => panic!("Could not get vertex"),
            }            
        }
    }
}

pub fn colorize_plane(plane: &mut Object3D, colormap: &ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let dimensions = colormap.dimensions();
    let width = dimensions.0;
    let height = dimensions.1;
    for x in 0..width{
        for y in 0..height{
            match plane.colors.get_mut((x + y * width) as usize) {
                Some(color) => {
                    let pixel = colormap.get_pixel(x, y);
                    *color = rgba_to_u32(*pixel);
                },
                None => (),
            }            
        }
    }
}

pub fn scale_image(buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, target_size: (u32, u32), scale_method: FilterType) -> Result<(), Box<dyn Error>> {
    let (target_width, target_height) = target_size;

    if target_width == 0 || target_height == 0 {
        return Err("Target size should be greater than zero".into());
    }

    let scaled_image = image::imageops::resize(buffer, target_width, target_height, scale_method);
    *buffer = scaled_image;

    Ok(())
}