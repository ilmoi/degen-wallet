use termion::event::Key;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Block, List, ListItem};
use tui::Frame;

use crate::tui::helpers::{ListApp, TermBck};
use crate::tui::state::{AppState, Drawable, Screen};

pub struct Welcome<'a> {
    pub list_app: ListApp<'a>,
}

impl<'a> Welcome<'a> {
    pub fn new() -> Self {
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
        _state: &mut AppState,
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
    fn set_keybinding(&mut self, key: Key, state: &mut AppState) {
        match key {
            Key::Char('\n') => {
                // println!("{}", self.list_app.items.state.selected().unwrap());
                state.screen = Screen::NewWallet;
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
