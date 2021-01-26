trait Color {}

#[derive(Debug, Clone, Copy)]
pub struct RGBColor<T>(pub(crate) T, pub(crate) T, pub(crate) T);

impl<T> RGBColor<T> {
    #[allow(dead_code)]
    fn rgb2rgba(&self) -> Self {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RGBAColor<T>(pub(crate) T, pub(crate) T, pub(crate) T, pub(crate) T);

pub const BLACK: RGBColor<u8> = RGBColor(0, 0, 0);
pub const RED: RGBColor<u8> = RGBColor(255, 0, 0);
pub const GREEN: RGBColor<u8> = RGBColor(0, 255, 0);
pub const BLUE: RGBColor<u8> = RGBColor(0, 0, 255);
pub const YELLOW: RGBColor<u8> = RGBColor(255, 255, 0);
pub const MAGENTA: RGBColor<u8> = RGBColor(255, 0, 255);
pub const CYAN: RGBColor<u8> = RGBColor(0, 255, 255);
pub const WHITE: RGBColor<u8> = RGBColor(255, 255, 255);
