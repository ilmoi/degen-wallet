use solana_client::rpc_client::RpcClient;

pub mod balance;
pub mod program;
pub mod transaction;

fn setup_solana_client() -> RpcClient {
    solana_client::rpc_client::RpcClient::new("https://api.devnet.solana.com".into())
}

pub fn u64_to_float(u64: u64, decimals: usize) -> f64 {
    let base: i32 = 10;
    u64 as f64 / base.pow(decimals as u32) as f64
}

pub fn float_to_u64(float: f64, decimals: usize) -> u64 {
    let base: i32 = 10;
    (float * base.pow(decimals as u32) as f64).floor() as u64
}
