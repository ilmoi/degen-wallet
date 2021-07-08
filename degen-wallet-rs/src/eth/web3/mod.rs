//! Using #[tokio::main] as workaround to make functions blocking
//! Currently I don't think there's a way to make the tui async
//! Even with async_trait, as soon as Drawable is made async there are tonnes of lifetime errors that are hard to decipher
//! The macro probably does something to the draw_body() fn which fucks with &mut state reference
//! Basically right now not a battle worth fighting

use std::str::FromStr;
use web3::transports::Http;
use web3::types::U256;
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

pub fn u256_to_float(u256: U256, decimals: usize) -> anyhow::Result<f64> {
    let (quot, rem) = u256.div_mod(U256::exp10(decimals));
    let float = f64::from_str(&format!("{}.{}", quot, rem))?;
    // println!("{}", float);
    Ok(float)
}

/// (!) will panic if <5 decimals
pub fn float_to_u256(float: f64, decimals: usize) -> U256 {
    // preserving 5 decimal places
    let u256 = (float * 100000.0) as u64;
    // subtracting 5 we already multiplied above
    U256::from(u256) * U256::exp10(decimals - 5)
}
