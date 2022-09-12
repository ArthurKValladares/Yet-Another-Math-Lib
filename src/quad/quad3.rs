use crate::{point::Point3D, size::Size2D};

pub struct Quad3D<N: num::Num + Copy> {
    top: N,
    right: N,
    bottom: N,
    left: N,
    depth: N,
}

impl<N> Quad3D<N>
where
    N: num::Num + Copy,
{
    pub fn from_top_right_bottom_left_depth(
        top: N,
        right: N,
        bottom: N,
        left: N,
        depth: N,
    ) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
            depth,
        }
    }

    pub fn from_top_left(top_left: Point3D<N>, size: Size2D<N>) -> Self {
        Self::from_top_right_bottom_left_depth(
            top_left.y(),
            top_left.x() + size.width(),
            top_left.y() + size.height(),
            top_left.x(),
            top_left.z(),
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

    pub fn depth(&self) -> N {
        self.depth
    }

    pub fn top_left(&self) -> Point3D<N> {
        Point3D::new(self.left, self.top, self.depth)
    }

    pub fn top_right(&self) -> Point3D<N> {
        Point3D::new(self.right, self.top, self.depth)
    }

    pub fn bottom_left(&self) -> Point3D<N> {
        Point3D::new(self.left, self.bottom, self.depth)
    }

    pub fn bottom_right(&self) -> Point3D<N> {
        Point3D::new(self.right, self.bottom, self.depth)
    }
}
