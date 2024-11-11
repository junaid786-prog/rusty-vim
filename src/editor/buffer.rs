use std::fmt::Error;

pub struct Buffer {
    pub lines: Vec<String>
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()]
        }
    }
    pub fn insert_char(&mut self, c: char, line: usize, col: usize) {
        while (line >= self.lines.len()){
            self.lines.push(String::new());
        }
        
        let current_line = &mut self.lines[line];

        if (col < current_line.len()) {
            current_line.insert(col, c);
        } else {
           current_line.push(c);
        }
    }

    pub fn remove_char(&mut self, line: usize, col: usize){
        if (line < self.lines.len()){
            let current_line = &mut self.lines[line];

            if (col < current_line.len()) {
                let removed_char = current_line.remove(col);
                print!("here is {}", removed_char);

            }
        }

    }
}