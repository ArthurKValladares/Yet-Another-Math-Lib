use crate::{point::Point2D, size::Size2D};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Rect2D<N: num::Num + Copy> {
    top: N,
    right: N,
    bottom: N,
    left: N,
}

impl<N> Rect2D<N>
where
    N: num::Num + Copy,
{
    pub fn from_top_right_bottom_left(top: N, right: N, bottom: N, left: N) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn from_top_left(top_left: Point2D<N>, size: Size2D<N>) -> Self {
        Self::from_top_right_bottom_left(
            top_left.y(),
            top_left.x() + size.width(),
            top_left.y() + size.height(),
            top_left.x(),
        )
    }

    pub fn top(&self) -> N {
        self.top
    }

    pub fn right(&self) -> N {
        self.right
    }

    pub fn bottom(&self) -> N {
        self.bottom
    }

    pub fn left(&self) -> N {
        self.left
    }

    pub fn top_left(&self) -> Point2D<N> {
        Point2D::new(self.left, self.top)
    }

    pub fn top_right(&self) -> Point2D<N> {
        Point2D::new(self.right, self.top)
    }

    pub fn bottom_left(&self) -> Point2D<N> {
        Point2D::new(self.left, self.bottom)
    }

    pub fn bottom_right(&self) -> Point2D<N> {
        Point2D::new(self.right, self.bottom)
    }
}
