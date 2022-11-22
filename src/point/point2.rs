#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
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

    pub fn as_f32(&self) -> Point2D<f32> {
        Point2D::<f32>::new(self.x() as f32, self.y() as f32)
    }
}
