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

- [X] Install RustLang with necessary dependencies as [illustrated here](doc/rust-setup.md).  


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

## Web Interface
A web interface for developers is available at this url:  [https://polkadot.js.org/apps](https://polkadot.js.org/apps)  
You should select your node address, for example ws://127.0.0.1:9944 

## Functions Available:

The callable functions aredivided in module (pallets), here the main modules:  

### Balances

The Balances module provides functions for:  
- Getting and setting free balances.  
- Retrieving total, reserved and unreserved balances.  
- Repatriating a reserved balance to a beneficiary account that exists.  
- Transferring a balance between accounts (when not reserved).  
- Slashing an account balance.  
- Account creation and removal.  
- Managing total issuance.  
- Setting and managing locks.  

Terminology:  
- Existential Deposit: The minimum balance required to create or keep an account open. This prevents "dust accounts" from filling storage. When the free plus the reserved balance (i.e. the total balance) fall below this, then the account is said to be dead; and it loses its functionality as well as any prior history and all information on it is removed from the chain's state. No account should ever have a total balance that is strictly between 0 and the existential deposit (exclusive). If this ever happens, it indicates either a bug in this pallet or an erroneous raw mutation of storage.  
- Total Issuance: The total number of units in existence in a system.  
- Reaping an account: The act of removing an account by resetting its nonce. Happens after its total balance has become zero (or, strictly speaking, less than the Existential Deposit).  
- Free Balance: The portion of a balance that is not reserved. The free balance is the only balance that matters for most operations.  
- Reserved Balance: Reserved balance still belongs to the account holder, but is suspended. Reserved balance can still be slashed, but only after all the free balance has been slashed.
- Imbalance: A condition when some funds were credited or debited without equal and opposite accounting (i.e. a difference between total issuance and account balances). Functions that result in an imbalance will return an object of the Imbalance trait that can be managed within your runtime logic. (If an imbalance is simply dropped, it should automatically maintain any book-keeping such as total issuance.)  
- Lock: A freeze on a specified amount of an account's free balance until a specified block number. Multiple locks always operate over the same funds, so they "overlay" rather than "stack".  

Transactions  (Existrinsics):
- setBalance(who, new_free, new_reserved) - Set the balances of a given account, it's allowed only by SUDO calls (super user access).  
- transfer(dest, value) - Transfer some liquid free balance to another account.  
- forceTransfer(source, dest, value) - Exactly as `transfer`, except the origin must be the super user by a SUDO calls and the source account may be different.  
- transferKeepAlive(dest, value) - Same as the [`transfer`] call, but with a check that the transfer will not kill the sending account because of minimum thresold reached.  

Queries:
- account(AccountId)-> AccountData - Get the balance of an account.  
- locks(AccountId)-> Vec<BalanceLock> - Any liquidity locks on the signing account balances.  
- totalissuance -> Balance - The total amount issued in the blockchain.  













