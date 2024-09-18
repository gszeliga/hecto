use std::io::Error;

use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};

mod terminal;
use terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => {}
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye!. \r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position { col: 0, row: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size { columns: _, rows } = Terminal::size()?;

        for row in 0..rows {
            Terminal::clear_line()?;
            if row == rows / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            Terminal::move_cursor_to(Position { col: 0, row })?;
        }
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut msg = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.columns as usize;
        let padding = (width - msg.len()) / 2;
        let margin = " ".repeat(padding - 1);

        msg = format!("~{margin}{msg}");
        msg.truncate(width);
        Terminal::print(&msg)?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~\n")?;
        Ok(())
    }
}
