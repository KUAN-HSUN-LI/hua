use core::panic;

use crate::{
    backend::base::DrawingBase,
    elements::{axis::AxisElm, fig::FigElm, ElmType},
};

use super::utils::LayoutPostion::*;
use super::{axis::AxisBuilder, utils::Vector};

pub struct CanvasBuilder<'a, DB: DrawingBase> {
    canvas: &'a mut DB,
    border: [u32; 4],
    label_areas_width: [u32; 4],
    axis_elms: [Option<&'a AxisElm>; 4],
}

impl<'a, DB: DrawingBase> CanvasBuilder<'a, DB> {
    pub fn run(canvas: &'a mut DB, fig_elm: &FigElm, label_elms: &[Vec<ElmType>; 4]) {
        let mut builder = CanvasBuilder::new(canvas);
        builder.draw_background(fig_elm);
        builder.build_border(fig_elm);
        builder.build_label_areas_width(label_elms);
        builder.draw_axis();
        println!("{:?}", builder.label_areas_width);
    }
    fn new(canvas: &'a mut DB) -> Self {
        CanvasBuilder {
            canvas,
            border: [0; 4],
            label_areas_width: [0; 4],
            axis_elms: [None; 4],
        }
    }

    fn build_border(&mut self, fig_elm: &FigElm) {
        let (w, h) = self.canvas.get_size();
        let border_pct = 0.01;

        // top, bottom
        self.border[0] = (h as f32 * border_pct) as u32;
        self.border[1] = (h as f32 * border_pct) as u32;

        // left, right
        self.border[2] = (w as f32 * border_pct) as u32;
        self.border[3] = (w as f32 * border_pct) as u32;
    }

    fn build_label_areas_width(&mut self, label_elms: &'a [Vec<ElmType>; 4]) {
        // top, bottom, left, right
        for i in 0..4 {
            self.label_areas_width[i] = self.build_label_area_width(i, &label_elms[i])
        }
    }

    fn build_label_area_width(&mut self, pos: usize, elms: &'a Vec<ElmType>) -> u32 {
        let mut size = 0;
        for elm in elms.iter() {
            match elm {
                ElmType::Axis(elm) => {
                    self.axis_elms[pos] = Some(elm);
                    size += elm.get_size();
                }
                _ => panic!("Wrong Lable Type matched."),
            }
        }
        size
    }

    fn draw_background(&mut self, fig_elm: &FigElm) {
        let color = fig_elm.get_background_color();
        let (w, h) = self.canvas.get_size();
        self.canvas.draw_rect((0, 0), (w, h), Some(color));
    }

    fn draw_axis(&mut self) {
        for (pos, axis_elm) in self.axis_elms.iter().enumerate() {
            if axis_elm.is_some() {
                // let axis_elm.unwrap().get_size();
                println!("{:?}", self.calc_axis_vector(pos));
                AxisBuilder::run(self.canvas, axis_elm.unwrap(), self.calc_axis_vector(pos));
            }
        }
    }

    //  0-----1
    //  |     |
    //  |     |
    //  3-----2

    fn calc_axis_vector(&self, pos: usize) -> Vector {
        let (w, h) = self.canvas.get_size();
        let mut points = [(0, 0); 4];
        points[0] = (
            self.border[Left as usize] + self.label_areas_width[Left as usize],
            self.border[Top as usize] + self.label_areas_width[Top as usize],
        );
        points[1] = (
            w - self.border[Right as usize] + self.label_areas_width[Right as usize],
            self.border[Top as usize] + self.label_areas_width[Top as usize],
        );

        points[2] = (
            w - self.border[Right as usize] - self.label_areas_width[Right as usize],
            h - self.border[Bottom as usize] - self.label_areas_width[Bottom as usize],
        );
        points[3] = (
            self.border[Left as usize] + self.label_areas_width[Left as usize],
            h - self.border[Bottom as usize] - self.label_areas_width[Bottom as usize],
        );
        println!("{}", pos);
        println!("{:?}", self.border);
        println!("{:?}", self.label_areas_width);
        println!("{:?}", points);
        if pos == Top as usize {
            Vector::new(points[0], points[1])
        } else if pos == Bottom as usize {
            Vector::new(points[3], points[2])
        } else if pos == Left as usize {
            Vector::new(points[3], points[0])
        } else {
            Vector::new(points[2], points[1])
        }
    }
}
