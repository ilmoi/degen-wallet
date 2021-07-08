use termion::event::Key;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Paragraph, Wrap};
use tui::Frame;

use crate::eth::wallet::external::decrypt_keystore_file;
use crate::tui::helpers::TermBck;
use crate::tui::state::{AppState, Drawable, Screen};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};

pub enum LoginState {
    GetPassword,
    Wait,
    Login,
}

pub struct Login<'a> {
    pub input: String,
    pub msg: Vec<Span<'a>>,
    pub login_state: LoginState,
}

impl Login<'_> {
    pub fn new() -> Self {
        Self {
            input: "".into(),
            msg: vec![
                Span::raw("Enter your passphrase. Hit "),
                Span::styled("<Enter>", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" when done. "),
            ],
            login_state: LoginState::GetPassword,
        }
    }

    fn render_get_pw(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        _state: &mut AppState,
    ) {
        f.render_widget(body_block, body_chunk);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(4)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(f.size());

        //Spans doesn't accept a reference to a vector, only the vector itself - hence have to use clone()
        let intro_p = Paragraph::new(Spans::from(self.msg.clone())).wrap(Wrap { trim: true });
        f.render_widget(intro_p, chunks[0]);

        let input_p = Paragraph::new(self.input.as_ref())
            .style(
                Style::default()
                    .bg(Color::Rgb(20, 20, 20))
                    .fg(Color::Yellow),
            )
            .wrap(Wrap { trim: true });
        f.render_widget(input_p, chunks[1]);

        //set the cursor
        f.set_cursor(chunks[1].x + self.input.len() as u16, chunks[1].y);
    }

    fn render_wait(&mut self, body_chunk: Rect, body_block: Block, f: &mut Frame<TermBck>) {
        let p = Paragraph::new("Logging you in...").block(body_block);
        f.render_widget(p, body_chunk);
        self.login_state = LoginState::Login;
    }

    fn render_login(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        state: &mut AppState,
    ) {
        // continue rendering prev screen
        let p = Paragraph::new("Logging you in...").block(body_block);
        f.render_widget(p, body_chunk);

        match decrypt_keystore_file(&self.input) {
            Ok(mnemonic) => {
                state.mnemonic = Some(mnemonic);
                state.screen = Screen::Accounts;
            }
            Err(_) => {
                self.msg.push(Span::styled(
                    "Bad passphrase. Try again. ",
                    Style::default().fg(Color::Red),
                ));
                self.login_state = LoginState::GetPassword;
            }
        }
    }
}

impl Drawable for Login<'_> {
    fn draw_body(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        state: &mut AppState,
    ) {
        state.prev_screen = Screen::Welcome; //no point going back to login

        match self.login_state {
            LoginState::GetPassword => self.render_get_pw(body_chunk, body_block, f, state),
            LoginState::Wait => self.render_wait(body_chunk, body_block, f),
            LoginState::Login => self.render_login(body_chunk, body_block, f, state),
        }
    }
    fn set_keybinding(&mut self, key: Key, _state: &mut AppState) {
        match key {
            Key::Char('\n') => match self.login_state {
                LoginState::GetPassword => self.login_state = LoginState::Wait,
                _ => {}
            },
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
