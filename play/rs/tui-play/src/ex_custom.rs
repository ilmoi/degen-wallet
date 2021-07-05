use crate::util::event::{Event, Events};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::style::Color;
use tui::text::Span;
use tui::{
    backend::TermionBackend, buffer::Buffer, layout::Rect, style::Style, widgets::Widget, Terminal,
};

struct Label<'a> {
    text: &'a str,
}

struct Square {
    side: u16,
    color: Color,
}

impl<'a> Default for Label<'a> {
    fn default() -> Label<'a> {
        Label { text: "" }
    }
}

impl Default for Square {
    fn default() -> Square {
        Square {
            side: 10,
            color: Color::Cyan,
        }
    }
}

impl<'a> Widget for Label<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(
            area.right() - 20,
            area.bottom() - 20,
            self.text,
            Style::default(),
        );
    }
}

impl Widget for Square {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // ok so I think area = the rectangle of the screen provided to me in case I need it
        // but I don't have to use it

        for i in 0..self.side {
            for j in 0..self.side {
                buf.set_string(i, j, "x", Style::default().bg(Color::Cyan))
            }
        }
    }
}

impl<'a> Label<'a> {
    fn text(mut self, text: &'a str) -> Label<'a> {
        self.text = text;
        self
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let label = Label::default().text("Test");
            f.render_widget(label, size);

            let s = Square::default();
            f.render_widget(s, size);
        })?;

        if let Event::Input(key) = events.next()? {
            if key == Key::Char('q') {
                break;
            }
        }
    }

    Ok(())
}
