use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{queue, Command};
use std::fmt::Display;
use std::io::Error;

use std::io::{stdout, Write};

pub struct Terminal;

#[derive(Clone, Copy)]
pub struct Size {
    pub columns: usize,
    pub rows: usize,
}

#[derive(Clone, Copy, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl Terminal {
    pub fn move_caret_to(pos: Position) -> Result<(), Error> {
        Self::queue_cmd(MoveTo(pos.col as u16, pos.row as u16))
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_cmd(Clear(ClearType::All))
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_cmd(Clear(ClearType::CurrentLine))
    }

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
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
        Ok(Size {
            columns: usize::from(columns),
            rows: usize::from(rows),
        })
    }

    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_cmd(Hide)
    }

    pub fn show_caret() -> Result<(), Error> {
        Self::queue_cmd(Show)
    }

    pub fn print<T: Display>(str: T) -> Result<(), Error> {
        Self::queue_cmd(Print(str))
    }

    pub(crate) fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    fn queue_cmd<T: Command>(cmd: T) -> Result<(), Error> {
        queue!(stdout(), cmd)?;
        Ok(())
    }
}
