use std::fs;
use nalgebra::Vector4;

use super::render::Object3D;

pub fn unit_sphere(color: u32) -> Object3D{
    let mut vertices: Vec<Vector4<f32>> = vec![];
    let mut colors: Vec<u32> = vec![];
    let mut triangles: Vec<(usize, usize, usize)> = vec![];

    let radius = 1.0;
    let sector_count = 36;
    let stack_count = 18;

    let pi = std::f32::consts::PI;
    let sector_step = 2.0 * pi / sector_count as f32;
    let stack_step = pi / stack_count as f32;

    for i in 0..stack_count + 1 {
        let stack_angle = pi / 2.0 - i as f32 * stack_step;
        let xy = radius * stack_angle.cos();
        let z = radius * stack_angle.sin();

        for j in 0..sector_count + 1 {
            let sector_angle = j as f32 * sector_step;

            let x = xy * sector_angle.cos();
            let y = xy * sector_angle.sin();
            vertices.push(Vector4::new(x, y, z, 1.0));
            colors.push(color);
        }
    }

    for i in 0..stack_count {
        let k1 = i * (sector_count + 1);
        let k2 = k1 + sector_count + 1;

        for j in 0..sector_count {
            if i != 0 {
                triangles.push((k1 + j, k2 + j, k1 + j + 1));
            }

            if i != (stack_count - 1) {
                triangles.push((k1 + j + 1, k2 + j, k2 + j + 1));
            }
        }
    }

    Object3D::new(vertices, colors, vec![], triangles)
}

pub fn unit_cube(color: u32) -> Object3D{
    let cube_vertices = vec![
        Vector4::new(-1.0, -1.0, -1.0, 1.0),
        Vector4::new( 1.0, -1.0, -1.0, 1.0),
        Vector4::new( 1.0,  1.0, -1.0, 1.0),
        Vector4::new(-1.0,  1.0, -1.0, 1.0),
        Vector4::new(-1.0, -1.0,  1.0, 1.0),
        Vector4::new( 1.0, -1.0,  1.0, 1.0),
        Vector4::new( 1.0,  1.0,  1.0, 1.0),
        Vector4::new(-1.0,  1.0,  1.0, 1.0)
    ];

    let colors = vec![
        color, color, color, color,
        color, color, color, color
    ];

    let cube_triangles  = vec![
        (0, 1, 2), (2, 3, 0),
        (4, 5, 6), (6, 7, 4),
        (0, 4, 7), (7, 3, 0),
        (1, 5, 6), (6, 2, 1),
        (2, 6, 7), (7, 3, 2),
        (0, 4, 5), (5, 1, 0)
    ]; 

    Object3D::new(cube_vertices, colors, vec![], cube_triangles)
}

pub fn unit_plane(x_division: usize, z_division: usize, color: u32) -> Object3D {
    let mut vertices: Vec<Vector4<f32>> = vec![];
    let mut colors: Vec<u32> = vec![];
    let mut triangles: Vec<(usize, usize, usize)> = vec![];

    for x in 0..x_division {
        for z in 0..z_division {
            let x = x as f32 - x_division as f32 / 2.0;
            let z = z as f32 - z_division as f32 / 2.0;
            vertices.push(Vector4::new(x, 0.0, z, 1.0));
            colors.push(color);
        }
    }

    for x in 0..x_division-1 {
        for z in 0..z_division-1 {
            let a = x * z_division + z;
            let b = x * z_division + z + 1;
            let c = (x + 1) * z_division + z + 1;
            let d = (x + 1) * z_division + z;
            triangles.push((a, b, c));
            triangles.push((a, c, d));
        }
    }
    Object3D::new(vertices, colors, vec![], triangles)
}

pub fn read_obj(path: &str) -> Object3D {
    let content = fs::read_to_string(path).expect("Something went wrong reading the file");
    let mut vertices: Vec<Vector4<f32>> = vec![];
    let mut triangles: Vec<(usize, usize, usize)> = vec![];
    let mut colors: Vec<u32> = vec![];

    for line in content.lines(){
        let mut words = line.split_whitespace();
        match words.next() {
            Some("v") => {
                let x: f32 = words.next().unwrap().parse().expect("Error, could not parse vertex");
                let y: f32 = words.next().unwrap().parse().expect("Error, could not parse vertex");
                let z: f32 = words.next().unwrap().parse().expect("Error, could not parse vertex");
                vertices.push(Vector4::new(x, y, z, 1.0));
                colors.push(0xFF00EC);
            },
            Some("f") => {
                let a: usize = words.next().unwrap().parse().expect("Error, could not parse triangle");
                let b: usize = words.next().unwrap().parse().expect("Error, could not parse triangle");
                let c: usize = words.next().unwrap().parse().expect("Error, could not parse triangle");
                triangles.push((a-1, b-1, c-1));
            },
            _ => {}
        }
    }
    Object3D::new(vertices,colors, vec![], triangles)
}