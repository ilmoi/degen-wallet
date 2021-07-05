use std::collections::HashMap;
use std::io::Stdout;
use std::{error::Error, io, thread};

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

use crate::eth::{generate_and_save_mnemonic, EthAddr};
use crate::tui::helpers::{draw_standard_grid, init_terminal, TermBck};
use crate::tui::screens::new_wallet::NewWallet;
use crate::tui::screens::welcome::Welcome;
use crate::tui::util::event::{Event, Events};
use crate::tui::util::{StatefulList, TabsState};

// ----------------------------------------------------------------------------- app state

pub struct AppState {
    pub screen: Screen,
    pub mnemonic: Option<Mnemonic>,
    pub file_uuid: Option<String>,
    pub eth_accounts: Vec<EthAddr>,
}

impl AppState {
    pub fn fresh_state() -> Self {
        Self {
            screen: Screen::Welcome,
            mnemonic: None,
            file_uuid: None,
            eth_accounts: vec![],
        }
    }
}

// ----------------------------------------------------------------------------- screens

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum Screen {
    Welcome,
    NewWallet,
}

impl Screen {
    /// Hashmap of trait objects instead of generics.
    /// Generics wouldn't let us have heterogenous objects in the same hashmap.
    ///
    /// Another way to think about it: imagine Scren is an external library we wrote that is being called from this app.
    /// Screen has no idea what the user will want to draw - and it doesn't care.
    /// As long as items going into the hashmap implement Drawable they're good.
    ///
    /// The downside of this approach is the added cost during runtime due to dynamic dispatch.
    /// Arguably in this particular case we as the author of the tui app know all the screens,
    /// and we could use as simple match statement with static dispatch.
    ///
    /// https://doc.rust-lang.org/stable/book/ch17-02-trait-objects.html
    pub fn init_screens() -> HashMap<Screen, Box<dyn Drawable>> {
        let mut h: HashMap<Screen, Box<dyn Drawable>> = HashMap::new();
        h.insert(Screen::Welcome, Box::new(Welcome::new()));
        h.insert(Screen::NewWallet, Box::new(NewWallet::new()));
        h
    }
}

// ----------------------------------------------------------------------------- drawable

pub trait Drawable {
    fn draw_body(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        state: &mut AppState,
    );
    fn set_keybinding(&mut self, key: Key, state: &mut AppState);
}
