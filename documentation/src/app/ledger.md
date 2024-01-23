# Ledger

The ledger hardware communicates via the application protocol data unit [(APDU)](https://en.wikipedia.org/wiki/Smart_card_application_protocol_data_unit). This unit is responsible for the communication between the ledger hardware and the host computer. The APDU is a binary format and is used to send commands to the ledger hardware and to receive responses from the ledger hardware. The protocol consists of a command response. 

## Command
An APDU command consists of the following fields:

| Field | Description |
| --- | --- |
| CLA | Class of the instruction, indicates the type of the command interindustry or proprietary |
| INS | Instruction code, this is defined by each ledger application, think of it as the application instruction set |
| P1 & P2| Instruction parameters if the instruction has arguments |
| Lc | Number of bytes present in the data field of the command. |
| Data | Data field of the command. |
| Le | Maximum number of bytes expected in the data field of the response to the command. |


## Response
The response to an APDU command consists of the following fields:

| Field | Description |
| --- | --- |
| Data | Data field of the response. |
| SW1 & SW2 | Status word of the response. |

A complete list of the status words can be found [here](https://www.eftlab.com/knowledge-base/complete-list-of-apdu-responses).

## Ledger Ethereum
The embedded Ethereum app on the ledger hardware is responsible for signing Ethereum transactions. This is the application we want to communicate with. The application has a set of instructions that it understands. The source code for the application is [here](https://github.com/LedgerHQ/app-ethereum). When a user updates their ethereum application they do it with the next version release of this repository. 

## Ledger Rust SDK Ecosystem
There are a few abstractions around this protocol different people have worked on. The most notable ones are:

- [Official Ledger SDK](https://github.com/LedgerHQ/ledger-device-rust-sdk): I tried working with this one first. It was not bad and I still look at it for reference. There is also this other library [Ledger Tauri](https://github.com/LYC386/ledger-tauri). That is a good reference I still look at.
- [ether-rs ledger signer](https://github.com/gakonst/ethers-rs/tree/master/ethers-signers): I tried working with this one second because I was excited about having some generic abstraction over signers. Turns out this one is kind of broken. However, I did learn that it was built by James on top of this well-maintained library summa coins
- [summa-coins](https://github.com/summa-tx/coins): This is the library I ended up building my solution on top of. It is also what will be used when rewriting the ledger-signer in alloy (or at least that's what James told me) so it felt like the best option. I was impressed with this library and how well it was built. It also has support for both bip32 and bip39.

## Ledger Client

LedgerClient is the SDK in our repository built on top of the summa-coins library. It provides a minimal SDK around the Ledger Ethereum application. It currently supports a subset of all the ethereum ledger instructions but we can add more support later.

### Utilization

When communicating with the ledger device we need to acquire a lock on the `HIDTransport`. This means that if there is another application talking to the ledger, this will not work. After obtaining the lock on the ledger we can interact with the ethereum application if it is open. If it is not open we will only be able to send instructions that return meta-data about the application. For example, we can check the version of the application. 

```rust ignore
extern crate clients;
use clients::ledger::LedgerClient;

let ledger = LedgerClient::new_connection(clients::ledger::types::DerivationType::LedgerLive(0)).await;
```

When creating a new ledger connection you must specify the account derivation path indicated by bitcoin improvement proposal 32 [bip32](https://www.youtube.com/watch?v=2HrMlVr1QX8). This allows for a hierarchical deterministic wallet. The derivation path is a string that looks like this `m/44'/60'/0'/0/0`. The first part of the path is the purpose. The second part is the coin type. The third part is the account. The fourth part is the change. The fifth part is the address index. The `LedgerClient` will use this path to derive the public key for the address index. 

In most cases we will want to use the `LedgerLive` derivation type. This is the derivation path used by the ledger live application. The `LedgerLive` derivation type takes an index as an argument. This is the index of the account in the ledger live application. The `LedgerLive` derivation type will derive the derivation path for the account at the index. 

When we have a ledger connection we can use it to sign transactions by giving it an ethers transaction request 

```rust ignore
use ethers::prelude::TransactionRequest;
use clients::ledger::LedgerClient;

let ledger = LedgerClient::new_connection(clients::ledger::types::DerivationType::LedgerLive(0)).await;

let tx = TransactionRequest::default();

// This currently correctly prompts the user to review this transaction
let sig = ledger.sign_tx(&tx).await.unwrap();
// once a user approves the transaction this will resolve and return the ethers signature type
```