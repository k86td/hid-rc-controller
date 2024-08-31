pub mod minimal;
pub mod utils;
pub mod widgets;

use std::io::{self, Stdout};

use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

// initalize terminal and open a "popup" window
pub fn init(stdout: &mut Stdout) -> io::Result<()> {
    execute!(stdout, EnterAlternateScreen)?;
    execute!(stdout, cursor::Hide)?;
    enable_raw_mode()?;
    Ok(())
}

// destroy terminal window and restore previous view
pub fn restore(stdout: &mut Stdout) -> io::Result<()> {
    execute!(stdout, LeaveAlternateScreen)?;
    execute!(stdout, cursor::Show)?;
    disable_raw_mode()?;
    Ok(())
}
