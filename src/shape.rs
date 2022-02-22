use crate::canvas::Canvas;

pub mod line;
pub mod point;

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas);
}
