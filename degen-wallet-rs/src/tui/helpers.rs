use std::io::Stdout;
use std::{error::Error, io};

use termion::raw::RawTerminal;
use termion::{raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

pub type TermBck = TermionBackend<AlternateScreen<RawTerminal<Stdout>>>;

pub fn init_terminal() -> Result<Terminal<TermBck>, Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    // let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn draw_standard_grid(f: &mut Frame<TermBck>) -> Rect {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        // .margin(20)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(size);

    // ----------------------------------------------------------------------------- header
    let header = Block::default().borders(Borders::ALL);
    let title = Paragraph::new("degen 🍌 wallet")
        .block(header)
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    // ----------------------------------------------------------------------------- footer
    let footer = Block::default().borders(Borders::ALL);
    let text = Spans::from(vec![
        Span::styled("<q>", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" quit"),
        Span::raw("     "),
        Span::styled("<Esc>", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" go back"),
        Span::raw("     "),
        Span::styled("<Enter>", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" select"),
        Span::raw("     "),
        Span::styled("← ↑ → ↓", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" move around"),
    ]);
    let tips = Paragraph::new(text)
        .block(footer)
        .alignment(Alignment::Center);
    f.render_widget(tips, chunks[2]);

    let body_chunk = chunks[1];
    body_chunk
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn centered_rect_fixed(fixed_x: u16, fixed_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - fixed_y) / 2),
                Constraint::Percentage(fixed_y),
                Constraint::Percentage((100 - fixed_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - fixed_x) / 2),
                Constraint::Percentage(fixed_x),
                Constraint::Percentage((100 - fixed_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
