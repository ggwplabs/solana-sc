# GGWP Staking solana smart contracts

## Installing dependencies

```
$ cargo install --git https://github.com/project-serum/anchor avm --locked --force
$ avm use 0.25.0
```

Other dependencies instructions: <https://book.anchor-lang.com/getting_started/installation.html>

## Build the constract

```
$ anchor build
```

## Run the functional tests

```
$ npm install --legacy-peer-deps
$ anchor test
```

## Run the unit tests

```
$ cargo test --tests
```

## Deploy Freezing contract

Get the program_id

```
$ anchor build
$ solana-keygen pubkey target/deploy/freezing-keypair.json
<PROGRAM_ID_PUBKEY>
```

Set up the <PROGRAM_ID_PUBKEY> into declare_id! macro in programs/freezing/src/lib.rs

```
declare_id!("<PROGRAM_ID_PUBKEY>");
```

Deploy

```
$ anchor deploy -p freezing --provider.cluster <cluster>
```

Cluster must be one of [localnet, testnet, mainnet, devnet].

## Deploy Staking contract

Get the program_id

```
$ anchor build
$ solana-keygen pubkey target/deploy/staking-keypair.json
<PROGRAM_ID_PUBKEY>
```

Set up the <PROGRAM_ID_PUBKEY> into declare_id! macro in programs/staking/src/lib.rs

```
declare_id!("<PROGRAM_ID_PUBKEY>");
```

Deploy

```
$ anchor deploy -p staking --provider.cluster <cluster>
```

Cluster must be one of [localnet, testnet, mainnet, devnet].
