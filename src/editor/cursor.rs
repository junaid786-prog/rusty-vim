pub struct Cursor {
    pub x: usize,
    pub y: usize
}

const MAX_X: usize = 1000;

impl Cursor {
    pub fn new() -> Self {
        Self {
            x : 0,
            y : 0,
        }
    }

    pub fn move_up(&mut self) {
        if (self.x > 0){
            self.x -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if (self.x < MAX_X) {
            self.x += 1;
        }
    }
}