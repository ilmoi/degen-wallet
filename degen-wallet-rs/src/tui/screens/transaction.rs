use termion::event::Key;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Paragraph, Wrap};
use tui::Frame;

use crate::eth::wallet::external::decrypt_keystore_file;
use crate::eth::web3::send_signed_tx;
use crate::tui::helpers::TermBck;
use crate::tui::state::{AppState, Drawable, Screen};
use sha3::digest::generic_array::typenum::private::Trim;
use std::str::FromStr;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use web3::types::Address;

#[derive(PartialEq)]
pub enum TxState {
    TxDetails,
    TxConfirmation,
}

pub struct Transaction<'a> {
    pub input: String,
    pub msg: Vec<Span<'a>>,
    pub tx_state: TxState,
    pub tx_hash: String,
}

impl Transaction<'_> {
    pub fn new() -> Self {
        Self {
            input: "0xC48ad5fd060e1400a41bcf51db755251AD5A2475, 0.123".into(),
            msg: vec![
                Span::raw("Enter \"to\" address and amount, in the following format: "),
                Span::styled(
                    "0xC48ad5fd060e1400a41bcf51db755251AD5A2475, 0.123",
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

        //todo clone() - Spans doesn't accept a reference
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

    pub fn render_tx_confirmation(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        state: &mut AppState,
    ) {
        let text = vec![
            Span::styled("Transaction succeeded. ", Style::default().fg(Color::Green)),
            Span::raw(format!("Tx Hash: {}", self.tx_hash)),
            Span::raw(" Press "),
            Span::styled("<Esc>", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to go back. "),
        ];
        let p = Paragraph::new(Spans::from(text))
            .block(body_block)
            .wrap(Wrap { trim: true });
        f.render_widget(p, body_chunk);
    }

    pub fn parse_input(&self) -> anyhow::Result<(Address, f64)> {
        let mut split = self.input.split(",");
        let mut split_vec = split.collect::<Vec<&str>>();
        let amount = split_vec
            .pop()
            .ok_or_else(|| anyhow::anyhow!("cant pop"))?
            .trim()
            .parse::<f64>()?;
        let addr = split_vec
            .pop()
            .ok_or_else(|| anyhow::anyhow!("cant pop"))?
            .trim();
        let addr = Address::from_str(addr)?;

        if split_vec.len() != 0 {
            return Err(anyhow::anyhow!("vector should have been left empty"));
        }

        Ok((addr, amount))
    }
}

impl Drawable for Transaction<'_> {
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
            TxState::TxConfirmation => {
                self.render_tx_confirmation(body_chunk, body_block, f, state)
            }
        }
    }
    fn set_keybinding(&mut self, key: Key, state: &mut AppState) {
        match key {
            Key::Char('\n') => {
                if self.tx_state == TxState::TxDetails {
                    if let Ok((to, amount)) = self.parse_input() {
                        let prvk = state.eth_accounts.2[state.selected_acc];
                        if let Ok(tx_hash) = send_signed_tx(to, amount, &prvk) {
                            self.tx_hash = tx_hash;
                            self.tx_state = TxState::TxConfirmation
                        } else {
                            self.msg.push(Span::styled(
                                "Tx failed. Try again. ",
                                Style::default().fg(Color::Red),
                            ))
                        }
                    } else {
                        self.msg.push(Span::styled(
                            "Bad input. Try again. ",
                            Style::default().fg(Color::Red),
                        ))
                    }
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
