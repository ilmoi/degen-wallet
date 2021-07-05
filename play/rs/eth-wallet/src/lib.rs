use std::path::Path;
use std::str::FromStr;

use bip39::{Language, Mnemonic, MnemonicType, Seed};
use bitcoin::util::bip32::ExtendedPubKey;
use bitcoin::{
    network::constants::Network,
    util::bip32::{DerivationPath, ExtendedPrivKey},
    PublicKey,
};
use eth_keystore::encrypt_key;
use hdpath::{Purpose, StandardHDPath};
use secp256k1::Secp256k1;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

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
pub fn wallet() {
    // ----------------------------------------------------------------------------- 1 mnemonic
    let mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);
    // let mnemonic = Mnemonic::from_phrase(
    //     "machine fabric tiny arctic alien brave start donkey near despair manual chest",
    //     Language::English,
    // )
    // .unwrap();
    println!("Mnemonic: {}", mnemonic);

    // ----------------------------------------------------------------------------- 2 keystore
    // save it as a keystore file
    // my understanding is that you save the entropy, not the seed into keystore - https://support.mycrypto.com/general-knowledge/ethereum-blockchain/difference-between-wallet-types
    let entropy = mnemonic.entropy();
    println!("Entropy: {:?}", entropy); //128 bits for 12 words, 256 bits for 24 words
    println!("Entropy in hex: {}", hex::encode(entropy)); //this is the "private key" in traditional sense

    let mut rng = rand::thread_rng();
    let dir = Path::new("./keys");
    let uuid = encrypt_key(&dir, &mut rng, entropy, "password_to_keystore").unwrap();
    println!("{}", uuid);

    // -----------------------------------------------------------------------------
    let secp = secp256k1::Secp256k1::new();
    let main_prvk = secp256k1::SecretKey::from_slice(entropy).unwrap();
    let main_pubk = secp256k1::PublicKey::from_secret_key(&secp, &main_prvk);
    let main_addr = pubk_to_addr(main_pubk);
    println!("main addr is {:?}", main_addr);

    // ----------------------------------------------------------------------------- 3 derived addr
    // get the HD wallet seed
    let seed = Seed::new(&mnemonic, ""); //128 hex chars = 512 bits
    let seed_bytes: &[u8] = seed.as_bytes();
    println!("Seed: {:X}", seed);
    // println!("Seed as bytes: {:?}", seed_bytes);

    for i in (0..10) {
        let hd_path = StandardHDPath::new(Purpose::Pubkey, 60, 0, 0, i);
        //Defined in the BIP 32 spec, extended private keys are a Base58 encoding of the private key, chain code, and some additional metadata.
        //xpk never stored in this impl
        let (_xprv, xpub) = get_extended_keypair(&seed_bytes, &hd_path);
        let pubk = xpubk_to_pubk(xpub);
        let _eth_addr = pubk_to_addr(pubk);
    }

    // √ verify against https://iancoleman.io/bip39/#english
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EthAddr(String);

impl EthAddr {
    pub fn new(addr: &str) -> Self {
        let mut proper_addr = addr.to_owned();
        //check for 0x prefix
        if !addr.starts_with("0x") {
            proper_addr = format!("0x{}", addr);
        }
        //check that passed str is a hex string
        hex::decode(&proper_addr[2..])
            .map_err(|e| {
                println!("String passed into EthAddr is not hex.");
                e
            })
            .unwrap();
        //check length
        if proper_addr.len() != 42 {
            panic!(
                "String passed into EthAddr is {} hex chars long instead of 42.",
                proper_addr.len()
            );
        }
        //checksum and return
        let checksummed_addr = eth_checksum::checksum(&proper_addr);
        println!("New eth addr: {}", checksummed_addr);
        Self(checksummed_addr)
    }
    pub fn get(&self) -> &str {
        &self.0
    }
}

fn get_extended_keypair(
    seed: &[u8],
    hd_path: &StandardHDPath,
) -> (ExtendedPrivKey, ExtendedPubKey) {
    //https://wolovim.medium.com/ethereum-201-hd-wallets-11d0c93c87f7 this explains in-depth how derivation actually happens
    let secp = Secp256k1::new();
    let xprv = ExtendedPrivKey::new_master(Network::Bitcoin, seed)
        // we convert HD Path to bitcoin lib format (DerivationPath)
        .and_then(|k| k.derive_priv(&secp, &DerivationPath::from(hd_path)))
        .unwrap();
    let xpub = ExtendedPubKey::from_private(&secp, &xprv);

    println!("HD path: {}", hd_path);
    // println!(
    //     "xprv: {}, pk: {}, chain_code: {}",
    //     xpk, xpk.private_key, xpk.chain_code
    // );
    // println!(
    //     "xpub: {}, pubk: {}, chain_code: {}",
    //     xpubk, xpubk.public_key, xpubk.chain_code
    // );

    (xprv, xpub)
}

fn xpubk_to_pubk(xpub: ExtendedPubKey) -> secp256k1::PublicKey {
    let pubk_str = xpub.public_key.to_string();
    secp256k1::PublicKey::from_str(&pubk_str).unwrap()
}

fn pubk_to_addr(pubk: secp256k1::PublicKey) -> EthAddr {
    //format as uncompressed key, remove "04" in the beginning
    let pubk_uncomp = &PublicKey::new_uncompressed(pubk).to_string()[2..];
    //decode from hex and pass to keccak for hashing
    let pubk_bytes = hex::decode(pubk_uncomp).unwrap();
    let addr = &keccak_hash(&pubk_bytes);
    //keep last 20 bytes of the result
    let addr = &addr[(addr.len() - 40)..];
    //massage into domain unit
    EthAddr::new(addr)
}

fn keccak_hash<T>(data: &T) -> String
where
    T: ?Sized + Serialize + AsRef<[u8]>,
{
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let hex_r = hex::encode(result);
    // println!("{}", hex_r);
    hex_r
}
