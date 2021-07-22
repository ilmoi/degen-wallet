use termion::event::Key;
use tui::{
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, List, ListItem},
    Frame,
};

use crate::{
    eth::wallet::external::get_key_path,
    tui::{
        helpers::TermBck,
        state::{AppState, Drawable, Screen},
        util::ListApp,
    },
};

pub struct Welcome<'a> {
    pub list_app: ListApp<'a>,
    // pub trigger_new: bool,
}

impl<'a> Welcome<'a> {
    pub fn new() -> Self {
        Self {
            list_app: ListApp::new(vec!["create new", "import existing"]),
            // trigger_new: false,
        }
    }
}

impl<'a> Drawable for Welcome<'a> {
    fn draw_body(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        state: &mut AppState,
    ) {
        state.prev_screen = Screen::Welcome;

        if self.list_app.items.items.len() == 2 {
            if let Some(_key) = get_key_path() {
                self.list_app.items.items.insert(0, "login with passphrase");
            }
        }

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

        //this way can show a wait msg while thread does the work
        // if self.trigger_new {
        //     let block = Block::default().borders(Borders::ALL);
        //     let p = Paragraph::new("Generating wallet...")
        //         .block(block)
        //         .alignment(Alignment::Center);
        //     let area = centered_rect_fixed(30, 7, f.size());
        //     f.render_widget(Clear, area); //this clears out the background
        //     f.render_widget(p, area);
        //
        //     state.screen = Screen::NewWallet;
        // }
    }
    fn set_keybinding(&mut self, key: Key, state: &mut AppState) {
        match key {
            Key::Char('\n') => {
                // if self.list_app.items.state.selected().unwrap_or(4) == 0 {
                //     self.trigger_new = true;
                // }
                if self.list_app.items.items.len() == 2 {
                    match self.list_app.items.state.selected().unwrap_or(4) {
                        0 => {
                            state.screen = Screen::NewWallet;
                        }
                        1 => {
                            state.screen = Screen::ImportWallet;
                        }
                        _ => {}
                    }
                } else {
                    match self.list_app.items.state.selected().unwrap_or(4) {
                        0 => {
                            state.screen = Screen::Login;
                        }
                        1 => {
                            state.screen = Screen::NewWallet;
                        }
                        2 => {
                            state.screen = Screen::ImportWallet;
                        }
                        _ => {}
                    }
                }
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
