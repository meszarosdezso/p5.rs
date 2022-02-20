use crate::color::Color;

#[derive(Clone)]
pub struct Canvas {
    pub(crate) width: u32,
    pub(crate) height: u32,

    /// Vector of RGBA values
    pub(crate) pixels: Vec<Color>,
}

impl Canvas {
    pub fn from_size(width: u32, height: u32) -> Self {
        let capacity = (width * height) as usize;
        let mut pixels = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            pixels.push([0, 0, 0, 255]);
        }

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn background(&mut self, color: Color) {
        for pixel in self.pixels.iter_mut() {
            *pixel = color;
        }
    }

    pub(crate) fn get_frame(&self) -> &Vec<Color> {
        &self.pixels
    }
}