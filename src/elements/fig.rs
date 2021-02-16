use super::{Element, ElmOption};
use crate::color::{RGBAColor, RGBColor};

#[allow(dead_code)]
pub struct FigElm {
    background_color: RGBAColor,
    border: f32,
}

impl Element for FigElm {}

impl FigElm {
    pub fn new(options: Vec<FigureOption>) -> Self {
        let mut fig = FigElm {
            background_color: RGBAColor::default(),
            border: 1.0,
        };
        fig.parse_options(options);
        fig
    }

    fn set_background_color(&mut self, color: RGBAColor) {
        self.background_color = color;
    }

    pub fn get_background_color(&self) -> RGBAColor {
        self.background_color
    }

    fn set_border(&mut self, border: f32) {
        self.border = border;
    }

    pub fn get_border(&self) -> f32 {
        self.border
    }
}

// #[derive(Clone, Copy)]
pub enum FigureOption {
    BackGroundColor(RGBColor),
    BorderPCT(f32),
}

// impl FigureOption {
//     fn _match_elm_param(&self, fig_elm: &mut FigElm) {}
// }

impl ElmOption<FigElm> for FigureOption {
    fn match_elm_param(&self, fig_elm: &mut FigElm) {
        #[allow(unreachable_patterns)]
        match self {
            FigureOption::BackGroundColor(c) => fig_elm.set_background_color(c.to_rgba()),
            FigureOption::BorderPCT(pct) => fig_elm.set_border(*pct),
            _ => unimplemented!(),
        }
    }
}
