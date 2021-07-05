use std::{error::Error, io};

use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, layout::{Alignment, Constraint, Direction, Layout}, style::{Color, Modifier, Style}, Terminal, text::Span, widgets::{Block, Borders, BorderType}, text};

use crate::util::event::{Event, Events};

pub fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup event handlers
    let events = Events::new();

    loop {
        terminal.draw(|f| {
            // Wrapping block for a group
            // Just draw the block and the group on the same area and build the group
            // with at least a margin of 1
            let mut size = f.size();

            let s = Style::default().fg(Color::Blue).add_modifier(Modifier::UNDERLINED);

            // Surrounding block
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .title(text::Span::styled("Main block with round corners", s))
                .border_type(BorderType::Double);

            f.render_widget(block, size);

            // splits into exactly 2 chunks on vertical axis (so top and bottom)
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(4)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            // Top two inner blocks
            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[0]);

            // Top left inner block with green background
            let block = Block::default()
                .title(vec![
                    Span::styled("word1", Style::default().fg(Color::Red)),
                    Span::from(" word2"),
                    Span::styled("word3", Style::default().bg(Color::Blue)),
                ])
                .style(Style::default().bg(Color::Yellow));
            f.render_widget(block, top_chunks[0]);

            // Top right inner block with styled title aligned to the right
            let block = Block::default()
                .title(Span::styled(
                    "Styled title",
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::Red)
                        .add_modifier(Modifier::BOLD),
                ));
            f.render_widget(block, top_chunks[1]);

            // Bottom two inner blocks
            let bottom_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(chunks[1]);

            // Bottom left block with all default borders
            let block = Block::default().title("With borders").borders(Borders::ALL);
            f.render_widget(block, bottom_chunks[0]);

            // Bottom right block with styled left and right border
            let block = Block::default()
                .title("With styled borders and doubled borders")
                .border_style(Style::default().fg(Color::Cyan))
                .borders(Borders::LEFT | Borders::RIGHT)
                .border_type(BorderType::Double);
            f.render_widget(block, bottom_chunks[1]);
        })?;

        if let Event::Input(key) = events.next()? {
            if key == Key::Char('q') {
                break;
            }
        }
    }
    Ok(())
}