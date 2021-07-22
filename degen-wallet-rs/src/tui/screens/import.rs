use termion::event::Key;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Paragraph, Wrap};
use tui::Frame;

use crate::eth::wallet::external::{import_and_save_mnemonic, mnemonic_from_phrase};
use crate::tui::helpers::TermBck;
use crate::tui::screens::import::ImportState::GetMnemonic;
use crate::tui::state::{AppState, Drawable, Screen};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};

pub enum ImportState {
    GetMnemonic,
    GetPassphrase,
    Wait,
    Import,
}

pub struct Import<'a> {
    pub mnemonic: String,
    pub passphrase: String,
    pub import_state: ImportState,
    pub msg: Vec<Span<'a>>,
}

impl Import<'_> {
    pub fn new() -> Self {
        Self {
            mnemonic: "".into(),
            passphrase: "".into(),
            import_state: GetMnemonic,
            msg: vec![
                Span::raw("Enter your Mnemonic. Hit "),
                Span::styled("<Enter>", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" when done. "),
            ],
        }
    }

    fn render_get_mnemonic(
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
        let text = Spans::from(self.msg.clone());

        let intro_p = Paragraph::new(text).wrap(Wrap { trim: true });
        f.render_widget(intro_p, chunks[0]);

        let input_p = Paragraph::new(self.mnemonic.as_ref())
            .style(
                Style::default()
                    .bg(Color::Rgb(20, 20, 20))
                    .fg(Color::Yellow),
            )
            .wrap(Wrap { trim: true });
        f.render_widget(input_p, chunks[1]);

        //set the cursor
        f.set_cursor(chunks[1].x + self.mnemonic.len() as u16, chunks[1].y);
    }

    fn render_get_passphrase(
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

        //set the cursor
        f.set_cursor(chunks[1].x + self.passphrase.len() as u16, chunks[1].y);
    }

    fn render_wait(&mut self, body_chunk: Rect, body_block: Block, f: &mut Frame<TermBck>) {
        let p = Paragraph::new("Importing your wallet...").block(body_block);
        f.render_widget(p, body_chunk);
        self.import_state = ImportState::Import;
    }

    fn render_import(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        state: &mut AppState,
    ) {
        //continue to render prev screen
        let p = Paragraph::new("Importing your wallet...").block(body_block);
        f.render_widget(p, body_chunk);

        import_and_save_mnemonic(state.mnemonic.as_ref().unwrap(), &self.passphrase);
        state.screen = Screen::Coin;
        self.import_state = ImportState::GetMnemonic; // in case we go through import twice
    }
}

impl Drawable for Import<'_> {
    fn draw_body(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        state: &mut AppState,
    ) {
        state.prev_screen = Screen::Welcome; //no point going back to import

        match self.import_state {
            ImportState::GetMnemonic => {
                self.render_get_mnemonic(body_chunk, body_block, f, state);
            }
            ImportState::GetPassphrase => {
                self.render_get_passphrase(body_chunk, body_block, f, state);
            }
            ImportState::Wait => self.render_wait(body_chunk, body_block, f),
            ImportState::Import => self.render_import(body_chunk, body_block, f, state),
        }
    }
    fn set_keybinding(&mut self, key: Key, state: &mut AppState) {
        match key {
            Key::Char('\n') => match self.import_state {
                ImportState::GetMnemonic => match mnemonic_from_phrase(&self.mnemonic) {
                    Ok(mnemonic) => {
                        state.mnemonic = Some(mnemonic);
                        self.import_state = ImportState::GetPassphrase;
                    }
                    Err(_) => self.msg.push(Span::styled(
                        "Bad mnemonic. Try again. ",
                        Style::default().fg(Color::Red),
                    )),
                },
                ImportState::GetPassphrase => {
                    self.import_state = ImportState::Wait;
                }
                _ => {}
            },
            Key::Char(c) => match self.import_state {
                ImportState::GetMnemonic => {
                    self.mnemonic.push(c);
                }
                ImportState::GetPassphrase => {
                    self.passphrase.push(c);
                }
                _ => {}
            },
            Key::Backspace => match self.import_state {
                ImportState::GetMnemonic => {
                    self.mnemonic.pop();
                }
                ImportState::GetPassphrase => {
                    self.passphrase.pop();
                }
                _ => {}
            },
            _ => {}
        }
    }
}
