use std::str::FromStr;

use termion::event::Key;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Paragraph, Wrap};
use tui::Frame;
use web3::types::Address;

use crate::sol::client::program::transfer_spl_token;
use crate::sol::client::transaction::send_sol;
use crate::tui::helpers::TermBck;
use crate::tui::state::{AppState, Drawable, Screen};
use secp256k1::SecretKey;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;

#[derive(PartialEq)]
pub enum TxState {
    TxDetails,
    TxWait,
    TxSend,
    TxConfirmation,
}

pub struct SolTransaction<'a> {
    pub input: String,
    pub msg: Vec<Span<'a>>,
    pub tx_state: TxState,
    pub tx_hash: String,
}

impl SolTransaction<'_> {
    pub fn new() -> Self {
        Self {
            input: "6X46UvyMhWgUuMoPDquZo19mvL6r8puN771FERHJc3Nn, sol, 0.123".into(),
            msg: vec![
                Span::raw("Enter \"to\" address and amount, in the following format: "),
                Span::styled(
                    "6X46UvyMhWgUuMoPDquZo19mvL6r8puN771FERHJc3Nn, sol, 0.123",
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
            state.sol_accounts.0[state.selected_acc]
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

        let payer = &state.sol_accounts.1[state.selected_acc];
        let (to, token, amount) = self.parse_input().unwrap(); //ok to unwrap coz we check before entering this state

        // here we triage sol and token transactions
        if let Ok(tx_hash) = SolTransaction::send_sol_or_tokens(&to, token, amount, &payer) {
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

    pub fn parse_input(&self) -> anyhow::Result<(Pubkey, &str, f64)> {
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
        let addr = Pubkey::from_str(addr)?;

        if split_vec.len() != 0 {
            return Err(anyhow::anyhow!("vector should have been left empty"));
        }

        Ok((addr, token, amount))
    }

    pub fn send_sol_or_tokens(
        to: &Pubkey,
        token: &str,
        amount: f64,
        payer: &Keypair,
    ) -> anyhow::Result<String> {
        if token == "sol" {
            send_sol(to, amount, payer)
        } else {
            transfer_spl_token(token, payer, to, amount)
        }
    }
}

impl Drawable for SolTransaction<'_> {
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
