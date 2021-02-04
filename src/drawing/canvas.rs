use super::shape::Rect;
#[allow(unused_imports)]
use crate::layout::layout;

#[allow(dead_code)]
pub struct Canvas<T: Default + Clone + Copy> {
    pub buf: Vec<T>,
    rect: Rect,
}

pub trait Drawing<U: Default + Clone + Copy> {
    // fn split_layout() -> Vec<U>;
    fn draw_background(&mut self, color: U);
}

impl<U: Default + Clone + Copy> Canvas<U> {
    pub fn new(x0: u32, y0: u32, x1: u32, y1: u32) -> Self {
        let w = x1 - x0;
        let h = y1 - y0;
        Canvas {
            buf: vec![U::default(); (w * h) as usize],
            rect: Rect(x0, y0, x1, y1),
        }
    }
}

impl<U: Default + Clone + Copy> Drawing<U> for Canvas<U> {
    // fn split_layout(&mut self) -> Vec<U> {
    //     self.buf
    // }
    fn draw_background(&mut self, color: U) {
        self.buf.iter_mut().for_each(|c| *c = color);
    }
}
