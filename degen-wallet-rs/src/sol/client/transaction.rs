use crate::sol::client::{float_to_u64, setup_solana_client};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::system_transaction::transfer;

pub fn send_sol(to: &Pubkey, amount: f64, payer: &Keypair) -> anyhow::Result<String> {
    let lamports = float_to_u64(amount, 9);

    let client = setup_solana_client();
    let tx = transfer(payer, to, lamports, client.get_recent_blockhash()?.0);
    let tx_hash = client.send_transaction(&tx)?;

    // println!("sol transfer complete : {}", tx_hash);
    Ok(format!("{}", tx_hash))
}
