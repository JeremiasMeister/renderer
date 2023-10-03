mod renderer;
use nalgebra::Vector4;
use minifb::{Key, WindowOptions, Window, Scale};

use renderer::render::{draw_object, Camera};


fn main() {      
    // loading object from obj
    let path = "resources/monkey.obj";
    let monkey = renderer::reader::read_obj(path);
    // creating unit cube
    let cube = renderer::reader::unit_cube(0xFFFFFF);
    // creating unit plane
    let plane = renderer::reader::unit_plane(10, 10, 0x00FF00);
    // creating unit sphere
    let sphere = renderer::reader::unit_sphere(0xFF0000);

    let camera = Camera {
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
        "Render Object",
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
    }
    
}