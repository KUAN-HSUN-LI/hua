use crate::{backend::base::DrawingBase, elements::axis::AxisElm};
use crate::{build::utils::Vector, BLACK};

pub struct AxisBuilder<'a, DB: DrawingBase> {
    canvas: &'a mut DB,
    vector: Vector,
    axis_width: u32,
    tick_width: u32,
    range: (i32, i32),
}

impl<'a, DB: DrawingBase> AxisBuilder<'a, DB> {
    pub fn run(canvas: &'a mut DB, axis_elm: &AxisElm, vector: Vector) {
        let mut builder = AxisBuilder::new(canvas, axis_elm, vector);
        builder.draw();
    }
    fn new(canvas: &'a mut DB, axis_elm: &AxisElm, vector: Vector) -> Self {
        AxisBuilder {
            canvas,
            vector,
            axis_width: axis_elm.get_size(),
            tick_width: axis_elm.get_size(),
            range: axis_elm.get_range().unwrap_or((0, 0)),
        }
    }

    fn draw(&mut self) {
        let (upper_left, bottom_right) = self.get_rect_point();
        println!("{:?} {:?}", upper_left, bottom_right);
        self.canvas
            .draw_rect(upper_left, bottom_right, Some(BLACK.to_rgba()));
    }

    fn get_rect_point(&self) -> ((u32, u32), (u32, u32)) {
        let upper_left: (u32, u32);
        let bottom_right: (u32, u32);
        if self.vector.from().0 == self.vector.to().0 {
            if self.vector.from().1 < self.vector.to().1 {
                upper_left = (self.vector.from().0 - self.axis_width, self.vector.from().1);
                bottom_right = self.vector.to();
            } else {
                upper_left = (self.vector.to().0 - self.axis_width, self.vector.to().1);
                bottom_right = self.vector.from();
            }
        } else {
            if self.vector.from().0 < self.vector.to().0 {
                upper_left = self.vector.from();
                bottom_right = (self.vector.to().0, self.vector.to().1 + self.axis_width);
            } else {
                upper_left = self.vector.to();
                bottom_right = (self.vector.from().0, self.vector.from().1 + self.axis_width);
            }
        }

        (upper_left, bottom_right)
    }
}
