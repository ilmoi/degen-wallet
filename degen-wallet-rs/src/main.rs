use degen_wallet_rs::tui::draw::draw_screen;

use degen_wallet_rs::eth::web3::contract::{query_contract, query_contracts};
use std::panic;
use std::panic::PanicInfo;
use std::str::FromStr;
use web3::types::H160;

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

    // // ----------------------------------------------------------------------------- addresses
    let a1 = H160::from_str("0xCd550E94040cEC1b33589eB99B0E1241Baa75D19").unwrap();
    let a2 = H160::from_str("0xC48ad5fd060e1400a41bcf51db755251AD5A2475").unwrap();
    // println!("{:?}, {:?}", a1, a2);

    // // ----------------------------------------------------------------------------- eth wallet
    // let mnemonic = "window recall kid brief dragon worry intact board thumb aunt hair cement";
    // let mnemonic = mnemonic_from_phrase(mnemonic).unwrap();
    // let (a, pubk, prvk) = generate_eth_wallet(&mnemonic);

    // ----------------------------------------------------------------------------- draw
    draw_screen().unwrap();

    // ----------------------------------------------------------------------------- ctr
    // query_contracts(&vec![a1, a2]);
}
