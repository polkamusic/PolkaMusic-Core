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

## Balances

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



## CRM - Contract Rights Management
The Crm module is central to the managements of contracts rights. It allows to:
1) Store a new contract for rights managements, settings rules od sharing and quorums required to change the contract.  
2) Submit a change proposal of the rights managament that must be voted.  
3) Vote for a change proposal of rights management.  

### Add new contract. 
```newContract(crmid,crmdata,master,composition,othercontracts)```
This functions allow to store a new contracts for rights management with multiple fields:  

- "crmid" is the unique id of the contract (unsigned number 32 bit - u32).  
The generation of a unique is is external the logic of the blockchain. The function will check for duplicated id.  

- "crmdata" should contains a json structure regarding the main information of the contracts as follows:  
{  
	"ipfshash": "xxxxxx",            				// ipfs hash of the metadata (one hash is usable for whole folder of files)  
	"ipfshashprivate": ["xxxxxx","yyyyyyyy",..]     // ipfs hash array for the private files (audio and artworks)  
	"globalquorum": 80			    				// the quorum required to change the shares of master/composition and othercontracts (crowdfundingshare are not changeable)  
	"mastershare":30,               				// the shares for the master  
	"masterquorum":51,								// the quorum required to change the master data  
	"compositionshare": 30,         				// the shares of the composition group  
	"compositionquorum":51,							// the quorum required to change the composition data  
	"othercontractsshare": 20, 						// other contracts crowdfundingshare get shares (optional)  
	"othercontratsquorum":75,  						// the quorum required to change the other countracts data  
	"crowdfundingshare": 20,  						// crowd founders can get share   
	"crowdfounders": "xxxxxx"					    // crowd funding campaign Id  
}  
for example:  
```
{"ipfshash":"0E7071C59DF3B9454D1D18A15270AA36D54F89606A576DC621757AFD44AD1D2E","ipfshashprivate": "B45165ED3CD437B9FFAD02A2AAD22A4DDC69162470E2622982889CE5826F6E3D","globalquorum":100,"mastershare":50,"masterquorum":51,"compositionshare":50,"compositionquorum":51,"othercontractsshare":0,"othercontractsquorum":51}
```

- "master" field should contains a json with the informations regarding the shares for the Master(s):
{
    "master": [
        {"nickname": "xxxxxxxxxxxxx",                                                       // the nickname of the master's account
         "account": "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"    // the account of the master in hexadecimal format
         "percentage: xx}                                                                   // the percentage of rights for this master 
         ,{....}                                                                            // other master record
    ]
}
To be noticed that the total of the percentages must be = 100
for example:  
```
{"master": [{"nickname": "Bob","account": "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","percentage":50},{"nickname": "Bob Stash","account": "0xfe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e","percentage":50}]}
```

- "composition" field should contains a json with the informations regarding the shares for the composition members:
{  
    "composition": [  
        {"nickname": "xxxxxxxxxxxxx",                                                       // the nickname of the composition member's account  
         "account": "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"    // the account of the composition member in hexadecimal format  
         "percentage: xx}                                                                   // the percentage of rights for this composition member  
         ,{....}                                                                            // other composition member record record  
    ]  
}  
To be noticed that the total of the percentages must be = 100  
for example:  
```
{"composition": [{"nickname": "Charlie","account": "0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22","percentage":50},{"nickname": "Dave","account": "0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20","percentage":50}]}
```
- "othercontracts" field should contains a json with the informations the shares assigned to other contracts, this field is optional:  
{  
    "othercontracts": 
    [{  "id": x,                // contract id, it must be already present on chain  
        "percentage": xx        // percentage assigned to this contract id  
    },  
    {..}                        // other contract id/percentage  

To be noticed that the total of the percentages must be = 100.  
for example:  
```
{"othercontracts": [{"id":1,"percentage":50},{"id":2,"percentage":50}]}
```

The runtime works with hexadecimal accounts, to convert an SS58 account into hex format you can use the following tool:  
[https://polkadot.subscan.io/tools/ss58_transform(https://polkadot.subscan.io/tools/ss58_transform)]

The client libraries like https://polkadot.js.org/docs/ support the conversion easiliy.  
  
- Attenttion - Please don't use space behind numeric value in the json structure or the field be evaluated =0.  

### Change Proposal for main CRM data

Once stored, the contract for right management can be changed only through a voting process of the member.  
This functions allow to submit a change proposal from any account. Gas fees are charged to mitigate possible spamming.
The main data of the contract can be changed with the vote of the rights owners once the voting has reached the minimum quorum.
To submit a change proposal for the main CRM data, there is a specific function:  
```changeProposalCrmdata(changeid, crmdata)```  
- "changeid" is a unique id (unsigned number 32 bit - u32) to be assigned for the proposalof a specific contract id.  
- "crmdata" is the new json structure in the same format used for creating the new contract + an additional field "crmid" containing the id of the contract to change. For example: 
``` 
{"crmid":1,"ipfshash":"0E7071C59DF3B9454D1D18A15270AA36D54F89606A576DC621757AFD44AD1D2E","ipfshashprivate": "B45165ED3CD437B9FFAD02A2AAD22A4DDC69162470E2622982889CE5826F6E3D","globalquorum":75,"mastershare":60,"masterquorum":51,"compositionshare":40,"compositionquorum":51,"othercontractsshare":0,"othercontractsquorum":51}
```
The function generates events that can be intercepted for alterting the parts involved.

### Voting Change Proposal for Main CRM Data

The change proposal are kept in the queue for voting until they reach the quorum required. The user interface may decide for not showing the proposal after xx blocks.  

```voteProposalCrmdata(changeid, vote)```  
- "changeid" is a unique id (unsigned number 32 bit - u32) to be assigned for the proposal of a specific contract id.  
Different contract data category like main data,master data, composition data,Other Contracts data)  can use the same change id.  
- "vote" - is a booelan variable. It can be set to "Yes/True" to approve the proposal or "No/False" to disapprove.  


### Change Proposal for Master CRM data

The master data of the contract can be changed with the vote of the rights owners once the voting has reached the minimum quorum.
To submit a change proposal for the master CRM data, there is a specific function:  
```changeProposalCrmMasterdata(changeid, crmdata)```  
- "changeid" is a unique id (unsigned number 32 bit - u32) to be assigned for the proposalof a specific contract id.  
- "crmdata" is the new json structure in the same format used for creating the new contract + an additional filed "crmid" containing the id of the contract to change. For example: 
``` 
{"crmid":1,"master": [{"nickname": "Bob","account": "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","percentage":40},{"nickname": "Bob Stash","account": "0xfe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e", "percentage":60}]}
```
The function generates events that can be intercepted for alterting the parts involved.

### Voting Change Proposal for Master CRM Data

The change proposals are kept in the queue for voting until they reach the quorum required. The user interface may decide for not showing the proposal after xx blocks.  

```voteProposalCrmMasterdata(changeid, vote)```  
- "changeid" is a unique id (unsigned number 32 bit - u32) to be assigned for the proposal of a specific contract id.  
Different contract data category like main data,master data, composition data,Other Contracts data)  can use the same change id.  
- "vote" - is a booelan variable. It can be set to "Yes/True" to approve the proposal or "No/False" to disapprove.  


### Change Proposal for Composition CRM data

The composition data of the contract can be changed with the vote of the rights owners once the voting has reached the minimum quorum.
To submit a change proposal for the composition CRM data, there is a specific function:  
```changeProposalCrmCompositiondata(changeid, crmdata)```  
- "changeid" is a unique id (unsigned number 32 bit - u32) to be assigned for the proposalof a specific contract id.  
- "crmdata" is the new json structure in the same format used for creating the new contract + an additional filed "crmid" containing the id of the contract to change. For example: 
``` 
{"crmid":1,"composition": [{"nickname": "Bob","account": "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","percentage":40},{"nickname": "Bob Stash","account": "0xfe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e", "percentage":60}]}
```
The function generates events that can be intercepted for alterting the parts involved.

### Voting Change Proposal for Composition CRM Data

The change proposals are kept in the queue for voting until they reach the quorum required. The user interface may decide for not showing the proposal after xx blocks.  

```voteProposalCrmCompositiondata(changeid, vote)```  
- "changeid" is a unique id (unsigned number 32 bit - u32) to be assigned for the proposal of a specific contract id.  
Different contract data category like main data,master data, composition data,Other Contracts data)  can use the same change id.  
- "vote" - is a booelan variable. It can be set to "Yes/True" to approve the proposal or "No/False" to disapprove.  

### Change Proposal for Other Contracts CRM data

The Other Contracts data of the contract can be changed with the vote of the rights owners once the voting has reached the minimum quorum.
To submit a change proposal for the Other Contracts CRM data, there is a specific function:  
```changeProposalCrmOtherContractsdata(changeid, crmdata)```  
- "changeid" is a unique id (unsigned number 32 bit - u32) to be assigned for the proposalof a specific contract id.  
- "crmdata" is the new json structure in the same format used for creating the new contract + an additional filed "crmid" containing the id of the contract to change. For example: 
``` 
{"crmid":1,"othercontracts": [{"id":1,"percentage":70},{"id":2,"percentage":30}]}

```
The function generates events that can be intercepted for alterting the parts involved.

### Voting Change Proposal for Other Contracts CRM Data

The change proposals are kept in the queue for voting until they reach the quorum required. The user interface may decide for not showing the proposal after xx blocks.
The right of vote to change the other contracts is assigned to the same account stored in Master data.    

```voteProposalCrmOtherContractsdata(changeid, vote)```  
- "changeid" is a unique id (unsigned number 32 bit - u32) to be assigned for the proposal of a specific contract id.  
Different contract data category like main data,master data, composition data,Other Contracts data)  can use the same change id.  
- "vote" - is a booelan variable. It can be set to "Yes/True" to approve the proposal or "No/False" to disapprove.  


### Queries

You can query the maps stored, to get an updated list you can select "crm" from "Developer","Chain State" from the web interface:  
https://polkadot.js.org/apps/#/chainstate














