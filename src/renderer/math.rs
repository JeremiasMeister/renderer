pub fn lerp(a: u8, b: u8, t: f32) -> u8 {
    ((1.0 - t) * a as f32 + t * b as f32) as u8
}

pub fn lerp_color(a: u32, b: u32, t: f32) -> u32 {
    let a = (a >> 16, (a >> 8) & 0xFF, a & 0xFF);
    let b = (b >> 16, (b >> 8) & 0xFF, b & 0xFF);
    let r = lerp(a.0 as u8, b.0 as u8, t);
    let g = lerp(a.1 as u8, b.1 as u8, t);
    let b = lerp(a.2 as u8, b.2 as u8, t);
    (r as u32) << 16 | (g as u32) << 8 | b as u32
}

pub fn remap(value: f32, old_min: f32, old_max: f32, new_min: f32, new_max: f32) -> f32 {
    if value < old_min {
        return new_min;
    }
    
    (value - old_min) / (old_max - old_min) * (new_max - new_min) + new_min
}