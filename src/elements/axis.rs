use super::{Element, ElmOption};

#[allow(dead_code)]
pub struct AxisElm {
    range: Option<(i32, i32)>,
    size: u32,
}

impl Element for AxisElm {}

impl AxisElm {
    pub fn new(options: Vec<AxisOption>) -> Self {
        let mut axis = AxisElm {
            range: None,
            size: 1,
        };
        axis.parse_options(options);
        axis
    }

    fn set_range(&mut self, range: (i32, i32)) {
        self.range = Some(range);
    }

    pub fn get_range(&self) -> Option<(i32, i32)> {
        self.range
    }

    fn set_size(&mut self, size: u32) {
        self.size = size;
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }
}
pub enum AxisOption {
    Range(i32, i32),
    Size(u32),
}

impl ElmOption<AxisElm> for AxisOption {
    fn match_elm_param(&self, axis_elm: &mut AxisElm) {
        #[allow(unreachable_patterns)]
        match self {
            AxisOption::Range(start, end) => axis_elm.set_range((*start, *end)),
            AxisOption::Size(size) => axis_elm.set_size(*size),
            _ => todo!(),
        }
    }
}
