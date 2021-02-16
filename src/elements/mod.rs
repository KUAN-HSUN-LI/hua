use axis::AxisElm;
use fig::FigElm;

pub mod axis;
pub mod fig;

pub enum ElmType {
    Axis(AxisElm),
    Fig(FigElm),
}

pub trait Element {
    fn parse_options(&mut self, options: Vec<impl ElmOption<Self>>) -> &Self
    where
        Self: Sized,
    {
        options.iter().for_each(|o| o.match_elm_param(self));
        return self;
    }
}

pub trait ElmOption<T> {
    fn match_elm_param(&self, elm: &mut T);
}
