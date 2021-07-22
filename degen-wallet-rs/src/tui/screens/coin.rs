use termion::event::Key;
use tui::{
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, List, ListItem},
    Frame,
};

use crate::tui::{
    helpers::TermBck,
    state::{AppState, Drawable, Screen, SelectedCoin},
    util::ListApp,
};

pub struct Coin<'a> {
    pub list_app: ListApp<'a>,
}

impl<'a> Coin<'a> {
    pub fn new() -> Self {
        Self {
            list_app: ListApp::new(vec!["eth", "sol"]),
        }
    }
}

impl<'a> Drawable for Coin<'a> {
    fn draw_body(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        state: &mut AppState,
    ) {
        state.prev_screen = Screen::Welcome;

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
    fn set_keybinding(&mut self, key: Key, state: &mut AppState) {
        match key {
            Key::Char('\n') => match self.list_app.items.state.selected().unwrap_or(4) {
                0 => {
                    state.selected_coin = SelectedCoin::Eth;
                    state.screen = Screen::Accounts;
                    state.eth_accounts = (vec![], vec![], vec![]); //nullify existing accounts since we're gonna have new
                    state.sol_accounts = (vec![], vec![]);
                }
                1 => {
                    state.selected_coin = SelectedCoin::Sol;
                    state.screen = Screen::Accounts;
                    state.eth_accounts = (vec![], vec![], vec![]); //nullify existing accounts since we're gonna have new
                    state.sol_accounts = (vec![], vec![]);
                }
                _ => {}
            },
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
