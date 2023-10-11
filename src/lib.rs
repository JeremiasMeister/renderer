pub mod renderer;
pub mod modifiers;

use std::io;
use image::imageops::FilterType;
use nalgebra::Vector4;
use minifb::{Key, WindowOptions, Window, Scale};

use renderer::render::{draw_object, Camera};
use modifiers::io::load_texture;

fn run_debug_scene() {
    // loading object from obj
    let path = "resources/monkey.obj";
    let monkey = renderer::reader::read_obj(path);
    // creating unit cube
    let cube = renderer::reader::unit_cube(0xFFFFFF);
    // creating unit plane
    let plane = renderer::reader::unit_plane(10, 10, 0x00FF00);
    // creating unit sphere
    let sphere = renderer::reader::unit_sphere(0xFF0000);

    let mut camera = Camera {
        fov: 90.0,
        near: 0.1,
        up: Vector4::new(0.0, 1.0, 0.0, 0.0),
        far: 1000.0,
        position: Vector4::new(0.0, 0.0, -20.0, 1.0),
        look_at: Vector4::new(0.0, 0.0, 0.0, 1.0),
    };
    let dimensions = (1024, 800);
    let mut buffer = vec![0u32; dimensions.0 * dimensions.1];


    let mut window = Window::new(
        "DEBUG SCENE",
        dimensions.0,
        dimensions.1,
        WindowOptions {
            scale: Scale::X1,
            ..Default::default()
        },
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let monkey_pos = Vector4::new(0.0, 0.0, 0.0, 0.0);
    let cube_pos = Vector4::new(-2.0, 2.0, 0.0, 0.0);
    let plane_pos = Vector4::new(-5.0, 4.0, 2.5, 0.0);
    let sphere_pos = Vector4::new(2.0, 2.5, 5.0, 0.0);

    let rotation = Vector4::new(0.0, 0.0, 0.0, 0.0);
    let rotation_2 = Vector4::new(0.0, 30.0, 0.0, 0.0);
    let rotation_3 = Vector4::new(0.0, 10.0, 180.0, 0.0);
    let scale = Vector4::new(1.0, 1.0, 1.0, 0.0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw_object(&mut buffer, &monkey, dimensions, &camera, monkey_pos, rotation_3, scale, Some(0x000000));
        draw_object(&mut buffer, &cube, dimensions, &camera, cube_pos, rotation_2, scale, None);
        draw_object(&mut buffer, &plane, dimensions, &camera, plane_pos, rotation, scale, None);
        draw_object(&mut buffer, &sphere, dimensions, &camera, sphere_pos, rotation, scale, None);
        window
            .update_with_buffer(&buffer, dimensions.0, dimensions.1)
            .unwrap();

        if window.get_mouse_down(minifb::MouseButton::Right) {
            if window.is_key_down(Key::Space) {
                camera.position.y += 0.1;
            }
        } else {
            if window.is_key_down(Key::A) {
                camera.rotate_around_look_at(camera.up, 0.1);
            }
            if window.is_key_down(Key::D) {
                camera.rotate_around_look_at(camera.up, -0.1);
            }
            if window.is_key_down(Key::Space) {
                camera.position.y -= 0.1;
            }
        }
    }
}

fn run_heightmap_display() {
    let window_size: (usize, usize) = (1024, 800);
    let dimensions: (usize, usize) = (256, 256);
    let image_filter = FilterType::Nearest;
    let mut heightmap = load_texture("resources/map_height.png");
    let mut colormap = load_texture("resources/map_color.png");

    match modifiers::modifiers::scale_image(&mut heightmap, (dimensions.0 as u32, dimensions.1 as u32), image_filter) {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    match modifiers::modifiers::scale_image(&mut colormap, (dimensions.0 as u32, dimensions.1 as u32), image_filter) {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let mut plane = renderer::reader::unit_plane(dimensions.0 as usize, dimensions.1 as usize, 0x00FF00);
    let mut camera = Camera {
        fov: 90.0,
        near: 0.1,
        up: Vector4::new(0.0, 1.0, 0.0, 0.0),
        far: 1000.0,
        position: Vector4::new(0.0, 5.0, -20.0, 1.0),
        look_at: Vector4::new(0.0, 0.0, 0.0, 1.0),
    };
    camera.rotate_around_look_at(camera.up, 45.0);

    let mut buffer = vec![0u32; window_size.0 * window_size.1];
    let mut window = Window::new(
        "HEIGHTMAP DISPLAY",
        window_size.0,
        window_size.1,
        WindowOptions {
            scale: Scale::X1,
            ..Default::default()
        },
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    //displace plane
    modifiers::modifiers::displace_plane(&mut plane, &heightmap, 30.0);
    modifiers::modifiers::colorize_plane(&mut plane, &colormap);


    let rotation = Vector4::new(0.0, 0.0, 0.0, 0.0);
    let uni_size = 10.0;
    let scale = Vector4::new(uni_size / dimensions.0 as f32, uni_size / dimensions.0 as f32, uni_size / dimensions.0 as f32, 0.0);
    let position = Vector4::new(0.0, 1.0, 0.0, 0.0);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw_object(&mut buffer, &plane, window_size, &camera, position, rotation, scale, Some(0x000000));
        window
            .update_with_buffer(&buffer, window_size.0, window_size.1)
            .unwrap();

        if window.get_mouse_down(minifb::MouseButton::Right) {
            if window.is_key_down(Key::Space) {
                camera.position.y += 0.1;
            }
        } else {
            if window.is_key_down(Key::A) {
                camera.rotate_around_look_at(camera.up, 0.1);
            }
            if window.is_key_down(Key::D) {
                camera.rotate_around_look_at(camera.up, -0.1);
            }
            if window.is_key_down(Key::Space) {
                camera.position.y -= 0.1;
            }
        }
        if window.get_mouse_down(minifb::MouseButton::Left){
            let buffer_rgba = modifiers::modifiers::buffer_to_image_buffer_rgba(&buffer,(window_size.0 as u32, window_size.1 as u32));
            let buffer_rgb = modifiers::modifiers::buffer_to_image_buffer_rgb(&buffer,(window_size.0 as u32, window_size.1 as u32));
            modifiers::modifiers::save_image_to_desktop(&buffer_rgb, "RGB", "test");
            modifiers::modifiers::save_image_to_desktop(&buffer_rgba, "RGBA", "test");
        }
    }
}

pub fn run() {
    println!("Please enter the scene you want to Open:");
    println!("0: Open the Debug Scene");
    println!("1: Open the Heightmap Renderer");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read from stdin");
    let trimmed = input.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => {
            if i == 0 {
                run_debug_scene();
            } else if i == 1 {
                run_heightmap_display();
            } else {
                println!("Can't detect scene to open")
            }
        }
        Err(e) => {
            println!("Not a valid scene value provided");
        }
    }
}

fn main() {
    run();
}