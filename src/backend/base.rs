use crate::color::RGBAColor;

use super::{
    drawing::{line, rect},
    polygon,
};

pub struct Point(u32, u32);

pub trait DrawingBase {
    fn get_size(&self) -> (u32, u32);
    fn borrow_mut_buf(&mut self) -> &mut Vec<RGBAColor>;
    fn focus(&self, upper_left: Point, lower_right: Point) -> &Self
    where
        Self: Sized;
    fn draw_line(&mut self, from: (i32, i32), to: (i32, i32), line_width: u32)
    where
        Self: Sized,
    {
        line::draw_line(self, from, to, line_width);
    }
    fn draw_circle(&mut self);
    fn draw_rect(
        &mut self,
        upper_left: (u32, u32),
        bottom_right: (u32, u32),
        fill: Option<RGBAColor>,
    ) where
        Self: Sized,
    {
        rect::draw_rect(self, upper_left, bottom_right, fill)
    }
    fn draw_polygon(&mut self) {
        polygon::draw_polygon();
    }
    fn draw_text(&mut self);
}
