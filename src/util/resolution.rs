#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Resolution {
    pub fn new(width: u32, height: u32) -> Resolution {
        Resolution {width, height}
    }
}