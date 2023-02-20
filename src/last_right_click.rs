#[derive(Debug, Clone, Copy)]
pub struct LastRightClick {
    pub x: i32,
    pub y: i32,
}

impl LastRightClick {
    pub fn new() -> Self {
        LastRightClick { x: 0, y: 0 }
    }

    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
    }
}
