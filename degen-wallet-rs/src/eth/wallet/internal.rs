use std::{fs, io, path::Path, str::FromStr};

use bitcoin::{
    network::constants::Network,
    util::bip32::{DerivationPath, ExtendedPrivKey, ExtendedPubKey},
    PublicKey,
};
use hdpath::StandardHDPath;
use secp256k1::Secp256k1;
use serde::Serialize;
use sha3::{Digest, Keccak256};
use web3::types::Address;

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
    //     xprv, xprv.private_key, xprv.chain_code
    // );
    // println!(
    //     "xpub: {}, pubk: {}, chain_code: {}",
    //     xpub, xpub.public_key, xpub.chain_code
    // );

    (xprv, xpub)
}

pub fn xprv_to_prvk(xprv: ExtendedPrivKey) -> secp256k1::SecretKey {
    //NOTE: because the library is for btc not eth, this is WIF encoded
    // need to add .key to get it from wif format to normal secret key format
    // more here - https://learnmeabitcoin.com/technical/wif
    xprv.private_key.key
}

pub fn xpubk_to_pubk(xpub: ExtendedPubKey) -> secp256k1::PublicKey {
    //NOTE: this is outputted in hex, no further massaging necessary
    xpub.public_key.key
}

pub fn pubk_to_addr(pubk: secp256k1::PublicKey) -> Address {
    //format as uncompressed key, remove "04" in the beginning
    let pubk_uncomp = &PublicKey::new_uncompressed(pubk).to_string()[2..];
    //decode from hex and pass to keccak for hashing
    let pubk_bytes = hex::decode(pubk_uncomp).unwrap();
    let addr = &keccak_hash(&pubk_bytes);
    //keep last 20 bytes of the result
    let addr = &addr[(addr.len() - 40)..];
    //massage into domain unit
    Address::from_str(addr).unwrap()
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
