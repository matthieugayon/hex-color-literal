use hex_color::*;

#[derive(Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Color {
    pub fn from_u8_array(rgb_array: [u8; 3]) -> Color {
        Color { 
            r: f32::from(rgb_array[0]) / 255.0, 
            g: f32::from(rgb_array[1])/ 255.0, 
            b: f32::from(rgb_array[2]) / 255.0, 
            a: 1. 
        }
    }

    pub const fn from_rgb_array(rgb_array: [f32; 3]) -> Color {
        Color { r: rgb_array[0], g: rgb_array[1], b: rgb_array[2], a: 1. }
    }
}

const fn color(rgb_array: [f32; 3]) -> Color {
    return Color::from_rgb_array(rgb_array)
}

#[test]
fn test_macro(){
    const white: Color = color(hex_color!("#B00"));

    println!("{:?}", white);
}