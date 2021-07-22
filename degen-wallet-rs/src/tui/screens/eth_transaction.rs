use std::str::FromStr;

use secp256k1::SecretKey;
use termion::event::Key;
use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Paragraph, Wrap},
    Frame,
};
use web3::types::Address;

use crate::{
    eth::web3::{contract::transfer_contract_public, transaction::send_transaction_public},
    tui::{
        helpers::TermBck,
        state::{AppState, Drawable, Screen},
    },
};

#[derive(PartialEq)]
pub enum TxState {
    TxDetails,
    TxWait,
    TxSend,
    TxConfirmation,
}

pub struct EthTransaction<'a> {
    pub input: String,
    pub msg: Vec<Span<'a>>,
    pub tx_state: TxState,
    pub tx_hash: String,
}

impl EthTransaction<'_> {
    pub fn new() -> Self {
        Self {
            input: "0xC48ad5fd060e1400a41bcf51db755251AD5A2475, eth, 0.123".into(),
            msg: vec![
                Span::raw("Enter \"to\" address and amount, in the following format: "),
                Span::styled(
                    "0xC48ad5fd060e1400a41bcf51db755251AD5A2475, eth, 0.123",
                    Style::default()
                        .add_modifier(Modifier::ITALIC)
                        .fg(Color::Cyan),
                ),
                Span::raw(" Hit "),
                Span::styled("<Enter>", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" when done. "),
            ],
            tx_state: TxState::TxDetails,
            tx_hash: "".into(),
        }
    }

    pub fn render_tx_details(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        state: &mut AppState,
    ) {
        f.render_widget(body_block, body_chunk);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(4)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(f.size());

        let text = Span::raw(format!(
            "Sending from {}",
            state.eth_accounts.0[state.selected_acc]
        ));
        let p = Paragraph::new(Spans::from(text)).wrap(Wrap { trim: true });
        f.render_widget(p, chunks[0]);

        //Spans doesn't accept a reference to a vector, only the vector itself - hence have to use clone()
        let intro_p = Paragraph::new(Spans::from(self.msg.clone())).wrap(Wrap { trim: true });
        f.render_widget(intro_p, chunks[1]);

        let input_p = Paragraph::new(self.input.as_ref())
            .style(
                Style::default()
                    .bg(Color::Rgb(20, 20, 20))
                    .fg(Color::Yellow),
            )
            .wrap(Wrap { trim: true });
        f.render_widget(input_p, chunks[2]);

        //set the cursor
        f.set_cursor(chunks[1].x + self.input.len() as u16, chunks[2].y);
    }

    fn render_wait(&mut self, body_chunk: Rect, body_block: Block, f: &mut Frame<TermBck>) {
        let p = Paragraph::new("Transmitting transaction...").block(body_block);
        f.render_widget(p, body_chunk);
        self.tx_state = TxState::TxSend;
    }

    fn render_send(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        state: &mut AppState,
    ) {
        // continue to render prev screen
        let p = Paragraph::new("Transmitting transaction...").block(body_block);
        f.render_widget(p, body_chunk);

        let prvk = state.eth_accounts.2[state.selected_acc];
        let (to, token, amount) = self.parse_input().unwrap(); //ok to unwrap coz we check before entering this state

        // here we triage eth and token transactions
        if let Ok(tx_hash) = EthTransaction::send_eth_or_tokens(to, token, amount, &prvk) {
            self.tx_hash = tx_hash;
            self.tx_state = TxState::TxConfirmation
        } else {
            self.msg.push(Span::styled(
                "Tx failed. Try again. ",
                Style::default().fg(Color::Red),
            ));
            self.tx_state = TxState::TxDetails;
        }
    }

    pub fn render_tx_confirmation(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        _state: &mut AppState,
    ) {
        let text = vec![
            Span::styled("Transaction succeeded. ", Style::default().fg(Color::Green)),
            Span::raw(format!("Tx Hash: {}", self.tx_hash)),
            Span::raw(" Press "),
            Span::styled("<Enter>", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to send another. "),
        ];
        let p = Paragraph::new(Spans::from(text))
            .block(body_block)
            .wrap(Wrap { trim: true });
        f.render_widget(p, body_chunk);
    }

    pub fn parse_input(&self) -> anyhow::Result<(Address, &str, f64)> {
        let split = self.input.split(",");
        let mut split_vec = split.collect::<Vec<&str>>();
        let amount = split_vec
            .pop()
            .ok_or_else(|| anyhow::anyhow!("cant pop"))?
            .trim()
            .parse::<f64>()?;

        let token = split_vec
            .pop()
            .ok_or_else(|| anyhow::anyhow!("cant pop"))?
            .trim();

        let addr = split_vec
            .pop()
            .ok_or_else(|| anyhow::anyhow!("cant pop"))?
            .trim();
        let addr = Address::from_str(addr)?;

        if split_vec.len() != 0 {
            return Err(anyhow::anyhow!("vector should have been left empty"));
        }

        Ok((addr, token, amount))
    }

    pub fn send_eth_or_tokens(
        to: Address,
        token: &str,
        amount: f64,
        prvk: &SecretKey,
    ) -> anyhow::Result<String> {
        if token == "eth" {
            send_transaction_public(to, amount, prvk)
        } else {
            transfer_contract_public(token, prvk, to, amount)
        }
    }
}

impl Drawable for EthTransaction<'_> {
    fn draw_body(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        state: &mut AppState,
    ) {
        state.prev_screen = Screen::Accounts;

        match self.tx_state {
            TxState::TxDetails => self.render_tx_details(body_chunk, body_block, f, state),
            TxState::TxWait => self.render_wait(body_chunk, body_block, f),
            TxState::TxSend => self.render_send(body_chunk, body_block, f, state),
            TxState::TxConfirmation => {
                self.render_tx_confirmation(body_chunk, body_block, f, state)
            }
        }
    }
    fn set_keybinding(&mut self, key: Key, _state: &mut AppState) {
        match key {
            Key::Char('\n') => {
                if self.tx_state == TxState::TxDetails {
                    if let Ok((_to, _token, _amount)) = self.parse_input() {
                        self.tx_state = TxState::TxWait;
                    } else {
                        self.msg.push(Span::styled(
                            "Bad input. Try again. ",
                            Style::default().fg(Color::Red),
                        ))
                    }
                } else if self.tx_state == TxState::TxConfirmation {
                    self.tx_state = TxState::TxDetails
                }
            }
            Key::Char(c) => {
                self.input.push(c);
            }
            Key::Backspace => {
                self.input.pop();
            }
            _ => {}
        }
    }
}
