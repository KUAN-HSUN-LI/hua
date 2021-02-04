use crate::{drawing, elements, preview::Preview};
use core::slice;
use drawing::canvas::{Canvas, Drawing};
use elements::fig::FigElm;
use elements::fig::FigureOption;
use elements::Element;
use minifb::{Key, Window, WindowOptions};
use png;
use std::io::BufWriter;
use std::path::Path;
use std::{fs::File, unimplemented};

use crate::color::RGBAColor;

#[allow(dead_code)]
pub struct Figure {
    size: (u32, u32),
    canvas: Canvas<RGBAColor>,
    elms: Vec<Box<dyn Element>>,
}

pub trait Saver {
    fn save(&self, output_name: &str);
}

impl Figure {
    #[allow(unused_variables)]
    pub fn new(width: u32, height: u32, options: Vec<FigureOption>) -> Self {
        let figelm = FigElm::new(options);
        Figure {
            size: (width, height),
            canvas: Canvas::<RGBAColor>::new(0, 0, width as u32, height as u32),
            elms: vec![Box::new(figelm)],
        }
    }

    #[allow(dead_code)]
    pub fn line(&self) {
        unimplemented!()
    }

    #[allow(dead_code)]
    pub fn title(&self) {
        unimplemented!()
    }

    #[allow(dead_code)]
    pub fn x_axis(&self) {
        unimplemented!()
    }

    #[allow(dead_code)]
    pub fn y_axis(&self) {
        unimplemented!()
    }

    #[allow(dead_code)]
    pub fn set_axis(&self) {
        unimplemented!()
    }

    #[allow(dead_code)]
    pub fn draw(&mut self) -> &mut Self {
        self.canvas.draw_background(RGBAColor(255, 255, 255, 255));
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
                    self.canvas.buf.as_ptr() as *mut u8,
                    self.canvas.buf.len() * 4,
                )
            })
            .unwrap();
    }
}

impl Preview for Figure {
    fn show(&self) {
        let buffer = &self.canvas.buf;
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
