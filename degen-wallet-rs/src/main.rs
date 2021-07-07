use bip39::Mnemonic;
use degen_wallet_rs::eth::wallet::external::{generate_eth_wallet, mnemonic_from_phrase};
use degen_wallet_rs::eth::wallet::internal::get_extended_keypair;
use degen_wallet_rs::eth::web3::{eth_to_wei, send_signed_tx, send_tx};
use degen_wallet_rs::tui::draw::draw_screen;
use hdpath::{Purpose, StandardHDPath};
use std::panic;
use std::panic::PanicInfo;
use std::str::FromStr;
use web3::types::{Address, H160, U256};

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
    // let a1 = H160::from_str("0xCd550E94040cEC1b33589eB99B0E1241Baa75D19").unwrap();
    // let a2 = H160::from_str("0xC48ad5fd060e1400a41bcf51db755251AD5A2475").unwrap();
    // println!("{:?}, {:?}", a1, a2);
    //
    // // ----------------------------------------------------------------------------- private key
    // let mnemonic = "window recall kid brief dragon worry intact board thumb aunt hair cement";
    // let mnemonic = mnemonic_from_phrase(mnemonic).unwrap();
    // let (a, pubk, prvk) = generate_eth_wallet(&mnemonic);
    // let prvk = prvk[0];
    // println!("signing with {}", prvk);
    //
    // // ----------------------------------------------------------------------------- send
    // send_signed_tx(a1, a2, prvk);
    draw_screen().unwrap();

    // let input = "0xBa313096524df5a200A2D15B845BA3Dca473fD5f, 1.123456";
    // let mut split = input.split(",");
    // let mut split_vec = split.collect::<Vec<&str>>();
    // let amount = split_vec.pop().unwrap().trim().parse::<f64>().unwrap();
    // let addr = split_vec.pop().unwrap().trim();
    // let addr = Address::from_str(addr).unwrap();
    // println!("{} {}", addr, amount);
    //
    // let eth = (amount * 100000.0) as u64;
    // let x = U256::from(eth) * U256::exp10(13);
    // println!("converted amount is {}", x);
}
