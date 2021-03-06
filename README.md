# Degen wallet 🍌

TUI-based wallet for both Ethereum and Solana. Built for fun and no profit.

# What it does

- Let's you create a new wallet with a fresh mnemonic

![](https://i.imgur.com/zpPDxuY.gif)

- Let's you import an existing one

![](https://i.imgur.com/U93qDPc.gif)

- Saves it as a keystore file & let's you login back later

![](https://i.imgur.com/umGSbn3.gif)

- Let's you transact in eth and erc20 tokens

![](https://i.imgur.com/Sn22Uxj.gif)

- Let's you transact in sol and spl tokens

![](https://i.imgur.com/2GPn5gM.gif)

# Use
```asm
cd degen-wallet-rs
cargo run
```

Currently only works on mac / linux due to limitations of [Termion backend](https://crates.io/crates/termion). If you're on windows you can package it up using a simple Dockerfile along the lines of:
```Dockerfile
FROM rust:latest
WORKDIR app
COPY . .
CMD ["cargo run"]
```

`play` dir simply has a bunch of play code I wrote when exploring various libraries / learned how things work.

To add more tokens visit `src/sol/client/program` for spl or `src/eth/web3/contract` for erc20. It's manual and it sucks but hey this was a toy project.

# Original plan 
1. Build a wallet ✅
2. Do it for both eth and sol ✅
3. Do it in rust ✅
4. Use it to interact with the two defi ecosystems ❌ (decided too much for a tui-wallet)
5. See if I can do cross-chain stuff ❌ (decided too much for a tui-wallet)


