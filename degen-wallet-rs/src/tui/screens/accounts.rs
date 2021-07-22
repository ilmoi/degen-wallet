use std::collections::BTreeMap;

use termion::event::Key;
use tui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, Row, Table},
    Frame,
};

use crate::{
    eth::{
        wallet::{domain::StrAddr, external::generate_eth_wallet},
        web3::{balance::get_balances, contract::query_contracts},
    },
    sol::{
        client::{balance::get_sol_balances, program::query_programs},
        wallet::external::generate_sol_wallet,
    },
    tui::{
        helpers::TermBck,
        state::{AppState, Drawable, Screen, SelectedCoin},
        util::StatefulTable,
    },
};

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
        match state.selected_coin {
            SelectedCoin::Eth => {
                let balances = get_balances(&state.eth_accounts.0).unwrap();
                let token_balances = query_contracts(&state.eth_accounts.0).unwrap();

                //todo tried making this async / putting on another thread but problem with mutexes
                // asked - https://stackoverflow.com/questions/68254268/concurrency-in-a-rust-tui-app-lock-starvation-issue
                // I think the solution is either RefCell or channels. Decided to leave for now
                self.account_table.items = state
                    .eth_accounts
                    .0
                    .iter()
                    .enumerate()
                    .map(|(i, a)| {
                        // add eth balance
                        let mut h: BTreeMap<String, f64> = BTreeMap::new();
                        h.insert("eth".into(), balances[i]);

                        // add token balance
                        let relevant_h_with_tokens = token_balances.get(&a).unwrap();
                        h.extend(
                            relevant_h_with_tokens
                                .into_iter()
                                .map(|(k, v)| (k.clone(), v.clone())),
                        );

                        (a.to_str_addr(), h)
                    })
                    .collect();
            }
            SelectedCoin::Sol => {
                let balances = get_sol_balances(&state.sol_accounts.0).unwrap();
                let token_balances = query_programs(&state.sol_accounts.0).unwrap();

                self.account_table.items = state
                    .sol_accounts
                    .0
                    .iter()
                    .enumerate()
                    .map(|(i, a)| {
                        // add sol balance
                        let mut h: BTreeMap<String, f64> = BTreeMap::new();
                        h.insert("sol".into(), balances[i]);

                        // add token balance
                        let relevant_h_with_tokens = token_balances.get(&a).unwrap();
                        h.extend(
                            relevant_h_with_tokens
                                .into_iter()
                                .map(|(k, v)| (k.clone(), v.clone())),
                        );

                        (a.to_string(), h)
                    })
                    .collect();
            }
        };
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

        // populate the accounts on 1st render
        match state.selected_coin {
            SelectedCoin::Eth => {
                if state.eth_accounts.0.len() == 0 {
                    state.eth_accounts = generate_eth_wallet(state.mnemonic.as_ref().unwrap());
                    self.update_balances(state);
                }
            }
            SelectedCoin::Sol => {
                if state.sol_accounts.0.len() == 0 {
                    state.sol_accounts = generate_sol_wallet(state.mnemonic.as_ref().unwrap());
                    self.update_balances(state);
                }
            }
        }

        // refresh balances once every 3 seconds
        self.refresh_count += 1;
        if self.refresh_count >= 100 {
            self.update_balances(state);
            self.refresh_count = 0;
        }

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);

        let rows = self.account_table.items.iter().map(|item| {
            let mut cells = vec![];
            // left most cell = address
            cells.push(Cell::from(&item.0[..]));

            // other cells for other tokens
            for (_key, value) in &item.1 {
                let f64_str = format!("{}", value);
                cells.push(Cell::from(f64_str));
            }

            Row::new(cells).height(1).bottom_margin(1)
        });

        let mut header_cells = vec![];
        header_cells.push(Cell::from("account"));
        let first_row = &self.account_table.items[0];

        for (key, _value) in &first_row.1 {
            header_cells.push(Cell::from(&key[..]).style(Style::default().fg(Color::Red)));
        }

        let mut widths = vec![];
        for _ in 0..header_cells.len() {
            widths.push(Constraint::Percentage(100 / header_cells.len() as u16 - 1));
        }

        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let t = Table::new(rows)
            .header(header)
            .block(body_block)
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&widths);
        f.render_stateful_widget(t, body_chunk, &mut self.account_table.state);
    }
    fn set_keybinding(&mut self, key: Key, state: &mut AppState) {
        match key {
            Key::Char('\n') => {
                state.selected_acc = self.account_table.state.selected().unwrap_or(0);
                match state.selected_coin {
                    SelectedCoin::Eth => {
                        state.screen = Screen::EthTransaction;
                    }
                    SelectedCoin::Sol => {
                        state.screen = Screen::SolTransaction;
                    }
                }
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
