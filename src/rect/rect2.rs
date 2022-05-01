use crate::{point::Point2D, size::Size2D};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Rect2D {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

impl Rect2D {
    pub fn from_top_right_bottom_left(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn from_top_left(top_left: Point2D, size: Size2D) -> Self {
        Self::from_top_right_bottom_left(
            top_left.y(),
            top_left.x() + size.width,
            top_left.y() + size.height,
            top_left.x(),
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

    pub fn top_left(&self) -> Point2D {
        Point2D::new(self.left, self.top)
    }

    pub fn top_right(&self) -> Point2D {
        Point2D::new(self.right, self.top)
    }

    pub fn bottom_left(&self) -> Point2D {
        Point2D::new(self.left, self.bottom)
    }

    pub fn bottom_right(&self) -> Point2D {
        Point2D::new(self.right, self.bottom)
    }
}
