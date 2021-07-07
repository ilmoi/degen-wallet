use crate::eth::wallet::generate_and_save_mnemonic;
use crate::tui::util::event::{Event, Events};
use crate::tui::util::{StatefulList, TabsState};
use bip39::Mnemonic;
use std::collections::HashMap;
use std::io::Stdout;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use std::{error::Error, io, thread};
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

// ----------------------------------------------------------------------------- app state

struct AppState {
    screen: Screen,
    mnemonic: Option<Mnemonic>,
    file_uuid: Option<String>,
}

impl AppState {
    fn fresh_state() -> Self {
        Self {
            screen: Screen::Welcome,
            mnemonic: None,
            file_uuid: None,
        }
    }
}

// ----------------------------------------------------------------------------- screens

#[derive(Hash, Eq, PartialEq, Clone)]
enum Screen {
    Welcome,
    NewWallet,
}

impl Screen {
    fn init_screens() -> HashMap<Screen, Box<dyn Drawable>> {
        let mut h: HashMap<Screen, Box<dyn Drawable>> = HashMap::new();
        h.insert(Screen::Welcome, Box::new(Welcome::new()));
        h.insert(Screen::NewWallet, Box::new(NewWallet::new()));
        h
    }
}

// ----------------------------------------------------------------------------- drawable

trait Drawable {
    fn draw_body(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        state: Arc<Mutex<AppState>>,
    );
    fn set_keybinding(&mut self, key: Key, state: Arc<Mutex<AppState>>);
}

// ----------------------------------------------------------------------------- 1: welcome

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
}

impl<'a> Drawable for Welcome<'a> {
    fn draw_body(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        _state: Arc<Mutex<AppState>>,
    ) {
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
    fn set_keybinding(&mut self, key: Key, state: Arc<Mutex<AppState>>) {
        match key {
            Key::Char('\n') => {
                // println!("{}", self.list_app.items.state.selected().unwrap());
                let mut lock = state.lock().unwrap();
                lock.deref_mut().screen = Screen::NewWallet;
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

// ----------------------------------------------------------------------------- 2: new wallet

struct NewWallet {}

impl NewWallet {
    fn new() -> Self {
        Self {}
    }
}

impl Drawable for NewWallet {
    fn draw_body(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        state: Arc<Mutex<AppState>>,
    ) {
        let thread_state = state.clone();
        let mut lock = state.lock().unwrap();
        let state = lock.deref();

        let text = vec![
            Spans::from("Generating a new wallet..."),
            Spans::from(vec![
                Span::raw("Your mnemonic is:"),
                Span::styled(
                    format!("{:?}", state.mnemonic.as_ref()),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
            Spans::from("Write it down and hide it in a good place."),
            Spans::from(vec![
                Span::raw("We also generated a Keystore file for you and saved it under "),
                Span::styled(
                    format!("/keys/{:?}", state.file_uuid.as_ref()),
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

        if state.mnemonic.is_none() {
            thread::spawn(move || {
                let (mnemonic, file_uuid) = generate_and_save_mnemonic();

                let mut lock = thread_state.lock().unwrap();
                let state = lock.deref_mut();

                state.mnemonic = Some(mnemonic);
                state.file_uuid = Some(file_uuid);
            });
        }

        let p = Paragraph::new(text).block(body_block);

        f.render_widget(p, body_chunk);
    }
    fn set_keybinding(&mut self, key: Key, state: Arc<Mutex<AppState>>) {
        match key {
            Key::Char('\n') => {
                let mut lock = state.lock().unwrap();
                lock.deref_mut().screen = Screen::Welcome;
            }
            _ => {}
        }
    }
}

// ----------------------------------------------------------------------------- main fn

pub fn draw_screen() -> Result<(), Box<dyn Error>> {
    let mut terminal = init_terminal().unwrap();
    let events = Events::new();

    // app state
    let mut state = AppState::fresh_state();
    let arc_state = Arc::new(Mutex::new(state));
    let mut current_screen: &mut Box<dyn Drawable>;

    // pre-initialized screens
    // NOTE 1: trade-off: we don't have to re-init on every loop turn, but we might init screens that we never visit
    //         given the ratio of # of screens to # of loop turns this makes sense
    // NOTE 2: need mut because some screens hold their own state (eg lists)
    let mut screens = Screen::init_screens();

    loop {
        let local_state = arc_state.clone();
        let lock = local_state.lock().unwrap();
        current_screen = screens.get_mut(&lock.deref().screen).unwrap();

        terminal.draw(|f| {
            let body_chunk = draw_standard_grid(f);
            let body_block = Block::default().borders(Borders::ALL);
            current_screen.draw_body(body_chunk, body_block, f, arc_state.clone());
        });

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                _ => current_screen.set_keybinding(input, arc_state.clone()),
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
