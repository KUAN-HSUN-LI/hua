pub enum LayoutPostion {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    from: (u32, u32),
    to: (u32, u32),
}

impl Vector {
    pub fn new(from: (u32, u32), to: (u32, u32)) -> Self {
        Vector { from, to }
    }
    pub fn direction(&self) -> (i32, i32) {
        (
            (self.from.0 - self.to.0) as i32,
            (self.from.1 - self.to.1) as i32,
        )
    }
    pub fn from(&self) -> (u32, u32) {
        self.from
    }
    pub fn to(&self) -> (u32, u32) {
        self.to
    }
}
