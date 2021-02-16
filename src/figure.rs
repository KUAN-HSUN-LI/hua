use crate::{
    backend::canvas::Canvas,
    build::{builder::CanvasBuilder, utils::LayoutPostion::*},
    elements::ElmType,
};
use crate::{elements, preview::Preview};
use core::slice;
use elements::{axis::AxisElm, fig::FigureOption};
use elements::{axis::AxisOption, fig::FigElm};
use minifb::{Key, Window, WindowOptions};
use png;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[allow(dead_code)]
pub struct Figure {
    size: (u32, u32),
    canvas: Canvas,
    fig_elm: FigElm,
    label_elms: [Vec<ElmType>; 4], // top, bottom, left, right
}

pub trait Saver {
    fn save(&self, output_name: &str);
}

impl Figure {
    #[allow(unused_variables)]
    pub fn new(width: u32, height: u32, options: Vec<FigureOption>) -> Self {
        const INIT: Vec<ElmType> = vec![];
        Figure {
            size: (width, height),
            canvas: Canvas::new(width, height),
            fig_elm: FigElm::new(options),
            label_elms: [INIT; 4], // top, bottom, left, right
        }
    }

    #[allow(dead_code)]
    pub fn line(&self) -> Self {
        todo!()
    }

    #[allow(dead_code)]
    pub fn title(&self) -> Self {
        todo!()
    }

    #[allow(dead_code)]
    pub fn x_axis(&mut self, options: Vec<AxisOption>) -> &mut Self {
        self.label_elms[Bottom as usize].push(ElmType::Axis(AxisElm::new(options)));
        self
    }

    #[allow(dead_code)]
    pub fn y_axis(&mut self, options: Vec<AxisOption>) -> &mut Self {
        self.label_elms[Left as usize].push(ElmType::Axis(AxisElm::new(options)));
        self
    }

    #[allow(dead_code)]
    pub fn set_axis(&self) -> Self {
        todo!()
    }

    #[allow(dead_code)]
    pub fn x_label(&self) -> Self {
        todo!()
    }

    #[allow(dead_code)]
    pub fn y_label(&self) -> Self {
        todo!()
    }

    #[allow(dead_code)]
    pub fn draw(&mut self) -> &mut Self {
        CanvasBuilder::run(&mut self.canvas, &self.fig_elm, &self.label_elms);
        self
    }

    #[allow(dead_code)]
    fn parse_option() {}
}

impl Saver for Figure {
    fn save(&self, output_name: &str) {
        let path = Path::new(output_name);
        let file = File::create(path).expect("Failed to create file");
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.size.0 as u32, self.size.1 as u32);
        encoder.set_color(png::ColorType::RGBA);
        let mut writer = encoder.write_header().unwrap();
        writer
            .write_image_data(unsafe {
                slice::from_raw_parts(
                    self.canvas.borrow_buf().as_ptr() as *mut u8,
                    self.canvas.borrow_buf().len() * 4,
                )
            })
            .unwrap();
    }
}

impl Preview for Figure {
    fn show(&self) {
        let buffer = &self.canvas.borrow_buf();
        let (w, h) = self.size;
        let mut window =
            Window::new("Preview", w as usize, h as usize, WindowOptions::default()).unwrap();
        while window.is_open() && !window.is_key_down(Key::Escape) {
            window
                .update_with_buffer(unsafe { std::mem::transmute(&buffer[..]) })
                .unwrap();
        }
    }
}
