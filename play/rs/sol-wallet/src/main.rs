use std::str::FromStr;

use bip39::{Language, Mnemonic, Seed};
use bitcoin::{
    util::{
        bip32::{DerivationPath, ExtendedPrivKey, ExtendedPubKey},
        psbt::serialize::Serialize,
    },
    Network, PublicKey,
};
use hdpath::{Purpose, StandardHDPath};
use secp256k1::Secp256k1;
use sha3::Digest;
use web3::types::Address;
use std::convert::TryInto;
use solana_sdk::signature::Signer;
use solana_sdk::pubkey::Pubkey;

fn main() {
    // pk == seed == is 64 bytes long
    let pk = [
        85, 239, 63, 248, 184, 4, 53, 71, 125, 118, 216, 176, 12, 115, 49, 167, 34, 229, 163, 29,
        206, 99, 122, 38, 233, 228, 252, 181, 118, 235, 231, 45, 182, 65, 152, 76, 42, 84, 178,
        179, 24, 156, 102, 242, 131, 25, 226, 165, 163, 154, 147, 58, 34, 85, 220, 224, 116, 78,
        185, 2, 31, 74, 231, 162,
    ];

    // mnemonic is 12 words long
    let mnemonic = Mnemonic::from_phrase(
        "window recall kid brief dragon worry intact board thumb aunt hair cement",
        Language::English,
    )
        .unwrap();
    let seed = Seed::new(&mnemonic, ""); //128 hex chars = 512 bits = 64 bytes
    // hex -> bits https://www.scadacore.com/tools/programming-calculators/online-hex-converter/ (take each 8 bits
    // bits -> bytes https://www.mathsisfun.com/binary-decimal-hexadecimal-converter.html
    let seed_bytes: &[u8] = seed.as_bytes();
    println!("Seed: {:X}", seed);
    println!("Seed as bytes: {:?}", seed_bytes); //nice so this derives the exact same seed as above.

    // ----------------------------------------------------------------------------- eth - uses secp256k1
    // eth_keys(seed_bytes);

    // ----------------------------------------------------------------------------- sol - uses Ed25519
    sol_keys(seed_bytes);

}

pub fn sol_keys(seed_bytes: &[u8]) {
    // ----------------------------------------------------------------------------- deterministic
    let secret_key_bytes: [u8; 32] = seed_bytes[..32].try_into().unwrap();
    let secret_key = ed25519_dalek::SecretKey::from_bytes(&secret_key_bytes).unwrap();
    let public_key = ed25519_dalek::PublicKey::from(&secret_key);
    let secret_key_bs58 = bs58::encode(secret_key.as_bytes()).into_string();
    let public_key_bs58 = bs58::encode(public_key.as_bytes()).into_string();
    // nice this public key matches that derived by solana-keygen - DGTDP6fWYRSmQB1ocucKskAzEdDodjzHFSm86ZGzCE3s √
    println!("{:?}, {:?}", secret_key_bs58, public_key_bs58);

    // ----------------------------------------------------------------------------- 3 HD wallet (bip 32 + bip 44)
    // let mut private_keys = vec![];
    // let mut public_keys = vec![];
    // let mut derived_eth_addresses = vec![];

    for i in 0..10 {
        // note that all keys on solana are hardened - so with '
        // m/501'/60'/0'/i' - matches what I got using solana-keygen in terminal:
        // 0'/0' = C8xgzKnCAQbvQ5dEDtZmWfTbWJAzVcW91WXiu35aqjAH √
        // 0'/1'= 5g9h6vPci8WE9BNUGU3KC7FYyf5LivRW21LHAcdSTCb1 √
        // None/None = qp7uJTPA2k96uVK17VhdSiEgVwWhH1C5sTwzvX6yqGY √
        // 0'/None = 8mtimzx9WD3dfzU41dKyDfKLsjTkU5fDQjsBHw9gmmV √

        // let derivation_path = solana_sdk::derivation_path::DerivationPath::from_key_str("0").unwrap(); // <-- this also prepends with 501/0
        // let derivation_path = solana_sdk::derivation_path::DerivationPath::new_bip44(Some(0),None);
        let derivation_path = solana_sdk::derivation_path::DerivationPath::new_bip44(Some(i),Some(0));
        let keypair = solana_sdk::signer::keypair::keypair_from_seed_and_derivation_path(
            seed_bytes,
            Some(derivation_path),
        ).unwrap();
        let public_key = keypair.pubkey();

        let token_mint_addr = Pubkey::from_str("sDBauwT39Epe8Z8hcGxvNYje53K95GUjVo7vRDad2BJ").unwrap();
        let token_acc_addr =
        spl_associated_token_account::get_associated_token_address(&public_key, &token_mint_addr);

        println!("{}, {}", public_key, token_acc_addr);
    }
}

// -----------------------------------------------------------------------------
// ----------------------------------------------------------------------------- eth stuff below... just for ref

pub fn eth_keys(seed_bytes: &[u8]) {
    // ----------------------------------------------------------------------------- 2 deterministic wallet (bip 32)
    // NOTE: we need to use the left 32 bytes of the seed for deterministic wallet derivation More:
    // https://github.com/rust-bitcoin/rust-secp256k1/issues/321
    // "Split I into two 32-byte sequences, IL and IR." - https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki
    let secp = secp256k1::Secp256k1::new();
    let deterministic_prvk = secp256k1::SecretKey::from_slice(&seed_bytes[..32]).unwrap();
    let deterministic_pubk = secp256k1::PublicKey::from_secret_key(&secp, &deterministic_prvk);
    let deterministic_addr = pubk_to_addr(deterministic_pubk);
    println!(
        "addr: {:?}, pubk: {}, prvk: {}",
        deterministic_addr, deterministic_pubk, deterministic_prvk
    );

    // √ verify against https://www.myetherwallet.com/access-my-wallet

    // ----------------------------------------------------------------------------- 3 HD wallet (bip 32 + bip 44)
    let mut private_keys = vec![];
    let mut public_keys = vec![];
    let mut derived_eth_addresses = vec![];

    for i in 0..10 {
        // m/44'/60'/0'/0
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
    let mut hasher = sha3::Keccak256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let hex_r = hex::encode(result);
    // println!("{}", hex_r);
    hex_r
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
