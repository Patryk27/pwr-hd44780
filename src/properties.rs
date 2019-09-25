use crate::{Font, Point};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Properties {
    pub dimensions: Point,
    pub font: Font,
}