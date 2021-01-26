use super::shape::Rect;

#[allow(dead_code)]
pub struct Canvas<T> {
    pub buf: Vec<T>,
    rect: Rect,
}

impl<T> Canvas<T> {
    pub fn new(x0: u32, y0: u32, x1: u32, y1: u32) -> Self {
        Canvas {
            buf: Vec::new(),
            rect: Rect(x0, y0, x1, y1),
        }
    }
}
