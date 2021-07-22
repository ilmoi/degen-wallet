use termion::event::Key;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Clear, Paragraph, Wrap};
use tui::Frame;

use crate::eth::wallet::external::generate_and_save_mnemonic;
use crate::tui::helpers::TermBck;
use crate::tui::screens::new_wallet::NewWalletState::RequestPassword;
use crate::tui::state::{AppState, Drawable, Screen};

pub enum NewWalletState {
    RequestPassword,
    WaitForGen,
    GenerateMnemonic,
    DisplayMnemonic,
}

pub struct NewWallet {
    pub passphrase: String,
    pub new_wallet_state: NewWalletState,
}

impl NewWallet {
    pub fn new() -> Self {
        Self {
            passphrase: "".into(),
            new_wallet_state: RequestPassword,
        }
    }
    fn render_request_passphrase(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
    ) {
        f.render_widget(body_block, body_chunk);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(4)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(f.size());

        let text = Spans::from(vec![
            Span::raw("Enter a passphrase to protect your Mnemonic. Hit "),
            Span::styled("<Enter>", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" when done."),
        ]);

        let intro_p = Paragraph::new(text).wrap(Wrap { trim: true });
        f.render_widget(intro_p, chunks[0]);

        let input_p = Paragraph::new(self.passphrase.as_ref())
            .style(
                Style::default()
                    .bg(Color::Rgb(20, 20, 20))
                    .fg(Color::Yellow),
            )
            .wrap(Wrap { trim: true });
        f.render_widget(input_p, chunks[1]);

        //set the cursor to be visible
        f.set_cursor(chunks[1].x + self.passphrase.len() as u16, chunks[1].y);
    }

    fn render_wait(&mut self, body_chunk: Rect, body_block: Block, f: &mut Frame<TermBck>) {
        let p = Paragraph::new("Generating new wallet...").block(body_block);
        f.render_widget(p, body_chunk);
        self.new_wallet_state = NewWalletState::GenerateMnemonic;
    }

    fn render_gen(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        state: &mut AppState,
    ) {
        // continue to render prev screen
        let p = Paragraph::new("Generating new wallet...").block(body_block);
        f.render_widget(p, body_chunk);

        //strictly speaking don't need this if statement, but adding for extra protection
        if state.mnemonic.is_none() {
            let (mnemonic, file_uuid) = generate_and_save_mnemonic(&self.passphrase);
            state.mnemonic = Some(mnemonic);
            state.file_uuid = Some(file_uuid);
        };
        self.new_wallet_state = NewWalletState::DisplayMnemonic;
    }

    fn render_display_mnemonic(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        state: &mut AppState,
    ) {
        let text = vec![
            Spans::from(vec![
                Span::raw("Your mnemonic is: "),
                Span::styled(
                    format!("{}", state.mnemonic.as_ref().unwrap()),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
            Spans::from("Write it down and hide it in a good place."),
            Spans::from(vec![
                Span::raw("We also generated a Keystore file for you and saved it under "),
                Span::styled(
                    format!(
                        "/keys/{}",
                        state.file_uuid.as_ref().unwrap_or(&String::from(""))
                    ),
                    Style::default().fg(Color::Cyan),
                ),
                Span::raw("."),
            ]),
            Spans::from(
                "It's encrypted with your passphrase so you can share it - but best not to.",
            ),
            Spans::from(vec![
                Span::raw("Hit "),
                Span::styled("<Enter>", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" when ready to proceed."),
            ]),
        ];

        let p = Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .block(body_block);

        f.render_widget(Clear, body_chunk);
        f.render_widget(p, body_chunk);
    }
}

impl Drawable for NewWallet {
    fn draw_body(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        state: &mut AppState,
    ) {
        state.prev_screen = Screen::Welcome; //no point going back to new wallet

        match self.new_wallet_state {
            NewWalletState::RequestPassword => {
                self.render_request_passphrase(body_chunk, body_block, f)
            }
            NewWalletState::WaitForGen => self.render_wait(body_chunk, body_block, f),
            NewWalletState::GenerateMnemonic => self.render_gen(body_chunk, body_block, f, state),
            NewWalletState::DisplayMnemonic => {
                self.render_display_mnemonic(body_chunk, body_block, f, state)
            }
        }
    }
    fn set_keybinding(&mut self, key: Key, state: &mut AppState) {
        match key {
            Key::Char('\n') => match self.new_wallet_state {
                NewWalletState::RequestPassword => {
                    self.new_wallet_state = NewWalletState::WaitForGen;
                    state.mnemonic = None; //need to clear any previous mnemonics from logging in
                }
                NewWalletState::DisplayMnemonic => {
                    state.screen = Screen::Coin;
                }
                _ => {}
            },
            Key::Char(c) => {
                self.passphrase.push(c);
            }
            Key::Backspace => {
                self.passphrase.pop();
            }
            _ => {}
        }
    }
}
