use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};

use std::io::stdout;

pub struct Terminal {}

impl Terminal {
    pub fn move_cursor_to(col: u16, row: u16) -> Result<(), std::io::Error> {
        execute!(stdout(), MoveTo(col, row))?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout: std::io::Stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    pub fn size() -> Result<(u16, u16), std::io::Error> {
        size()
    }
}
