pub mod fig;

pub trait Element {
    fn parse_options(&mut self, options: Vec<impl Option<Self>>) -> &Self
    where
        Self: Sized;
}

pub trait Option<T> {
    fn match_elm_param(&self, elm: &mut T);
}
