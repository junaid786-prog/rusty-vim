pub mod screen;
pub mod editor;

use crossterm::event::KeyCode;
use editor::input::handle_input;
use editor::Editor;

use crate::screen::{init_screen, restore_screen, render_screen};
use crate::editor::buffer::Buffer;
use crate::editor::cursor::Cursor;
use crate::editor::mode::MODE;

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
        if let Some(input) = handle_input() {
            println!("{}", input);
            
            // Process input
            //process_input(input, &mut buffer, &mut cursor, &mut mode);

            // Exit condition
            // if input == KeyCode::Char('q') && mode == MODE::Normal {
                // break;
            // }
        }
        // Break condition (e.g., user quits)
        // For example purposes, we break immediately
        break;
    }

    // Restore the terminal screen
    restore_screen().expect("Failed to restore screen");
}
