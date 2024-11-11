Here’s a concise `TODO` list with main functionality names:

---

# Simple Text Editor - TODO

## Project Setup

- [ ] **Initialize Project**: Set up Rust project and dependencies

## Core Functionalities

- [ ] **Editor Setup**: Define `Editor` struct and main loop

### Buffer Management

- [ ] **Text Storage**: Implement buffer to store lines
- [ ] **Text Manipulation**: Add `insert` and `delete` functionality

### Cursor Management

- [ ] **Position Tracking**: Set up `Cursor` struct with `x` and `y` positions
- [ ] **Navigation**: Implement movement (up, down, left, right)

### Input Handling

- [ ] **Input Capture**: Process keyboard input with `crossterm`
- [ ] **Mode-Specific Commands**: Handle Normal and Insert modes

### Mode Management

- [ ] **Mode Switching**: Implement Normal and Insert modes

### Screen Rendering

- [ ] **Terminal Control**: Set up raw mode and cursor placement
- [ ] **Screen Refresh**: Clear and render updated content

### File I/O

- [ ] **Open and Save**: Implement file read and write functions

## Testing

- [ ] **Unit Tests**: Test buffer, cursor, and file operations