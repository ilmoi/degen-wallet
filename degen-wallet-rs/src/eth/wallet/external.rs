use std::fs;
use std::path::{Path, PathBuf};

use bip39::{Language, Mnemonic, MnemonicType, Seed};
use bitcoin::PublicKey;
use eth_keystore::{decrypt_key, encrypt_key};
use hdpath::{Purpose, StandardHDPath};
use secp256k1::SecretKey;
use web3::types::Address;

use crate::eth::wallet::internal::{
    get_extended_keypair, pubk_to_addr, remove_dir_contents, xprv_to_prvk, xpubk_to_pubk,
};

/// (!) todo I don't claim this wallet is 100% secure.
///     This is my first ever wallet put together with the help of reddit and a bunch of random websits.
///     For all I know it might be leaking private shit left and right.
///     The only guarantee at this point that I'm prepared to make is that it _works_.
///     As in - the private key / mnemonic / keystore file successfully access the same ethereum accounts derived below
///     When entered on https://www.myetherwallet.com/access-my-wallet
pub fn generate_eth_wallet(
    mnemonic: &Mnemonic,
) -> (
    Vec<Address>,
    Vec<secp256k1::PublicKey>,
    Vec<secp256k1::SecretKey>,
) {
    // seed vs entropy
    // seed = entropy that went through 2048 repeated rounds of HMAC-SHA256 hashing as described here https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki#from-mnemonic-to-seed
    // use seed, NOT entropy for private key calc - https://github.com/rust-bitcoin/rust-secp256k1/issues/321
    let seed = Seed::new(&mnemonic, ""); //128 hex chars = 512 bits = 64 bytes
    let seed_bytes: &[u8] = seed.as_bytes();
    // println!("Seed: {:X}", seed);
    // println!("Seed as bytes: {:?}", seed_bytes.len());

    // ----------------------------------------------------------------------------- 1 deterministic wallet
    // NOTE: we need to use the left 32 bytes of the seed for deterministic wallet derivation More:
    // https://github.com/rust-bitcoin/rust-secp256k1/issues/321
    // "Split I into two 32-byte sequences, IL and IR." - https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki
    // let secp = secp256k1::Secp256k1::new();
    // let deterministic_prvk = secp256k1::SecretKey::from_slice(&seed_bytes[..32]).unwrap();
    // let deterministic_pubk = secp256k1::PublicKey::from_secret_key(&secp, &deterministic_prvk);
    // let deterministic_addr = pubk_to_addr(deterministic_pubk);
    // println!(
    //     "addr: {:?}, pubk: {}, prvk: {}",
    //     deterministic_addr, deterministic_pubk, deterministic_prvk
    // );

    // √ verify against https://www.myetherwallet.com/access-my-wallet

    // ----------------------------------------------------------------------------- 2 HD wallet
    let mut private_keys = vec![];
    let mut public_keys = vec![];
    let mut derived_eth_addresses = vec![];

    for i in 0..10 {
        let hd_path = StandardHDPath::new(Purpose::Pubkey, 60, 0, 0, i);
        //Defined in the BIP 32 spec, extended private keys are a Base58 encoding of the private key, chain code, and some additional metadata.
        let (xprv, xpub) = get_extended_keypair(&seed_bytes, &hd_path);
        let prvk = xprv_to_prvk(xprv);
        let pubk = xpubk_to_pubk(xpub);
        let eth_addr = pubk_to_addr(pubk);

        // println!("{}, {}, {}", eth_addr, pubk, prvk);
        // √ verify against https://iancoleman.io/bip39/#english

        private_keys.push(prvk);
        public_keys.push(pubk);
        derived_eth_addresses.push(eth_addr);
    }

    (derived_eth_addresses, public_keys, private_keys)
}

pub fn mnemonic_from_phrase(mnemonic: &str) -> Result<Mnemonic, anyhow::Error> {
    Mnemonic::from_phrase(mnemonic, Language::English)
}

pub fn import_and_save_mnemonic(mnemonic: &Mnemonic, password: &str) -> String {
    encrypt_keystore_file(mnemonic, password)
}

pub fn generate_and_save_mnemonic(password: &str) -> (Mnemonic, String) {
    let mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);
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
