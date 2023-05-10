use crate::{point::Point2D, size::Size2D};
use serde::Deserialize;

#[repr(C)]
#[derive(Debug, Deserialize, Default, Copy, Clone, PartialEq)]
pub struct Rect2D<N: num::Num + Copy, M: num::Num + Copy> {
    offset: Point2D<N>,
    size: Size2D<M>,
}

impl<N, M> Rect2D<N, M>
where
    N: num::Num + Copy,
    M: num::Num + Copy,
{
    pub fn from_offset_and_size(offset: Point2D<N>, size: Size2D<M>) -> Self {
        Self { offset, size }
    }

    pub fn from_size(size: Size2D<M>) -> Self {
        Self::from_offset_and_size(Point2D::zero(), size)
    }

    pub fn from_width_height(width: M, height: M) -> Self {
        Self::from_size(Size2D::new(width, height))
    }

    pub fn offset(&self) -> Point2D<N> {
        self.offset
    }

    pub fn size(&self) -> Size2D<M> {
        self.size
    }

    pub fn width(&self) -> M {
        self.size.width()
    }

    pub fn height(&self) -> M {
        self.size.height()
    }
}

impl<N, M> From<Size2D<M>> for Rect2D<N, M>
where
    N: num::Num + Copy,
    M: num::Num + Copy,
{
    fn from(size: Size2D<M>) -> Self {
        Self::from_offset_and_size(Point2D::zero(), size)
    }
}
