use crossterm::event::{self, read, Event, KeyCode};

pub fn handle_input() -> Option<KeyCode> {
    if let Event::Key(event) = read().unwrap() {
        return Some(event.code)
    }
    None
}