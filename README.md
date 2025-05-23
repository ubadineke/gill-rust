# Implementation of the [gill](https://github.com/solana-foundation/gill/) library in Rust

## Installation

```bash
cargo add gill
```

## Quick start

<!-- > Find a collection of example code snippets using `gill` inside the
> [`/examples` directory](https://github.com/solana-foundation/gill/tree/master/examples), including
> [basic operations](https://github.com/solana-foundation/gill/tree/master/examples/get-started) and common
> [token operations](https://github.com/solana-foundation/gill/tree/master/examples/tokens). -->

- [Create a Solana RPC connection](#create-a-solana-rpc-connection)
- [Making Solana RPC calls](#making-solana-rpc-calls)
- [Create a transaction](#create-a-transaction)
- [Signing transactions](#signing-transactions)
- [Simulating transactions](#simulating-transactions)
- [Sending and confirming transaction](#sending-and-confirming-transactions)
- [Get a transaction signature](#get-the-signature-from-a-signed-transaction)
- [Get a Solana Explorer link](#get-a-solana-explorer-link-for-transactions-accounts-or-blocks)
- [Calculate minimum rent balance for an account](#calculate-minimum-rent-for-an-account)
- [Generating keypairs and signers](#generating-keypairs-and-signers)
- [Generating extractable keypairs and signers](#generating-extractable-keypairs-and-signers)

<!-- You can also find some [NodeJS specific helpers](#node-specific-imports) like: -->

You can find some system specific helpers here

- [Loading a keypair from a file](#loading-a-keypair-from-a-file)
- [Saving a keypair to a file](#saving-a-keypair-to-a-file)
- [Loading a keypair from an environment variable](#loading-a-keypair-from-an-environment-variable)
- [Saving a keypair to an environment variable file](#saving-a-keypair-to-an-environment-file)
- [Loading a base58 keypair from an environment variable](#loading-a-base58-keypair-from-an-environment-variable)

You can find [transaction builders](#transaction-builders) for common tasks, including:

- [Creating a token with metadata](#create-a-token-with-metadata)
- [Minting tokens to a destination wallet](#mint-tokens-to-a-destination-wallet)
- [Transfer tokens to a destination wallet](#transfer-tokens-to-a-destination-wallet)

For troubleshooting and debugging your Solana transactions, see [Debug mode](#debug-mode) below.

> You can also consult the documentation for Anza's [JavaScript client](https://github.com/anza-xyz/solana-web3.js)
> library for more information and helpful resources.

## Using the modular crates from Anza

## Create a Solana RPC connection

Create a Solana `rpc` client for any RPC URL or standard Solana network moniker (i.e. devnet, localnet, mainnet etc) with a default commitment level of `confirmed`

```rust
use gill::SolanaClient;

let client = SolanaClient::new("devnet");
```

> Using the Solana moniker will connect to the public RPC endpoints. These are subject to rate limits and should not be
> used in production applications. Applications should find their own RPC provider and the URL provided from them.

To create an RPC client with another commitment level:

```rust
use gill::{SolanaClient, CommitmentConfig};

let client = SolanaClient::new_with_commitment("localnet", CommitmentConfig::processed);
```

To create an RPC client for an custom RPC provider or service:

```rust
use gill::SolanaClient;

let client = SolanaClient::new("https://private-solana-rpc-provider.com");
```

## Making Solana RPC calls

After you have a Solana rpc connection, you can make all the [JSON RPC method](https://solana.com/docs/rpc) calls directly off of it.

```rust
use gill::SolanaClient;

let client = SolanaClient::new("devnet");

//get slot
let slot = client.rpc.getSlot();

//get the latest blockhash
let latest_blockhash = client.rpc.get_latest_blockhash();
```

## Create a transaction

### Legacy

Quickly create a Solana transaction:

```rust
use gill::TxBuilder;

let tx = TxBuilder::new(
  fee_payer,
  instructions,
  // the compute budget values are HIGHLY recommend to be set in order to maximize your transaction landing rate
  compute_unit_limit, //optional
  compute_unit_price //optional
  );
```

### Versioned Transaction

Coming soon!

## Signing Transactions

```rust

use gill::TxBuilder;

const tx = TxBuilder::new(...);
const signed_tx = tx.sign(signers, latest_blockhash);
```

## Simulating transactions

Coming soon!

### Sending and confirming transactions

To send and confirm a transaction to the blockchain, you can use the sendAndConfirmTransaction method from the initialized SolanaClient.
The default commitment level is processed, to set another level check out - coming soon!

```rust
use gill::{...};

let client = SolanaClient::new("devnet");
const tx = TxBuilder::new(...);
const signed_tx = tx.sign(...);

let signature = client.rpc.send_and_confirmed_transaction(signed_tx);
```

Set custom config
Coming soon!
