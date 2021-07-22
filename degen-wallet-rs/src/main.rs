use std::{panic, panic::PanicInfo};

use degen_wallet_rs::tui::draw::draw_screen;

/// since our app is a tui, and we're inside an alt screen, normal panics won't show
/// we need to write a custom panic hook
/// https://github.com/fdehau/tui-rs/issues/177
fn panic_hook(info: &PanicInfo<'_>) {
    let location = info.location().unwrap(); // The current implementation always returns Some

    let msg = match info.payload().downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<Any>",
        },
    };
    println!(
        "{}thread '<unnamed>' panicked at '{}', {}\r",
        termion::screen::ToMainScreen,
        msg,
        location
    );
}

fn main() {
    panic::set_hook(Box::new(panic_hook));
    draw_screen().unwrap();
}
