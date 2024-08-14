pub mod app;

use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    Frame, Terminal,
};
use std::io::{self, stdout, Stdout};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

// initalize terminal and open a "popup" window
pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    Terminal::new(CrosstermBackend::new(stdout()))
}

// destroy terminal window and restore previous view
pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
