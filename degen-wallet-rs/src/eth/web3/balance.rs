use crate::eth::web3::{setup_web3, u256_to_float};
use secp256k1::SecretKey;
use std::str::FromStr;
use web3::transports::Http;
use web3::types::{Address, TransactionParameters, TransactionRequest, U256};
use web3::Web3;

#[tokio::main]
pub async fn get_balances(addresses: &Vec<Address>) -> anyhow::Result<Vec<f64>> {
    let mut handles = vec![];
    let mut balances = vec![];

    for addr in addresses {
        let addr = addr.to_owned();
        let h = tokio::spawn(async move {
            let b = get_balance(addr).await?;
            Ok::<f64, anyhow::Error>(b)
        });
        handles.push(h);
    }

    for h in handles {
        balances.push(h.await??);
    }

    Ok(balances)
}

async fn get_balance(address: Address) -> anyhow::Result<f64> {
    let web3 = setup_web3()?;
    //requires an actual address, not a reference (not ideal)
    let balance = web3.eth().balance(address, None).await?;
    Ok(u256_to_float(balance, 18)?)
}
