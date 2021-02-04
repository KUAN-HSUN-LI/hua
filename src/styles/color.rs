trait Color {}

#[derive(Debug, Default, Clone, Copy)]
pub struct RGBColor(pub(crate) u8, pub(crate) u8, pub(crate) u8);

impl RGBColor {
    #[allow(dead_code)]
    fn rgb2rgba(&self) -> RGBAColor {
        RGBAColor(self.0, self.1, self.2, 255)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RGBAColor(pub(crate) u8, pub(crate) u8, pub(crate) u8, pub(crate) u8);

impl Default for RGBAColor {
    fn default() -> Self {
        RGBAColor(0, 0, 0, 255)
    }
}

pub const BLACK: RGBColor = RGBColor(0, 0, 0);
pub const RED: RGBColor = RGBColor(255, 0, 0);
pub const GREEN: RGBColor = RGBColor(0, 255, 0);
pub const BLUE: RGBColor = RGBColor(0, 0, 255);
pub const YELLOW: RGBColor = RGBColor(255, 255, 0);
pub const MAGENTA: RGBColor = RGBColor(255, 0, 255);
pub const CYAN: RGBColor = RGBColor(0, 255, 255);
pub const WHITE: RGBColor = RGBColor(255, 255, 255);
