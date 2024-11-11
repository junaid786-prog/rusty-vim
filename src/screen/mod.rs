mod terminal;

use std::io::{self, Write};
use terminal::{enter_alternate_screen, leave_alternate_screen, clear_screen, move_cursor};
use crate::editor::buffer::Buffer;
use crate::editor::cursor::Cursor;

// Initializes the screen by entering alternate screen and clearing
pub fn init_screen() -> io::Result<()> {
    enter_alternate_screen()?;
    clear_screen()?;
    Ok(())
}

// Restores the screen by clearing and returning to the main screen
pub fn restore_screen() -> io::Result<()> {
    clear_screen()?;
    leave_alternate_screen()?;
    Ok(())
}

// Renders the buffer on the screen and positions the cursor
pub fn render_screen(buffer: &Buffer, cursor: &Cursor) -> io::Result<()> {
    clear_screen()?;

    // Render each line in the buffer
    for (i, line) in buffer.lines.iter().enumerate() {
        println!("{}", line);
    }

    // Position the cursor at the current location
    move_cursor(cursor.x as u16, cursor.y as u16)?;

    io::stdout().flush()?;  // Ensure everything is printed to the screen
    Ok(())
}
