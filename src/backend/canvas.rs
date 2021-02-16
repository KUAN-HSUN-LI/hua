use crate::color::RGBAColor;

use super::base::{DrawingBase, Point};

#[allow(dead_code)]
pub struct Canvas {
    buf: Vec<RGBAColor>,
    coord: [(u32, u32); 2], // top left and bottom right
}

impl Canvas {
    #[allow(dead_code, unused_variables)]
    pub fn new(width: u32, height: u32) -> Self {
        Canvas {
            buf: vec![RGBAColor::default(); (width * height) as usize],
            coord: [(0, 0), (width, height)],
        }
    }
    pub fn get_size(&self) -> (u32, u32) {
        (
            self.coord[1].0 - self.coord[0].0,
            self.coord[1].1 - self.coord[0].1,
        )
    }
    pub fn draw_background(&mut self, color: RGBAColor) {
        self.draw_rect(self.coord[0], self.coord[1], Some(color));
    }
    pub fn borrow_buf(&self) -> &Vec<RGBAColor> {
        &self.buf
    }
    fn borrow_mut_buf(&mut self) -> &mut Vec<RGBAColor> {
        &mut self.buf
    }
}

impl DrawingBase for Canvas {
    fn borrow_mut_buf(&mut self) -> &mut Vec<RGBAColor> {
        self.borrow_mut_buf()
    }
    fn get_size(&self) -> (u32, u32) {
        self.get_size()
    }
    #[allow(unused_variables)]
    fn focus(&self, upper_left: Point, lower_right: Point) -> &Self {
        todo!()
    }
    fn draw_circle(&mut self) {
        todo!()
    }

    fn draw_text(&mut self) {
        todo!()
    }
}
