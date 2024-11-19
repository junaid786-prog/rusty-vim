use crossterm::event::{self, read, Event, KeyCode};
use crate::editor::Buffer;
use crate::editor::Cursor;
use crate::editor::MODE;

pub fn handle_input() -> Option<KeyCode> {
    if let Event::Key(event) = read().unwrap() {
        println!("Key");
        return Some(event.code)
    }
    None
}

pub fn process_input(input: KeyCode, buffer: &mut Buffer, cursor: &mut Cursor, mode: &mut MODE) {
    match mode {
        MODE::Normal => match input {
            KeyCode::Char('i') => *mode = MODE::Insert, // Switch to Insert mode
            KeyCode::Esc => *mode = MODE::Normal,       // Stay in Normal mode
            // KeyCode::Left => cursor.move_left(),
            // KeyCode::Right => cursor.move_right(buffer),
            KeyCode::Up => cursor.move_up(),
            KeyCode::Down => cursor.move_down(),
            _ => {}
        },
        MODE::Insert => match input {
            KeyCode::Esc => *mode = MODE::Normal, // Switch back to Normal mode
            KeyCode::Char(c) => buffer.insert_char(c, cursor.y, cursor.x),
            // KeyCode::Backspace => buffer.delete_char(cursor.y, cursor.x),
            _ => {}
        },
        MODE::Visual => todo!(),
    }
}