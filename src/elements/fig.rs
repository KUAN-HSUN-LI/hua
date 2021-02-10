use super::{Element, Option};
use crate::color::RGBColor;

#[allow(dead_code)]
pub struct FigElm {
    background_color: RGBColor,
    border: f32,
}

impl Element for FigElm {
    #[allow(unused_variables)]
    fn parse_options(&mut self, options: Vec<impl Option<Self>>) -> &Self
    where
        Self: Sized,
    {
        for o in options {
            o.match_elm_param(self);
        }

        return self;
    }
}

impl FigElm {
    pub fn new(options: Vec<FigureOption>) -> Self {
        let mut fig = FigElm {
            background_color: RGBColor(0, 0, 0),
            border: 0.0,
        };
        fig.parse_options(options);
        fig
    }

    pub fn set_background_color(&mut self, color: RGBColor) {
        self.background_color = color;
    }

    pub fn set_border(&mut self, border: f32) {
        self.border = border;
    }
}

// #[derive(Clone, Copy)]
pub enum FigureOption {
    BackGroundColor(RGBColor),
    Border(f32),
}

// impl FigureOption {
//     fn _match_elm_param(&self, fig_elm: &mut FigElm) {}
// }

impl Option<FigElm> for FigureOption {
    fn match_elm_param(&self, fig_elm: &mut FigElm) {
        match self {
            FigureOption::BackGroundColor(c) => fig_elm.set_background_color(*c),
            FigureOption::Border(pct) => fig_elm.set_border(*pct),
        }
    }
}
