//! To add a token do 3 things:
//! 1) add its address to the hashmap in get_sol_token_addr
//! 2) add its decimal places to the hashmap in get_sol_token_decimals
//! 3) add the name to the TOKENS const at the top

use crate::sol::client::{float_to_u64, setup_solana_client, u64_to_float};
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use spl_token::instruction::transfer;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::str::FromStr;

// a little bit easier to manage than eth
// since spl_token is a standartized program - we don't need to import the abi of each token
// so we can just list token names and be done
const TOKENS: &[&str] = &["t1", "t2"];

// still have to do these painful get functions
// that's because I want to be able to iterate through tokens and pull their balances and decimals
// otherwise would have just hardcoded into CONSTs
// and rust currently doesn't supports hashmaps in consts - https://users.rust-lang.org/t/is-there-a-way-to-create-a-constant-map-in-rust/8358
fn get_sol_token_addr(token: &str) -> anyhow::Result<Pubkey> {
    let mut h = HashMap::new();

    // ########################## ADD MORE TOKENS HERE ##########################
    h.insert(
        "t1",
        Pubkey::from_str("5mkQ7uxYyYeWL64eg6cvrNFpEYppZ7gqiibUhbxcsMLG")?,
    );
    h.insert(
        "t2",
        Pubkey::from_str("7wih8Cjou8BSaAC3ErysgoMcfGKgo7Nv6ikjKGZfEzZA")?,
    );
    // ########################## ADD MORE TOKENS ^^^^ ##########################

    //not great, but we can't return a ref to a dict defined inside this fn
    let addr = h.get(token).ok_or(anyhow::anyhow!(
        "please add token ADDRESS to get_token_addr() function."
    ))?;
    Ok(addr.to_owned())
}

fn get_sol_token_decimals(token: &str) -> anyhow::Result<usize> {
    let mut h = HashMap::new();

    // ########################## ADD MORE TOKENS HERE ##########################
    h.insert("t1", 0);
    h.insert("t2", 0);
    // ########################## ADD MORE TOKENS ^^^^ ##########################

    //not great, but we can't return a ref to a dict defined inside this fn
    let decimals = h.get(token).ok_or(anyhow::anyhow!(
        "please add token DECIMALS to get_token_decimals() function."
    ))?;
    Ok(decimals.to_owned())
}

// ----------------------------------------------------------------------------- transact

pub fn transfer_spl_token(
    token: &str,
    payer: &Keypair,
    to_addr: &Pubkey,
    amount: f64,
) -> anyhow::Result<String> {
    let client = setup_solana_client();

    let token_mint_addr = get_sol_token_addr(&token)?;
    let decimals = get_sol_token_decimals(&token)?;

    let amount_native = float_to_u64(amount, decimals);

    let src_token_acc_addr = spl_associated_token_account::get_associated_token_address(
        &payer.pubkey(),
        &token_mint_addr,
    );

    let dest_token_acc_addr =
        spl_associated_token_account::get_associated_token_address(&to_addr, &token_mint_addr);

    ensure_assoc_acc_exists(
        &src_token_acc_addr,
        &payer.pubkey(),
        &payer,
        &token_mint_addr,
    )?;
    ensure_assoc_acc_exists(&dest_token_acc_addr, &to_addr, &payer, &token_mint_addr)?;

    let transfer_ix = transfer(
        &spl_token::id(),
        &src_token_acc_addr,
        &dest_token_acc_addr,
        &payer.pubkey(),
        &[&payer.pubkey()],
        amount_native,
    )?;

    let tx = Transaction::new_signed_with_payer(
        &[transfer_ix],
        Some(&payer.pubkey()),
        &[&*payer],
        client.get_recent_blockhash()?.0,
    );

    let tx_hash = client.send_transaction(&tx)?;
    // println!("transfer complete: {}", tx_hash);
    Ok(format!("{}", tx_hash))
}

pub fn ensure_assoc_acc_exists(
    token_acc_addr: &Pubkey,
    token_acc_owner_addr: &Pubkey,
    payer: &Keypair,
    token_mint_addr: &Pubkey,
) -> anyhow::Result<()> {
    let client = setup_solana_client();

    if let Err(_) = client.get_account(&token_acc_addr) {
        let create_dest_token_acc_ix =
            spl_associated_token_account::create_associated_token_account(
                &payer.pubkey(),
                &token_acc_owner_addr,
                &token_mint_addr,
            );
        let tx = Transaction::new_signed_with_payer(
            &[create_dest_token_acc_ix],
            Some(&payer.pubkey()),
            &[&*payer],
            client.get_recent_blockhash()?.0,
        );
        let r = client.send_transaction(&tx)?;
        // println!("assoc account created with signature: {}", r);
    };

    Ok(())
}

// ----------------------------------------------------------------------------- query

#[tokio::main]
pub async fn query_programs(
    wallet_addresses: &Vec<Pubkey>,
) -> anyhow::Result<HashMap<Pubkey, HashMap<String, f64>>> {
    let mut handles = vec![];
    let mut balances = HashMap::new();

    for token in TOKENS {
        //don't have a choice but to clone(), because we need actual addresses, not references due to move
        for wallet_addr in wallet_addresses.clone() {
            // same with token - no choice but to clone()
            let token = token.clone();
            let h = tokio::spawn(async move {
                let decimals = get_sol_token_decimals(&token)?;
                // if the function fails, means assoc account not yet created, means we make it a 0
                let raw_balance = query_program(&token, wallet_addr).unwrap_or(0);
                let float_balance = u64_to_float(raw_balance, decimals.to_owned());

                Ok::<(Pubkey, String, f64), anyhow::Error>((
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

pub fn query_program(token: &str, wallet_addr: Pubkey) -> anyhow::Result<u64> {
    let client = setup_solana_client();

    let token_mint_addr = get_sol_token_addr(&token)?;

    let token_acc_addr =
        spl_associated_token_account::get_associated_token_address(&wallet_addr, &token_mint_addr);

    // this will throw an error if the associated account doesn't yet exist
    let token_acc = client.get_account(&token_acc_addr)?;
    let token_acc_state = spl_token::state::Account::unpack(&token_acc.data.borrow())?;
    let balance = token_acc_state.amount;

    // println!("{}", balance);
    Ok(balance)
}
