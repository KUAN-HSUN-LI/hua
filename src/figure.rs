use png;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
pub trait Saver {
    fn save(&self, output_name: &str);
}

pub struct Figure {
    size: (u32, u32),
}

impl Figure {
    pub fn new(width: u32, height: u32) -> Self {
        Figure {
            size: (width, height),
        }
    }
}

impl Saver for Figure {
    fn save(&self, output_name: &str) {
        let path = Path::new(output_name);
        let file = File::create(path).expect("Failed to create file");
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.size.0, self.size.1);
        encoder.set_color(png::ColorType::RGB);
        let mut writer = encoder.write_header().unwrap();
        let (w, h) = self.size;
        let data = vec![0; (w * h * 3) as usize];
        writer.write_image_data(&data).unwrap();
    }
}
