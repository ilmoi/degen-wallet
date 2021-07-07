use crate::eth::web3::{setup_web3, wei_to_eth};
use secp256k1::SecretKey;
use std::str::FromStr;
use web3::transports::Http;
use web3::types::{Address, TransactionParameters, TransactionRequest, U256};
use web3::Web3;

#[tokio::main]
pub async fn get_balances(addresses: &Vec<Address>) -> Vec<f64> {
    let mut handles = vec![];
    let mut balances = vec![];

    for addr in addresses {
        let addr = addr.to_owned();
        let h = tokio::spawn(async move { get_balance(addr).await });
        handles.push(h);
    }

    for h in handles {
        balances.push(h.await.unwrap());
    }

    balances
}

pub async fn get_balance(address: Address) -> f64 {
    let web3 = setup_web3().unwrap();
    //requires an actual address, not a reference (not ideal)
    let balance = web3.eth().balance(address, None).await.unwrap();
    wei_to_eth(balance)
}
