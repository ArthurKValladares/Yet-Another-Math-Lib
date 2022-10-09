use crate::{point::Point2D, size::Size2D};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Rect2D<N: num::Num + Copy> {
    offset: Point2D<N>,
    size: Size2D<N>,
}

impl<N> Rect2D<N>
where
    N: num::Num + Copy,
{
    pub fn from_offset_and_size(offset: Point2D<N>, size: Size2D<N>) -> Self {
        Self { offset, size }
    }

    pub fn from_size(size: Size2D<N>) -> Self {
        Self::from_offset_and_size(Point2D::zero(), size)
    }

    pub fn from_width_height(width: N, height: N) -> Self {
        Self::from_size(Size2D::new(width, height))
    }

    pub fn width(&self) -> N {
        self.size.width()
    }

    pub fn height(&self) -> N {
        self.size.height()
    }
}
