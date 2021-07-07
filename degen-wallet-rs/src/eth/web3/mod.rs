//! Using #[tokio::main] as workaround to make functions blocking
//! Currently I don't think there's a way to make the tui async
//! Even with async_trait, as soon as Drawable is made async there are tonnes of lifetime errors that are hard to decipher
//! The macro probably does something to the draw_body() fn which fucks with &mut state reference
//! Basically right now not a battle worth fighting

use secp256k1::SecretKey;
use std::str::FromStr;
use web3::transports::Http;
use web3::types::{Address, TransactionParameters, TransactionRequest, U256};
use web3::Web3;

// ----------------------------------------------------------------------------- tx

/// this only works on Ganache - infura requires signed raw tx
#[tokio::main]
pub async fn send_tx(from: Address, to: Address) -> anyhow::Result<String> {
    let web3 = setup_web3()?;
    let tx_request = TransactionRequest {
        from,
        to: Some(to),
        gas: None,
        gas_price: None,
        value: Some(U256::exp10(17)), //0.1 eth
        data: None,
        nonce: None,
        condition: None,
        transaction_type: None,
        access_list: None,
    };
    let result = web3.eth().send_transaction(tx_request).await?;
    // println!("{}", result);
    Ok(result.to_string())
}

/// works with infura - https://github.com/tomusdrw/rust-web3/issues/516
#[tokio::main]
pub async fn send_signed_tx(to: Address, amount: f64, prvk: &SecretKey) -> anyhow::Result<String> {
    let web3 = setup_web3()?;
    let tx_object = TransactionParameters {
        to: Some(to),
        value: eth_to_wei(amount),
        ..Default::default()
    };

    let signed = web3.accounts().sign_transaction(tx_object, prvk).await?;

    let result = web3
        .eth()
        .send_raw_transaction(signed.raw_transaction)
        .await?;
    // println!("{}", result);
    Ok(format!("{:?}", result))
}

// ----------------------------------------------------------------------------- balance

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

pub fn setup_web3() -> web3::Result<Web3<Http>> {
    // let transport = web3::transports::Http::new("http://localhost:7545")?;
    let transport = web3::transports::Http::new(
        "https://rinkeby.infura.io/v3/ce0c2c5a809d408892888f67e83bf5e4",
    )?;
    Ok(web3::Web3::new(transport))
}

pub fn wei_to_eth(wei: U256) -> f64 {
    let (quot, rem) = wei.div_mod(U256::exp10(18));
    let float = f64::from_str(&format!("{}.{}", quot, rem)).unwrap();
    // println!("{}", float);
    float
}

pub fn eth_to_wei(eth: f64) -> U256 {
    // preserving 5 decimal places
    let eth = (eth * 100000.0) as u64;
    // need to multiply by another 13, since we already did 5 above
    U256::from(eth) * U256::exp10(13)
}
