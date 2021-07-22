use secp256k1::SecretKey;
use web3::types::{Address, TransactionParameters, TransactionRequest, U256};

use crate::eth::web3::{float_to_u256, setup_web3};

/// works with infura - https://github.com/tomusdrw/rust-web3/issues/516
#[tokio::main]
pub async fn send_transaction_public(
    to: Address,
    amount: f64,
    prvk: &SecretKey,
) -> anyhow::Result<String> {
    let web3 = setup_web3()?;
    let tx_object = TransactionParameters {
        to: Some(to),
        value: float_to_u256(amount, 18),
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

/// this only works on Ganache - infura requires signed raw tx
#[tokio::main]
pub async fn send_transaction_private(from: Address, to: Address) -> anyhow::Result<String> {
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
