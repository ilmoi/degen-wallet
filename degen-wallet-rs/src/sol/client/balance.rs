use solana_sdk::pubkey::Pubkey;

use crate::sol::client::{setup_solana_client, u64_to_float};

#[tokio::main]
pub async fn get_sol_balances(addresses: &Vec<Pubkey>) -> anyhow::Result<Vec<f64>> {
    let mut handles = vec![];
    let mut balances = vec![];

    for addr in addresses {
        let addr = addr.to_owned();
        let h = tokio::spawn(async move {
            let b = get_sol_balance(&addr).await?;
            Ok::<f64, anyhow::Error>(b)
        });
        handles.push(h);
    }

    for h in handles {
        balances.push(h.await??);
    }

    Ok(balances)
}

async fn get_sol_balance(address: &Pubkey) -> anyhow::Result<f64> {
    let client = setup_solana_client();
    let balance = client.get_balance(address)?;
    // LAMPORTS_PER_SOL = 10**9
    Ok(u64_to_float(balance, 9))
}
