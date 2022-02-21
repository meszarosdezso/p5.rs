use super::point::Point;
use super::Drawable;
use crate::algorithms::xiaolin_wu::xiaolin_wu_line;
use crate::canvas::Canvas;

pub struct Line {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
}

impl Line {
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Self { x0, y0, x1, y1 }
    }
}

impl Drawable for Line {
    fn draw(&self, canvas: &mut Canvas) {
        let pixels = xiaolin_wu_line(self.x0, self.y0, self.x1, self.y1);

        for Point { x, y, color } in pixels.iter() {
            let index = x + (y * canvas.height as i32);
            if let Some(p) = canvas.pixels.get_mut(index as usize) {
                *p = *color;
            }
        }
    }
}
