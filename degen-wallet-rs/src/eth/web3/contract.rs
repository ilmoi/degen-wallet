//! To add a token do 3 things:
//! 1) add its address to the hashmap in get_token_addr
//! 2) add its decimal places to the hashmap in get_token_decimals
//! 3) add the json abi to src/eth/web3/tokens dir

use std::{
    collections::HashMap,
    fs,
    fs::{DirEntry, File},
    io::{BufReader, Read},
    str::FromStr,
};

use secp256k1::SecretKey;
use serde_json::Value;
use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::{Address, H256, U256},
};

use crate::eth::web3::{float_to_u256, setup_web3, u256_to_float};

fn get_token_addr(token: &str) -> anyhow::Result<Address> {
    let mut h = HashMap::new();

    // ########################## ADD MORE TOKENS HERE ##########################
    h.insert(
        "uni",
        Address::from_str("0x1f9840a85d5af5bf1d1762f925bdaddc4201f984")?,
    );
    h.insert(
        "fdai",
        Address::from_str("0xf4beCBbFac50D4C0EdD58cc81177ad6713ACF457")?,
    );
    // ########################## ADD MORE TOKENS ^^^^ ##########################

    //not great, but we can't return a ref to a dict defined inside this fn
    let addr = h.get(token).ok_or(anyhow::anyhow!(
        "please add token ADDRESS to get_token_addr() function."
    ))?;
    Ok(addr.to_owned())
}

fn get_token_decimals(token: &str) -> anyhow::Result<usize> {
    let mut h = HashMap::new();

    // ########################## ADD MORE TOKENS HERE ##########################
    h.insert("uni", 18);
    h.insert("fdai", 18);
    // ########################## ADD MORE TOKENS ^^^^ ##########################

    //not great, but we can't return a ref to a dict defined inside this fn
    let decimals = h.get(token).ok_or(anyhow::anyhow!(
        "please add token DECIMALS to get_token_decimals() function."
    ))?;
    Ok(decimals.to_owned())
}

// ----------------------------------------------------------------------------- transact

/// works with infura
#[tokio::main]
pub async fn transfer_contract_public(
    token: &str,
    prvk: &SecretKey,
    to_addr: Address,
    amount: f64,
) -> anyhow::Result<String> {
    let token_addr = get_token_addr(&token)?;
    let decimals = get_token_decimals(&token)?;

    let contract = instantiate_contract(token, token_addr)?;

    let amount_native = float_to_u256(amount, decimals);

    let receipt = contract
        .signed_call_with_confirmations(
            "transfer",
            (to_addr, amount_native),
            Options::default(),
            1, //currently incorrectly handle the case where confirmations is set to 0 - https://github.com/tomusdrw/rust-web3/issues/195
            prvk,
        )
        .await?;

    Ok(format!("{:?}", receipt.transaction_hash))
}

/// only works with ganache
#[tokio::main]
pub async fn transfer_contract_private(
    token: &str,
    from_addr: Address,
    to_addr: Address,
    amount: f64,
) -> anyhow::Result<H256> {
    let token_addr = get_token_addr(&token)?;
    let decimals = get_token_decimals(&token)?;

    let contract = instantiate_contract(token, token_addr)?;

    let amount_native = float_to_u256(amount, decimals);

    let result = contract
        .call(
            "transfer",
            (to_addr, amount_native),
            from_addr,
            Options::default(),
        )
        .await?;

    // println!("{}", result);
    Ok(result)
}

// ----------------------------------------------------------------------------- query

#[tokio::main]
pub async fn query_contracts(
    wallet_addresses: &Vec<Address>,
) -> anyhow::Result<HashMap<Address, HashMap<String, f64>>> {
    let paths = fs::read_dir("src/eth/web3/tokens")?;
    let mut handles = vec![];
    let mut balances = HashMap::new();

    for path in paths {
        let token = extract_token_name(path?)?;
        //don't have a choice but to clone(), because we need actual addresses, not references due to move
        for wallet_addr in wallet_addresses.clone() {
            // same with token - no choice but to clone()
            let token = token.clone();
            let h = tokio::spawn(async move {
                let decimals = get_token_decimals(&token)?;
                let raw_balance = query_contract(&token, wallet_addr).await?;
                let float_balance = u256_to_float(raw_balance, decimals.to_owned())?;

                Ok::<(Address, String, f64), anyhow::Error>((
                    wallet_addr,
                    token.to_owned(),
                    float_balance,
                ))
            });
            handles.push(h);
        }
    }

    for h in handles {
        let h = h.await??;
        balances.entry(h.0).or_insert(HashMap::new());
        let token_map = balances
            .get_mut(&h.0)
            .ok_or(anyhow::anyhow!("failed to get token from hashmap"))?;
        token_map.insert(h.1, h.2);
    }

    // println!("{:?}", balances);
    Ok(balances)
}

// ----------------------------------------------------------------------------- helpers

async fn query_contract(token: &str, wallet_addr: Address) -> anyhow::Result<U256> {
    let token_addr = get_token_addr(&token)?;
    let contract = instantiate_contract(token, token_addr)?;

    let balance: U256 = contract
        .query("balanceOf", (wallet_addr,), None, Options::default(), None)
        .await?;

    // println!("{}", balance);
    Ok(balance)
}

fn instantiate_contract(token: &str, token_addr: Address) -> anyhow::Result<Contract<Http>> {
    let web3 = setup_web3()?;

    let file_content: String = read_file(&format!("src/eth/web3/tokens/{}.json", token));
    let json_abi: Value = serde_json::from_str(&file_content)?;
    let json_abi = json_abi.to_string();
    let json_abi = json_abi.as_bytes();

    let contract = Contract::from_json(web3.eth(), token_addr, json_abi)?;

    Ok(contract)
}

fn read_file(filepath: &str) -> String {
    let file = File::open(filepath).expect("could not open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    let _number_of_bytes: usize = match buffered_reader.read_to_string(&mut contents) {
        Ok(number_of_bytes) => number_of_bytes,
        Err(_err) => 0,
    };

    contents
}

fn extract_token_name(path: DirEntry) -> anyhow::Result<String> {
    let p = path.path().display().to_string();
    let split_slash = p.split("/");
    let file = split_slash.collect::<Vec<&str>>();

    let split_dot = file[file.len() - 1].split(".");
    let token = split_dot.collect::<Vec<&str>>()[0];

    // println!("Name: {}", token);
    Ok(String::from(token))
}
