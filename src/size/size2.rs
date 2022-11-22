#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Size2D<N: num::Num + Copy> {
    width: N,
    height: N,
}

impl<N> Size2D<N>
where
    N: num::Num + Copy,
{
    pub fn new(width: N, height: N) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> N {
        self.width
    }

    pub fn height(&self) -> N {
        self.height
    }

    pub fn as_f32(&self) -> Size2D<f32> {
        Size2D::<f32>::new(self.width().as_f32(), self.height().as_f32())
    }
}
