use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EthAddr(String);

impl EthAddr {
    pub fn new(addr: &str) -> Self {
        let mut proper_addr = addr.to_owned();
        //check for 0x prefix
        if !addr.starts_with("0x") {
            proper_addr = format!("0x{}", addr);
        }
        //check that passed str is a hex string
        hex::decode(&proper_addr[2..])
            .map_err(|e| {
                // println!("String passed into EthAddr is not hex.");
                e
            })
            .unwrap();
        //check length
        if proper_addr.len() != 42 {
            panic!(
                "String passed into EthAddr is {} hex chars long instead of 42.",
                proper_addr.len()
            );
        }
        //checksum and return
        let checksummed_addr = eth_checksum::checksum(&proper_addr);
        // println!("New eth addr: {}", checksummed_addr);
        Self(checksummed_addr)
    }
    pub fn get(&self) -> &str {
        &self.0
    }
}
