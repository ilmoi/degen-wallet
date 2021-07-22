use termion::event::Key;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Block, List, ListItem};
use tui::Frame;

use crate::eth::wallet::external::get_key_path;
use crate::sol::client::balance::get_sol_balances;
use crate::sol::client::program::query_programs;
use crate::sol::wallet::external::generate_sol_wallet;
use crate::tui::helpers::TermBck;
use crate::tui::state::{AppState, Drawable, Screen, SelectedCoin};
use crate::tui::util::ListApp;
use std::collections::BTreeMap;

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
