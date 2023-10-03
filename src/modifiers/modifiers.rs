use super::super::renderer::render;
use render::Object3D;
use image::{ImageBuffer, Rgba};


fn rgba_to_u32(rgba: image::Rgba<u8>) -> u32 {
    let [r, g, b, a] = rgba.0;
    ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

pub fn displace_plane(plane: &mut Object3D, heightmap: &ImageBuffer<Rgba<u8>, Vec<u8>>, scale: f32) {
    let dimensions = heightmap.dimensions();
    let width = dimensions.0;
    let height = dimensions.1;
    for x in 0..width-1{
        for y in 0..height-1{
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
    for x in 0..width-1{
        for y in 0..height-1{
            match plane.colors.get_mut((x + y * width) as usize) {
                Some(color) => {
                    let pixel = colormap.get_pixel(x, y);
                    *color = rgba_to_u32(*pixel);
                },
                None => panic!("Could not get vertex"),
            }            
        }
    }
}