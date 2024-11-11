pub mod screen;
pub mod editor;

use crate::screen::{init_screen, restore_screen, render_screen};
use crate::editor::buffer::Buffer;
use crate::editor::cursor::Cursor;

fn main() {
    let mut buffer = Buffer::new();
    let mut cursor = Cursor::new();

    // Initialize the terminal screen
    init_screen().expect("Failed to initialize screen");

    // Main editor loop
    loop {
        // Render the buffer and cursor
        render_screen(&buffer, &cursor).expect("Failed to render screen");

        // Handle input, update buffer/cursor (this would be part of your editor logic)

        // Break condition (e.g., user quits)
        // For example purposes, we break immediately
        // break;
    }

    // Restore the terminal screen
    restore_screen().expect("Failed to restore screen");
}
