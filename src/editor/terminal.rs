use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{execute, queue};
use std::io::Error;

use std::io::{stdout, Write};

pub struct Terminal;

#[derive(Clone, Copy)]
pub struct Size {
    pub columns: u16,
    pub rows: u16,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub col: u16,
    pub row: u16,
}

impl Terminal {
    pub fn move_cursor_to(pos: Position) -> Result<(), Error> {
        execute!(stdout(), MoveTo(pos.col, pos.row))?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))
    }

    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))
    }

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { col: 0, row: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (columns, rows) = size()?;
        Ok(Size { columns, rows })
    }

    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }

    pub fn print(str: &str) -> Result<(), Error> {
        queue!(stdout(), Print(str))?;
        Ok(())
    }

    pub(crate) fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
}
