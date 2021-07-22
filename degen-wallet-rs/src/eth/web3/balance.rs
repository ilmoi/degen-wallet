use crate::eth::web3::{setup_web3, u256_to_float};

use web3::types::Address;

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
    u256_to_float(balance, 18)
}
