use std::io;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use tui::widgets::{Block, Borders};
use tui::layout::{Direction, Constraint, Layout};
use termion::event::Key;
use crate::util::event::{Event, Events};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup event handlers
    let events = Events::new();

    loop {

        terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10)
                ].as_ref()
            )
            .split(f.size());
        let block = Block::default()
             .title("Block")
             .borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        let block = Block::default()
             .title("Block 2")
             .borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    })?;

        //detect a keystroke and exit
        if let Event::Input(key) = events.next().unwrap() {
            if key == Key::Char('q') {
                break;
            }
        }
    }

    Ok(())
}