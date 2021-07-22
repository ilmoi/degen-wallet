use degen_wallet_rs::tui::draw::draw_screen;

use degen_wallet_rs::sol::client::program::{query_program, transfer_spl_token};
use degen_wallet_rs::sol::client::transaction::send_sol;
use solana_sdk::pubkey::Pubkey;
use std::panic;
use std::panic::PanicInfo;
use std::str::FromStr;

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

    // // ----------------------------------------------------------------------------- addresses
    // let a1 = H160::from_str("0xCd550E94040cEC1b33589eB99B0E1241Baa75D19").unwrap();
    // let a2 = H160::from_str("0xC48ad5fd060e1400a41bcf51db755251AD5A2475").unwrap();
    // println!("{:?}, {:?}", a1, a2);

    // // ----------------------------------------------------------------------------- eth wallet
    // let mnemonic = "window recall kid brief dragon worry intact board thumb aunt hair cement";
    // let mnemonic = mnemonic_from_phrase(mnemonic).unwrap();
    // let (a, pubk, prvk) = generate_eth_wallet(&mnemonic);

    // ----------------------------------------------------------------------------- ctr
    // query_contracts(&vec![a1, a2]);

    // let r = transfer_contract_public("uni", prvk[0], a2, 0.01);

    // ----------------------------------------------------------------------------- test sol tx

    // let alice = Pubkey::from_str("Ga8HG4NzgcYkegLoJDmxJemEU1brewF2XZLNHd6B4wJ7").unwrap();
    // let bob = Pubkey::from_str("BxiV2mYXbBma1Kv7kxnn7cdM93oFHL4BhT9G23hiFfUP").unwrap();
    // let nob = Pubkey::from_str("C8xgzKnCAQbvQ5dEDtZmWfTbWJAzVcW91WXiu35aqjAH").unwrap();
    //
    // let secret = &[
    //     201, 101, 147, 128, 138, 189, 70, 190, 202, 49, 28, 26, 32, 21, 104, 185, 191, 41, 20, 171,
    //     3, 144, 4, 26, 169, 73, 180, 171, 71, 22, 48, 135, 231, 91, 179, 215, 3, 117, 187, 183, 96,
    //     74, 154, 155, 197, 243, 114, 104, 20, 123, 105, 47, 181, 123, 171, 133, 73, 181, 102, 41,
    //     236, 78, 210, 176,
    // ];
    // let payer = solana_sdk::signer::keypair::Keypair::from_bytes(secret).unwrap();
    //
    // // query_program("t1", alice).unwrap();
    // // transfer_spl_token("t1", &payer, bob, 1.0).unwrap();
    // send_sol(&bob, 0.1, &payer).unwrap();
}
