#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Size3D<N: num::Num + Copy> {
    width: N,
    height: N,
    depth: N,
}

impl<N> Size3D<N>
where
    N: num::Num + Copy,
{
    pub fn new(width: N, height: N, depth: N) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }

    pub fn width(&self) -> N {
        self.width
    }

    pub fn height(&self) -> N {
        self.height
    }

    pub fn depth(&self) -> N {
        self.depth
    }
}
