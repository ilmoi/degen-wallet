use crate::eth::wallet::generate_and_save_mnemonic;
use crate::tui::util::event::{Event, Events};
use crate::tui::util::{StatefulList, TabsState};
use bip39::Mnemonic;
use std::io::Stdout;
use std::{error::Error, io};
use termion::raw::RawTerminal;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::widgets::{List, ListItem, Tabs};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame, Terminal,
};

enum ScreenState {
    Welcome,
    NewWallet,
}

struct Welcome<'a> {
    list_app: ListApp<'a>,
}

impl<'a> Welcome<'a> {
    fn new() -> Self {
        Self {
            list_app: ListApp::new(vec![
                "create new",
                "import existing",
                "login with passphrase",
            ]),
        }
    }
    fn draw_body(&mut self, body_chunk: Rect, body_block: Block, f: &mut Frame<TermBck>) {
        let items: Vec<ListItem> = self
            .list_app
            .items
            .items
            .iter()
            .map(|i| ListItem::new(Span::from(i.to_owned())))
            .collect();

        let items = List::new(items)
            .block(body_block)
            .highlight_style(Style::default().bg(Color::Blue))
            .highlight_symbol(">> ");

        f.render_stateful_widget(items, body_chunk, &mut self.list_app.items.state);
    }
    fn set_keybinding(&mut self, key: Key, state: &mut ScreenState) {
        match key {
            Key::Char('\n') => {
                // println!("{}", self.list_app.items.state.selected().unwrap());
                *state = ScreenState::NewWallet;
            }
            Key::Left => {
                self.list_app.items.unselect();
            }
            Key::Down => {
                self.list_app.items.next();
            }
            Key::Up => {
                self.list_app.items.previous();
            }
            _ => {}
        }
    }
}

struct NewWallet {
    mnemonic: Mnemonic,
    file_uuid: String,
}

impl NewWallet {
    fn new() -> Self {
        let (mnemonic, file_uuid) = generate_and_save_mnemonic();
        Self {
            mnemonic,
            file_uuid,
        }
    }
    fn draw_body(&mut self, body_chunk: Rect, body_block: Block, f: &mut Frame<TermBck>) {
        //todo ask for pw

        let text = vec![
            Spans::from("Generating a new wallet..."),
            Spans::from(vec![
                Span::raw("Your mnemonic is:"),
                Span::styled(
                    format!("{}", self.mnemonic),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
            Spans::from("Write it down and hide it in a good place."),
            Spans::from(vec![
                Span::raw("We also generated a Keystore file for you and saved it under "),
                Span::styled(
                    format!("/keys/{}", self.file_uuid),
                    Style::default().fg(Color::Cyan),
                ),
                Span::raw("."),
            ]),
            Spans::from("It's encrypted with your password so you can share it - but best not to."),
            Spans::from(vec![
                Span::raw("Hit "),
                Span::styled("<Enter>", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" when ready to proceed."),
            ]),
        ];

        let p = Paragraph::new(text).block(body_block);

        f.render_widget(p, body_chunk);
    }
    fn set_keybinding(&mut self, key: Key, state: &mut ScreenState) {
        match key {
            Key::Char('\n') => {
                // println!("ready to proceed!");
                *state = ScreenState::Welcome;
            }
            _ => {}
        }
    }
}

pub fn draw_screen() -> Result<(), Box<dyn Error>> {
    let mut terminal = init_terminal().unwrap();
    let events = Events::new();

    let mut state = ScreenState::Welcome;
    let mut welcome_screen = Welcome::new();
    let mut wallet_screen = NewWallet::new();

    loop {
        terminal.draw(|f| {
            let body_chunk = draw_standard_grid(f);
            let body_block = Block::default().borders(Borders::ALL);

            match state {
                ScreenState::Welcome => welcome_screen.draw_body(body_chunk, body_block, f),
                ScreenState::NewWallet => wallet_screen.draw_body(body_chunk, body_block, f),
            }
        });

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                _ => match state {
                    ScreenState::Welcome => welcome_screen.set_keybinding(input, &mut state),
                    ScreenState::NewWallet => wallet_screen.set_keybinding(input, &mut state),
                },
            },
            _ => {}
        }
    }
    Ok(())
}

// ----------------------------------------------------------------------------- helpers

type TermBck = TermionBackend<AlternateScreen<RawTerminal<Stdout>>>;

fn init_terminal() -> Result<Terminal<TermBck>, Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    // let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn draw_standard_grid(f: &mut Frame<TermBck>) -> Rect {
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

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

fn centered_rect_fixed(fixed_x: u16, fixed_y: u16, r: Rect) -> Rect {
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

struct TabsApp<'a> {
    tabs: TabsState<'a>,
}

struct ListApp<'a> {
    items: StatefulList<&'a str>,
}

impl<'a> ListApp<'a> {
    fn new(items: Vec<&'a str>) -> ListApp<'a> {
        Self {
            items: StatefulList::with_items(items),
        }
    }
}
