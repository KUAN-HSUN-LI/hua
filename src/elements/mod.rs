pub mod fig;

pub trait Element {
    fn parse_options<T: Option>(&self, options: Vec<T>) -> Self
    where
        Self: Sized;
}

pub trait Option {}
