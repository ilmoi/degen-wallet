use bip39::{Mnemonic, Seed};
use solana_sdk::signature::Signer;
use std::convert::TryInto;

pub fn generate_sol_wallet(
    mnemonic: &Mnemonic,
) -> (
    Vec<solana_sdk::pubkey::Pubkey>,
    Vec<solana_sdk::signer::keypair::Keypair>,
) {
    // ----------------------------------------------------------------------------- 1 seed (bip 39)
    let seed = Seed::new(&mnemonic, ""); //128 hex chars = 512 bits = 64 bytes
    let seed_bytes: &[u8] = seed.as_bytes();
    // println!("Seed: {:X}", seed);
    // println!("Seed as bytes: {:?}", seed_bytes.len());

    // ----------------------------------------------------------------------------- 2 deterministic wallet (bip 32)
    // let secret_key_bytes: [u8; 32] = seed_bytes[..32].try_into().unwrap();
    // let secret_key = ed25519_dalek::SecretKey::from_bytes(&secret_key_bytes).unwrap();
    // let public_key = ed25519_dalek::PublicKey::from(&secret_key);
    // let secret_key_bs58 = bs58::encode(secret_key.as_bytes()).into_string();
    // let public_key_bs58 = bs58::encode(public_key.as_bytes()).into_string();
    // println!("{:?}, {:?}", secret_key_bs58, public_key_bs58);

    // ----------------------------------------------------------------------------- 3 HD wallet (bip 32 + bip 44)
    let mut keypairs = vec![];
    let mut public_keys = vec![];

    for i in 0..10 {
        let derivation_path =
            solana_sdk::derivation_path::DerivationPath::new_bip44(Some(i), Some(0));
        let keypair = solana_sdk::signer::keypair::keypair_from_seed_and_derivation_path(
            seed_bytes,
            Some(derivation_path),
        )
        .unwrap();
        let pubk = keypair.pubkey();

        keypairs.push(keypair);
        public_keys.push(pubk);
    }

    (public_keys, keypairs)
}
