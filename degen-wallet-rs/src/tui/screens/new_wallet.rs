use termion::event::Key;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Paragraph, Wrap};
use tui::Frame;

use crate::eth::generate_and_save_mnemonic;
use crate::tui::helpers::TermBck;
use crate::tui::state::{AppState, Drawable, Screen};

pub struct NewWallet {}

impl NewWallet {
    pub fn new() -> Self {
        Self {}
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
        //todo ask for pw

        if state.mnemonic.is_none() {
            // thread::spawn(|| {
            let (mnemonic, file_uuid) = generate_and_save_mnemonic();
            state.mnemonic = Some(mnemonic);
            state.file_uuid = Some(file_uuid);
            // });
        }

        let text = vec![
            Spans::from("Generating a new wallet..."),
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
                    format!("/keys/{}", state.file_uuid.as_ref().unwrap()),
                    Style::default().fg(Color::Cyan),
                ),
                Span::raw("."),
            ]),
            Spans::from("It's encrypted with your password so you can share it - but best not to."),
            Spans::from(vec![
                Span::raw("Hit "),
                Span::styled("<Enter>", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" when ready to proceed."),
            ]),
        ];

        let p = Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .block(body_block);

        f.render_widget(p, body_chunk);
    }
    fn set_keybinding(&mut self, key: Key, state: &mut AppState) {
        match key {
            Key::Char('\n') => {
                state.screen = Screen::Welcome;
            }
            _ => {}
        }
    }
}
