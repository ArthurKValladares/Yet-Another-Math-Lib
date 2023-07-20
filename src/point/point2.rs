use serde::Deserialize;

#[repr(C)]
#[derive(Debug, Deserialize, Default, Copy, Clone, PartialEq)]
#[serde(transparent)]
pub struct Point2D<N: num::Num + Copy> {
    data: [N; 2],
}

impl<N> Point2D<N>
where
    N: num::Num + Copy,
{
    pub fn zero() -> Self {
        Self {
            data: [N::zero(), N::zero()],
        }
    }

    pub fn new(x: N, y: N) -> Self {
        Self { data: [x, y] }
    }

    pub fn x(&self) -> N {
        self.data[0]
    }

    pub fn y(&self) -> N {
        self.data[1]
    }
}

impl<N> std::ops::Sub<Point2D<N>> for Point2D<N>
where
    N: num::Num + Copy,
{
    type Output = Point2D<N>;

    fn sub(self, rhs: Point2D<N>) -> Self::Output {
        Self {
            data: [self.x() - rhs.x(), self.y() - rhs.y()],
        }
    }
}
