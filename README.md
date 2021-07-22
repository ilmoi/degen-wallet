# Degen wallet

```
 _
//\
V  \
 \  \_
  \,'.`-.
   |\ `. `.
   ( \  `. `-.                        _,.-:\
    \ \   `.  `-._             __..--' ,-';/
     \ `.   `-.   `-..___..---'   _.--' ,'/
      `. `.    `-._        __..--'    ,' /
        `. `-_     ``--..''       _.-' ,'
          `-_ `-.___        __,--'   ,'
             `-.__  `----"""    __.-'
                  `--..____..--'
```

TUI-based wallet for both Ethereum and Solana. Built for fun and no profit.

# What it does

Let's you create a new mnemonic
![](https://media.giphy.com/media/7W47RXYCopdjtopVQS/giphy.gif)

Let's you import and existing

Saves it as a keystore file & let's you login back later

Let's you transact in eth and erc20 tokens

Let's you transact in sol and spl tokens

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


