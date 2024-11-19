pub mod buffer;
pub mod cursor;
pub mod input;
pub mod mode;

use buffer::Buffer;
use cursor::Cursor;
use mode::MODE;

pub struct Editor {
    buffer: Buffer,
    cursor: Cursor,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            cursor: Cursor::new(),
        }
    }

    pub fn run(&mut self) {
        println!("Editor is running")
    }
}