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

pub mod balance;
pub mod contract;
pub mod transaction;

pub fn setup_web3() -> web3::Result<Web3<Http>> {
    // let transport = web3::transports::Http::new("http://localhost:7545")?;
    let transport = web3::transports::Http::new(
        "https://rinkeby.infura.io/v3/ce0c2c5a809d408892888f67e83bf5e4",
    )?;
    Ok(web3::Web3::new(transport))
}

//todo replace with the below
pub fn wei_to_eth(wei: U256) -> f64 {
    let (quot, rem) = wei.div_mod(U256::exp10(18));
    let float = f64::from_str(&format!("{}.{}", quot, rem)).unwrap();
    // println!("{}", float);
    float
}

//todo replace with the below
pub fn eth_to_wei(eth: f64) -> U256 {
    // preserving 5 decimal places
    let eth = (eth * 100000.0) as u64;
    // need to multiply by another 13, since we already did 5 above
    U256::from(eth) * U256::exp10(13)
}

pub fn u256_to_float(wei: U256, decimals: usize) -> anyhow::Result<f64> {
    let (quot, rem) = wei.div_mod(U256::exp10(decimals));
    let float = f64::from_str(&format!("{}.{}", quot, rem))?;
    // println!("{}", float);
    Ok(float)
}
