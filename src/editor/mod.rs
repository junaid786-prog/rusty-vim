pub mod buffer;
pub mod cursor;
pub mod input;

use buffer::Buffer;
use cursor::Cursor;

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