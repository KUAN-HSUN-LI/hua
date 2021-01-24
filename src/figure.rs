use core::slice;
use minifb::{Key, Window, WindowOptions};
use png;
use preview::Preview;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::{preview, RGBAColor};
pub trait Saver {
    fn save(&self, output_name: &str);
}

pub struct Figure {
    pub size: (usize, usize),
    pub buffer: Vec<RGBAColor<u8>>,
}

impl Figure {
    pub fn new(width: usize, height: usize) -> Self {
        Figure {
            size: (width, height),
            buffer: vec![RGBAColor::<u8>(0, 0, 0, 0); width * height],
        }
    }
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
                slice::from_raw_parts(self.buffer.as_ptr() as *mut u8, self.buffer.len() * 4)
            })
            .unwrap();
    }
}

impl Preview for Figure {
    fn show(&self) {
        let buffer = &self.buffer;
        let mut window = Window::new(
            "Preview",
            self.size.0,
            self.size.1,
            WindowOptions::default(),
        )
        .unwrap();
        while window.is_open() && !window.is_key_down(Key::Escape) {
            window
                .update_with_buffer(unsafe { std::mem::transmute(&buffer[..]) })
                .unwrap();
        }
    }
}
