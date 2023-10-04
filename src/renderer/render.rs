
use nalgebra::{Matrix4, Vector4,Vector3, Translation3, Point3, Unit};
use super::math::{remap, lerp_color};

pub struct Object3D{
    pub vertices: Vec<Vector4<f32>>,
    pub colors: Vec<u32>,
    pub edges: Vec<(usize, usize)>,
    pub triangles: Vec<(usize,usize,usize)>
}

impl Object3D {
    pub fn new(vertices: Vec<Vector4<f32>>, colors: Vec<u32>, edges: Vec<(usize, usize)>, triangles: Vec<(usize,usize,usize)>) -> Object3D {
        Object3D { vertices, colors, edges, triangles }
    }        
}


pub struct Light {
    pub position: Vector4<f32>,
    pub color: u32,
    pub intensity: f32,
}

pub struct Camera {
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub up: Vector4<f32>,
    pub position: Vector4<f32>,
    pub look_at: Vector4<f32>,
}

impl Camera {
    pub fn get_projection_matrix(&self, aspect_ratio: f32) -> Matrix4<f32> {
        let fov_rad = self.fov.to_radians();
        let f = 1.0 / (fov_rad / 2.0).tan();
        
        Matrix4::new(
            f / aspect_ratio, 0.0, 0.0, 0.0,
            0.0, f, 0.0, 0.0,
            0.0, 0.0, (self.far + self.near) / (self.near - self.far), -1.0,
            0.0, 0.0, (2.0 * self.far * self.near) / (self.near - self.far), 0.0
        )
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        let position3 = Vector3::new(self.position.x, self.position.y, self.position.z);
        let look_at3 = Vector3::new(self.look_at.x, self.look_at.y, self.look_at.z);
        let up3 = Vector3::new(self.up.x, self.up.y, self.up.z);

        // Calculate the new basis vectors
        let f = (look_at3 - position3).normalize();
        let r = up3.cross(&f).normalize();
        let u = f.cross(&r).normalize();

        let p = -position3.dot(&r);
        let q = -position3.dot(&u);
        let r_val = -position3.dot(&f);
        // Construct the 4x4 camera matrix
        Matrix4::new(
            r.x, r.y, r.z, p,
            u.x, u.y, u.z, q,
            f.x, f.y, f.z, r_val,
            0.0, 0.0, 0.0, 1.0
        )
    }

    pub fn rotate_around_look_at(&mut self, axis: Vector4<f32>, angle: f32) {
        let rotation_matrix = Matrix4::from_axis_angle(&Unit::new_normalize(Vector3::new(axis.x, axis.y, axis.z)), angle);
        let position3 = Vector4::new(self.position.x, self.position.y, self.position.z,1.0);
        let look_at3 = Vector4::new(self.look_at.x, self.look_at.y, self.look_at.z,1.0);
        let up3 = Vector4::new(self.up.x, self.up.y, self.up.z,1.0);

        let new_position = rotation_matrix * position3;
        let new_look_at = rotation_matrix * look_at3;
        let new_up = rotation_matrix * up3;

        self.position = Vector4::new(new_position.x, new_position.y, new_position.z, 1.0);
        self.look_at = Vector4::new(new_look_at.x, new_look_at.y, new_look_at.z, 1.0);
        self.up = Vector4::new(new_up.x, new_up.y, new_up.z, 1.0);
    }
}


fn draw_line(buffer: &mut [u32], dimensions: (usize, usize), x0: usize, y0: usize, x1: usize, y1: usize, c0: u32, c1: u32) {
    let mut x0 = x0 as isize;
    let mut y0 = y0 as isize;
    let x1 = x1 as isize;
    let y1 = y1 as isize;
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    while x0 >= 0 && y0 >= 0 && x0 < dimensions.0 as isize && y0 < dimensions.1 as isize {
        buffer[y0 as usize * dimensions.0 + x0 as usize] = lerp_color(c0, c1, remap(x0 as f32, 0.0, dimensions.0 as f32, 0.0, 1.0));

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;

        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn draw_triangle(buffer: &mut [u32], dimensions: (usize, usize), x0: usize, y0: usize, x1: usize, y1: usize, x2: usize, y2: usize, c0: u32, c1: u32, c2: u32) {
    draw_line(buffer, dimensions, x0, y0, x1, y1, c0, c1);
    draw_line(buffer, dimensions, x1, y1, x2, y2, c1, c2);
    draw_line(buffer, dimensions, x2, y2, x0, y0, c2, c0);
}

pub fn draw_object(buffer: &mut [u32], object: &Object3D, dimensions: (usize,usize), camera: &Camera, position: Vector4<f32>, rotation: Vector4<f32>, scale: Vector4<f32>, background_color: Option<u32>){
    let aspect_ratio = dimensions.0 as f32 / dimensions.1 as f32;
    let projection_matrix = camera.get_projection_matrix(aspect_ratio);
    let view_matrix = camera.get_view_matrix();

    let rotation_matrix = Matrix4::from_euler_angles(
        rotation.x,
        rotation.y,
        rotation.z
    );
    let position_point = Point3::new(position.x, position.y, position.z);
    let scaling_vec = Vector3::new(scale.x, scale.y, scale.z);
    let scaling_matrix = Matrix4::new_nonuniform_scaling_wrt_point(&scaling_vec, &position_point);
    let translation_matrix = Translation3::new(position.x, position.y, position.z).to_homogeneous();
    let transform_matrix = translation_matrix * rotation_matrix * scaling_matrix;

    let transformed_vertices: Vec<_> = object.vertices.iter()
        .map(|vertex| transform_matrix * vertex)
        .map(|vertex| view_matrix * vertex)
        .map(|vertex| projection_matrix * vertex)
        .collect();

    if let Some(bg_color) = background_color {
        for pixel in buffer.iter_mut() {
            *pixel = bg_color;
        }
    }

    for i in 0..transformed_vertices.iter().len(){
        let vertex = transformed_vertices[i];
        let perspective_vertex = vertex / vertex.w;
        let x = ((perspective_vertex.x + 1.0) * (dimensions.0 as f32) / 2.0) as usize;
        let y = ((-perspective_vertex.y + 1.0) * (dimensions.1 as f32) / 2.0) as usize;
        if x < dimensions.0 && y < dimensions.1 {
            buffer[y * dimensions.0 + x] = object.colors[i];
        }
    }

    for i in 0..object.edges.iter().len(){
        let &(start, end) = &object.edges[i];
        let start_vertex = transformed_vertices[start] / transformed_vertices[start].w;
        let end_vertex = transformed_vertices[end] / transformed_vertices[end].w;
        let x0 = ((start_vertex.x + 1.0) * (dimensions.0 as f32) / 2.0) as usize;
        let y0 = ((-start_vertex.y + 1.0) * (dimensions.1 as f32) / 2.0) as usize;

        let x1 = ((end_vertex.x + 1.0) * (dimensions.0 as f32) / 2.0) as usize;
        let y1 = ((-end_vertex.y + 1.0) * (dimensions.1 as f32) / 2.0) as usize;
        draw_line(buffer, dimensions, x0, y0, x1, y1, object.colors[start], object.colors[end]);
    }

    for i in 0..object.triangles.iter().len(){
        let &(a, b, c) = &object.triangles[i];
        let a_vertex = transformed_vertices[a] / transformed_vertices[a].w;
        let b_vertex = transformed_vertices[b] / transformed_vertices[b].w;
        let c_vertex = transformed_vertices[c] / transformed_vertices[c].w;

        let x0 = ((a_vertex.x + 1.0) * (dimensions.0 as f32) / 2.0) as usize;
        let y0 = ((-a_vertex.y + 1.0) * (dimensions.1 as f32) / 2.0) as usize;

        let x1 = ((b_vertex.x + 1.0) * (dimensions.0 as f32) / 2.0) as usize;
        let y1 = ((-b_vertex.y + 1.0) * (dimensions.1 as f32) / 2.0) as usize;

        let x2 = ((c_vertex.x + 1.0) * (dimensions.0 as f32) / 2.0) as usize;
        let y2 = ((-c_vertex.y + 1.0) * (dimensions.1 as f32) / 2.0) as usize;

        draw_triangle(buffer, dimensions, x0, y0, x1, y1, x2, y2, object.colors[a], object.colors[b], object.colors[c]);
    }
}