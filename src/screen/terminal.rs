use crossterm::{cursor::MoveTo, execute, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}};
use std::io::{self, Write};

// Enters raw mode and the alternate screen
pub fn enter_alternate_screen() -> io::Result<()> {
    enable_raw_mode()?; // Enable raw mode
    execute!(io::stdout(), EnterAlternateScreen)?; // Switch to alternate screen
    Ok(())
}

// Restores the terminal to normal mode
pub fn leave_alternate_screen() -> io::Result<()> {
    disable_raw_mode()?; // Disable raw mode
    execute!(io::stdout(), LeaveAlternateScreen)?; // Return to the main screen
    Ok(())
}

pub fn clear_screen() -> io::Result<()> {
    execute!(io::stdout(), Clear(ClearType::All))?;
    Ok(())
}

pub fn move_cursor(x: u16, y: u16) -> io::Result<()> {
    execute!(io::stdout(), MoveTo(x, y))?;
    Ok(())
}