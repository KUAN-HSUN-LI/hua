use super::{Element, Option};
use crate::color::RGBColor;

#[allow(dead_code)]
pub struct FigElm {
    background_color: RGBColor,
}

#[derive(Clone, Copy)]
pub enum FigureOption {
    BackGroundColor(RGBColor),
    Border(f32),
}

impl Option for FigureOption {}

impl Element for FigElm {
    #[allow(unused_variables)]
    fn parse_options<T: Option>(&self, options: Vec<T>) -> Self
    where
        Self: Sized,
    {
        return FigElm {
            background_color: RGBColor(0, 0, 0),
        };
    }
}

impl FigElm {
    pub fn new(options: Vec<FigureOption>) -> Self {
        FigElm {
            background_color: RGBColor(0, 0, 0),
        }
        .parse_options(options)
    }
}
