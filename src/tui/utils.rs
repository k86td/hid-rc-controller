use std::io::{Stdout, Write};

use crossterm::{cursor, queue, terminal};

pub fn print_middle_row(text: &str, row_index: u16, stdout: &mut Stdout) {
    let middle_index = (terminal::size().unwrap().0 / 2) - (text.len() as u16 / 2);

    queue!(
        stdout,
        cursor::MoveToRow(row_index - 1),
        cursor::MoveToColumn(middle_index),
    )
    .unwrap();

    writeln!(stdout, "{}", text).unwrap();
}
