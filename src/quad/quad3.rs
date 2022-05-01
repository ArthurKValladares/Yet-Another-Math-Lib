use crate::{point::Point3D, size::Size2D};

pub struct Quad3D {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
    depth: f32,
}

impl Quad3D {
    pub fn from_top_right_bottom_left_depth(
        top: f32,
        right: f32,
        bottom: f32,
        left: f32,
        depth: f32,
    ) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
            depth,
        }
    }

    pub fn from_top_left(top_left: Point3D, size: Size2D) -> Self {
        Self::from_top_right_bottom_left_depth(
            top_left.y(),
            top_left.x() + size.width,
            top_left.y() + size.height,
            top_left.x(),
            top_left.z(),
        )
    }

    pub fn top(&self) -> f32 {
        self.top
    }

    pub fn right(&self) -> f32 {
        self.right
    }

    pub fn bottom(&self) -> f32 {
        self.bottom
    }

    pub fn left(&self) -> f32 {
        self.left
    }

    pub fn depth(&self) -> f32 {
        self.depth
    }

    pub fn top_left(&self) -> Point3D {
        Point3D::new(self.left, self.top, self.depth)
    }

    pub fn top_right(&self) -> Point3D {
        Point3D::new(self.right, self.top, self.depth)
    }

    pub fn bottom_left(&self) -> Point3D {
        Point3D::new(self.left, self.bottom, self.depth)
    }

    pub fn bottom_right(&self) -> Point3D {
        Point3D::new(self.right, self.bottom, self.depth)
    }
}
