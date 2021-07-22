use std::collections::HashMap;

use bip39::Mnemonic;
use termion::event::Key;
use tui::{layout::Rect, widgets::Block, Frame};
use web3::types::Address;

use crate::tui::{
    helpers::TermBck,
    screens::{
        accounts::Accounts, coin::Coin, eth_transaction::EthTransaction, import::Import,
        login::Login, new_wallet::NewWallet, sol_transaction::SolTransaction, welcome::Welcome,
    },
};

// ----------------------------------------------------------------------------- app state

pub enum SelectedCoin {
    Eth,
    Sol,
}

pub struct AppState {
    pub screen: Screen,
    pub prev_screen: Screen,
    pub mnemonic: Option<Mnemonic>,
    pub file_uuid: Option<String>,
    pub eth_accounts: (
        Vec<Address>,
        Vec<secp256k1::PublicKey>,
        Vec<secp256k1::SecretKey>,
    ),
    pub sol_accounts: (
        // in solana pubkey and address are the same so there's no need for 3 arrays
        Vec<solana_sdk::pubkey::Pubkey>,
        Vec<solana_sdk::signer::keypair::Keypair>, //decided to use keypair instead of secret key, as latter is a reference and I cba
    ),
    pub selected_coin: SelectedCoin,
    pub selected_acc: usize,
}

impl AppState {
    pub fn fresh_state() -> Self {
        Self {
            screen: Screen::Welcome,
            prev_screen: Screen::Welcome,
            mnemonic: None,
            file_uuid: None,
            eth_accounts: (vec![], vec![], vec![]),
            sol_accounts: (vec![], vec![]),
            selected_coin: SelectedCoin::Eth,
            selected_acc: 0,
        }
    }
}

// ----------------------------------------------------------------------------- screens

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Screen {
    Welcome,
    NewWallet,
    ImportWallet,
    Login,
    Accounts,
    EthTransaction,
    SolTransaction,
    Coin,
}

impl Screen {
    /// Hashmap of trait objects instead of generics.
    /// Generics wouldn't let us have heterogenous objects in the same hashmap.
    ///
    /// Another way to think about it: imagine Scren is an external library we wrote that is being called from this app.
    /// Screen has no idea what the user will want to draw - and it doesn't care.
    /// As long as items going into the hashmap implement Drawable they're good.
    ///
    /// The downside of this approach is the added cost during runtime due to dynamic dispatch.
    /// Arguably in this particular case we as the author of the tui app know all the screens,
    /// and we could use as simple match statement with static dispatch.
    ///
    /// https://doc.rust-lang.org/stable/book/ch17-02-trait-objects.html
    pub fn init_screens() -> HashMap<Screen, Box<dyn Drawable>> {
        let mut h: HashMap<Screen, Box<dyn Drawable>> = HashMap::new();
        h.insert(Screen::Welcome, Box::new(Welcome::new()));
        h.insert(Screen::NewWallet, Box::new(NewWallet::new()));
        h.insert(Screen::Accounts, Box::new(Accounts::new()));
        h.insert(Screen::ImportWallet, Box::new(Import::new()));
        h.insert(Screen::Login, Box::new(Login::new()));
        h.insert(Screen::EthTransaction, Box::new(EthTransaction::new()));
        h.insert(Screen::SolTransaction, Box::new(SolTransaction::new()));
        h.insert(Screen::Coin, Box::new(Coin::new()));
        h
    }
}

// ----------------------------------------------------------------------------- drawable

pub trait Drawable {
    fn draw_body(
        &mut self,
        body_chunk: Rect,
        body_block: Block,
        f: &mut Frame<TermBck>,
        state: &mut AppState,
    );
    fn set_keybinding(&mut self, key: Key, state: &mut AppState);
}
