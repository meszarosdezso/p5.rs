use crate::color::Color;

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

impl Point {
    pub fn new(x: i32, y: i32, color: Color) -> Self {
        Self { x, y, color }
    }
}
