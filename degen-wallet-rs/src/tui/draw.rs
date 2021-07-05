use std::collections::HashMap;
use std::io::Stdout;
use std::{error::Error, io, thread};

use crate::tui::helpers::{draw_standard_grid, init_terminal};
use crate::tui::state::{AppState, Drawable, Screen};
use crate::tui::util::event::{Event, Events};
use bip39::Mnemonic;
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

pub fn draw_screen() -> Result<(), Box<dyn Error>> {
    let mut terminal = init_terminal().unwrap();
    let events = Events::new();

    // app state
    let mut state = AppState::fresh_state();
    let mut current_screen: &mut Box<dyn Drawable>;

    // pre-initialized screens
    // NOTE 1: trade-off: we don't have to re-init on every loop turn, but we might init screens that we never visit
    //         given the ratio of # of screens to # of loop turns this makes sense
    // NOTE 2: need mut because some screens hold their own state (eg lists)
    let mut screens = Screen::init_screens();

    loop {
        current_screen = screens.get_mut(&state.screen).unwrap();

        terminal.draw(|f| {
            let body_chunk = draw_standard_grid(f);
            let body_block = Block::default().borders(Borders::ALL);
            current_screen.draw_body(body_chunk, body_block, f, &mut state);
        });

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                _ => current_screen.set_keybinding(input, &mut state),
            },
            _ => {}
        }
    }
    Ok(())
}
