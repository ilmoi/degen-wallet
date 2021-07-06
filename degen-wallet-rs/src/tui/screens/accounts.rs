use crate::eth::external::generate_eth_wallet;
use crate::tui::helpers::TermBck;
use crate::tui::state::{AppState, Drawable, Screen};
use crate::tui::util::StatefulTable;
use termion::event::Key;
use tui::layout::{Constraint, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Cell, Row, Table};
use tui::Frame;

pub struct Accounts {
    pub account_table: StatefulTable,
}

impl Accounts {
    pub fn new() -> Self {
        Self {
            account_table: StatefulTable::new(),
        }
    }
}

impl Drawable for Accounts {
    fn draw_body(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        state: &mut AppState,
    ) {
        if state.eth_accounts.len() == 0 {
            state.eth_accounts = generate_eth_wallet(state.mnemonic.as_ref().unwrap());
            self.account_table.items = state
                .eth_accounts
                .iter()
                .map(|a| (String::from(a.get()), 0.0))
                .collect();
        }

        let mut table = &mut self.account_table;

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);

        let header_cells = ["Account", "Balance"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));

        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let rows = table.items.iter().map(|item| {
            // let height = item
            //     .iter()
            //     .map(|content| content.chars().filter(|c| *c == '\n').count())
            //     .max()
            //     .unwrap_or(0)
            //     + 1;
            // let cells = item.iter().map(|c| Cell::from(*c));
            let cells = vec![
                Cell::from(item.0.clone()),
                Cell::from(format!("{}", item.1)),
            ];
            Row::new(cells).height(1).bottom_margin(1)
        });

        let t = Table::new(rows)
            .header(header)
            .block(body_block)
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Percentage(50),
                Constraint::Length(30),
                Constraint::Max(10),
            ]);
        f.render_stateful_widget(t, body_chunk, &mut table.state);
    }
    fn set_keybinding(&mut self, key: Key, state: &mut AppState) {
        match key {
            Key::Char('\n') => {
                state.screen = Screen::Welcome;
            }
            Key::Down => {
                self.account_table.next();
            }
            Key::Up => {
                self.account_table.previous();
            }
            _ => {}
        }
    }
}
