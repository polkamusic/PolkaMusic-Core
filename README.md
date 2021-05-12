# PolkaMusic

PolkaMusic is a [substrate](https://github.com/paritytech/substrate) based public chain crafted exclusively for operating decentralized music businesses on blockchain. Envisioned as a 360 infrastructure for decentralized music, music entrepreneurs can not only create their own micro-economy for their music platform, but also leverage the ever growing list of features such as NFT, crowdfunding, decentralized storage and more like on-chain governance using $POLM native tokens. PolkaMusic also allows existing music blockchains to connect to the Polkadot ecosystem in order to interconnect siloed music economies.

## Development

> The chain is a fork of the official [Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template) - a supported starting point that decreases the time it takes to implement a custom next-generation blockchain.

> The network is not ready for use, under development :hammer_and_wrench:

> For more information about the project check the website :link:[PolkaMusic](https://polkamusic.io), the current version of PolkaMusic.io hosts a fully functional streaming platform that will connect to the “Smart Streaming Platform (SSP)” WASM Module.

## Project Details

**Architecture**

As a first step, a “Smart Streaming Platform (SSP)” WASM Module will be created that will allow music entrepreneurs to be a part in the genesis of a brand new SSP microeconomy that rewards the artists and the respected content creators autonomously. While initializing the module, the SSP developer (initial admin) will be able to customize deployment parameters such as token name, initial balance, inflation setting, etc.

![Smart Streaming Platform (SSP) Module](https://user-images.githubusercontent.com/76401865/113667186-09fdf600-96ce-11eb-970c-4f70c5895c9f.jpg)

**$SSP_Token** - When a Smart Streaming Platform initializes the SSP WASM Module, the platform can issue its own token that can be used for royalty payments, membership fee and internal governance.

**Custom Inflation Logic** - In order to pay the artists in $SSP_Token, a daily reward pool must be created that results in inflation of $SSP_Token supply.

**Smart Record Contracts (SRC)** - Artists can create Smart Record Contracts for each song, containing meta data such as song name, license and the wallet information of multiple stakeholders.

**Stream Reports** - Every streams from the frontend will be reported via Off-chain workers to trigger the business logic as described in the smart record contract by that author.

**Royalty Splitter** - Whenever a payment is made to a Smart Record Contract, the payment is automatically split between the constituent band member’s wallets based on weights assigned during SRC creation.

**Autonomous Royalty Payments** - Processes the stream reports in a rolling 24 hours period by autonomously calculating the play_time against the usage of the whole platform in the past 24 hours and paying out of the reward pool.

### Build & Run Node

#### Prerequisites

- [X] Clone this repo and update the submodules:

```
git clone https://github.com/polkamusic/PolkaMusic-Core
cd polkamusic
```

- [X] Install RustLang with necessary dependencies

### Commands

```
cargo run --release -- --dev --tmp
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/polkamusic-node -h
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/polkamusic-node --dev
```

Command to purge the development chain's state:

```bash
./target/release/polkamusic-node purge-chain --dev
```

### Specs

- This chain uses ```substrate``` v3.0.0
