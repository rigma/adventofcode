#[derive(Clone, Debug)]
pub struct Point(i32, i32);

impl Point {
    #[inline]
    pub const fn x(&self) -> i32 {
        self.0
    }

    #[inline]
    pub const fn y(&self) -> i32 {
        self.1
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Self(value.0, value.1)
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self(
            value.0.try_into().expect("integer too large"),
            value.1.try_into().expect("integer too large"),
        )
    }
}
