use crate::eth::wallet::domain::StrAddr;
use crate::eth::wallet::external::generate_eth_wallet;
use crate::eth::web3::get_balances;
use crate::tui::helpers::TermBck;
use crate::tui::state::{AppState, Drawable, Screen};
use crate::tui::util::StatefulTable;

use termion::event::Key;
use tui::layout::{Constraint, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Cell, Row, Table};
use tui::Frame;

pub struct Accounts {
    pub account_table: StatefulTable,
    pub refresh_count: u8,
}

impl Accounts {
    pub fn new() -> Self {
        Self {
            account_table: StatefulTable::new(),
            refresh_count: 0,
        }
    }

    pub fn update_balances(&mut self, state: &mut AppState) {
        let balances = get_balances(&state.eth_accounts.0);

        //todo tried making this async / putting on another thread but problem with mutexes
        // asked - https://stackoverflow.com/questions/68254268/concurrency-in-a-rust-tui-app-lock-starvation-issue
        // I think the solution is either RefCell or channels. Decided to leave for now
        self.account_table.items = state
            .eth_accounts
            .0
            .iter()
            .enumerate()
            .map(|(i, a)| (a.to_str_addr(), balances[i]))
            .collect();
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
        state.prev_screen = Screen::Welcome;

        if state.eth_accounts.0.len() == 0 {
            state.eth_accounts = generate_eth_wallet(state.mnemonic.as_ref().unwrap());
            self.update_balances(state);
        }

        // refresh balances once every 10 seconds
        self.refresh_count += 1;
        if self.refresh_count >= 200 {
            self.update_balances(state);
            self.refresh_count = 0;
        }

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);

        let header_cells = ["Account", "Balance"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));

        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let rows = self.account_table.items.iter().map(|item| {
            let cells = vec![Cell::from(&item.0[..]), Cell::from(format!("{}", item.1))];
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
        f.render_stateful_widget(t, body_chunk, &mut self.account_table.state);
    }
    fn set_keybinding(&mut self, key: Key, state: &mut AppState) {
        match key {
            Key::Char('\n') => {
                state.selected_acc = self.account_table.state.selected().unwrap();
                state.screen = Screen::Transaction;
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
