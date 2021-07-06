use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::eth::domain::EthAddr;
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
use std::error::Error;
use std::fs::DirEntry;
use std::io::ErrorKind;
use std::{fs, io};

pub fn remove_dir_contents<P: AsRef<Path>>(path: P) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        fs::remove_file(entry?.path())?;
    }
    Ok(())
}

pub fn get_extended_keypair(
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

    // println!("HD path: {}", hd_path);
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

pub fn xpubk_to_pubk(xpub: ExtendedPubKey) -> secp256k1::PublicKey {
    let pubk_str = xpub.public_key.to_string();
    secp256k1::PublicKey::from_str(&pubk_str).unwrap()
}

pub fn pubk_to_addr(pubk: secp256k1::PublicKey) -> EthAddr {
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

pub fn keccak_hash<T>(data: &T) -> String
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
