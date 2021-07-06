use std::error::Error;
use std::fs::DirEntry;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fs, io};

use bip39::{Language, Mnemonic, MnemonicType, Seed};
use bitcoin::util::bip32::ExtendedPubKey;
use bitcoin::{
    network::constants::Network,
    util::bip32::{DerivationPath, ExtendedPrivKey},
    PublicKey,
};
use eth_keystore::{decrypt_key, encrypt_key};
use hdpath::{Purpose, StandardHDPath};
use secp256k1::Secp256k1;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

use crate::eth::domain::EthAddr;
use crate::eth::internal::{
    get_extended_keypair, pubk_to_addr, remove_dir_contents, xpubk_to_pubk,
};

/// How it works:
/// 1. generate a new mnemonic (tiny-bip39)
/// 2. mnemonic -> entropy -> keystore file (eth-keystore)
/// 3. mnemonic -> seed -> xpub -> pk, pubk -> eth addr (hdpath, bitcoin, secp25k1, sha3, eth_checksum)
///
/// (!) todo I don't claim this wallet is 100% secure.
///     This is my first ever wallet put together with the help of reddit and a bunch of random websits.
///     For all I know it might be leaking private shit left and right.
///     The only guarantee at this point that I'm prepared to make is that it _works_.
///     As in - the private key / mnemonic / keystore file successfully access the same ethereum accounts derived below
///     When entered on https://www.myetherwallet.com/access-my-wallet
pub fn generate_eth_wallet(mnemonic: &Mnemonic) -> Vec<EthAddr> {
    let entropy = mnemonic.entropy();

    // ----------------------------------------------------------------------------- 1 main addr
    let secp = secp256k1::Secp256k1::new();
    let main_prvk = secp256k1::SecretKey::from_slice(entropy).unwrap();
    let main_pubk = secp256k1::PublicKey::from_secret_key(&secp, &main_prvk);
    let _main_addr = pubk_to_addr(main_pubk);
    // println!("main addr is {:?}", main_addr);

    // ----------------------------------------------------------------------------- 2 derived addr
    // get the HD wallet seed
    let seed = Seed::new(&mnemonic, ""); //128 hex chars = 512 bits
    let seed_bytes: &[u8] = seed.as_bytes();
    // println!("Seed: {:X}", seed);
    // println!("Seed as bytes: {:?}", seed_bytes);

    let mut derived_eth_addresses = vec![];

    for i in 0..10 {
        let hd_path = StandardHDPath::new(Purpose::Pubkey, 60, 0, 0, i);
        //Defined in the BIP 32 spec, extended private keys are a Base58 encoding of the private key, chain code, and some additional metadata.
        //xpk never stored in this impl
        let (_xprv, xpub) = get_extended_keypair(&seed_bytes, &hd_path);
        let pubk = xpubk_to_pubk(xpub);
        let eth_addr = pubk_to_addr(pubk);
        // √ verify against https://iancoleman.io/bip39/#english
        derived_eth_addresses.push(eth_addr);
    }

    derived_eth_addresses
}

pub fn mnemonic_from_phrase(mnemonic: &str) -> Result<Mnemonic, anyhow::Error> {
    Mnemonic::from_phrase(mnemonic, Language::English)
}

pub fn import_and_save_mnemonic(mnemonic: &Mnemonic, password: &str) -> String {
    encrypt_keystore_file(mnemonic, password)
}

pub fn generate_and_save_mnemonic(password: &str) -> (Mnemonic, String) {
    // ----------------------------------------------------------------------------- 1 mnemonic
    let mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);
    // let mnemonic = Mnemonic::from_phrase(
    //     "machine fabric tiny arctic alien brave start donkey near despair manual chest",
    //     Language::English,
    // )
    // .unwrap();

    // ----------------------------------------------------------------------------- 2 keystore
    let uuid = encrypt_keystore_file(&mnemonic, password);
    (mnemonic, uuid)
}

pub fn encrypt_keystore_file(mnemonic: &Mnemonic, password: &str) -> String {
    let dir = Path::new("./keys");
    let mut rng = rand::thread_rng();

    // my understanding is that you save the entropy, not the seed into keystore - https://support.mycrypto.com/general-knowledge/ethereum-blockchain/difference-between-wallet-types
    let entropy = mnemonic.entropy();
    // println!("Entropy: {:?}", entropy); //128 bits for 12 words, 256 bits for 24 words
    // println!("Entropy in hex: {}", hex::encode(entropy)); //this is the "private key" in traditional sense

    remove_dir_contents(dir).unwrap(); //clean out existing keys
    let uuid = encrypt_key(&dir, &mut rng, entropy, password).unwrap();
    // println!("{}", uuid);

    uuid
}

pub fn decrypt_keystore_file(password: &str) -> Result<Mnemonic, anyhow::Error> {
    let key_path = get_key_path().unwrap();
    let entropy = decrypt_key(&key_path, password)?;
    Mnemonic::from_entropy(&entropy, Language::English)
}

pub fn get_key_path() -> Option<PathBuf> {
    let dir = Path::new("./keys");
    let entry = fs::read_dir(dir).unwrap().next();
    return if let Some(entry) = entry {
        Some(entry.unwrap().path())
    } else {
        None
    };
}
