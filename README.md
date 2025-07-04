# Implementation of the [gill](https://github.com/solana-foundation/gill/) library in Rust

## Installation

```bash
cargo add branchia
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
<!-- - [Generating extractable keypairs and signers](#generating-extractable-keypairs-and-signers) -->

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

### Using the modular crates from Anza

## Generating Keypairs and Signers

For most "signing" transactions, you'll need a Keypair instance, which can be used to sign transactions and messages.

```rust
use branchia::{Keypair, Signer};

let signer = Keypair::new();
```

> Ensure you import the `Signer` trait as it contains necessary methods to facilitate signing

## Create a Solana RPC connection

Create a Solana `rpc` client for any RPC URL or standard Solana network moniker (i.e. devnet, localnet, mainnet etc) with a default commitment level of `confirmed`

```rust
use branchia::SolanaClient;

let client = SolanaClient::new("devnet");
```

> Using the Solana moniker will connect to the public RPC endpoints. These are subject to rate limits and should not be
> used in production applications. Applications should find their own RPC provider and the URL provided from them.

To create an RPC client with another commitment level:

```rust
use branchia::{SolanaClient, CommitmentConfig};

let client = SolanaClient::new_with_commitment("localnet", CommitmentConfig::processed);
```

To create an RPC client for an custom RPC provider or service:

```rust
use branchia::SolanaClient;

let client = SolanaClient::new("https://private-solana-rpc-provider.com");
```

## Making Solana RPC calls

After you have a Solana rpc connection, you can make all the [JSON RPC method](https://solana.com/docs/rpc) calls directly off of it.

```rust
use branchia::SolanaClient;

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
use branchia::TxBuilder;

let tx = TxBuilder::new(
  fee_payer,
  instructions,
  // the compute budget values are HIGHLY recommend to be set in order to maximize your transaction landing rate
  compute_unit_limit, //optional
  compute_unit_price //optional
  );
```

### Versioned Transaction

**COMING SOON!**

## Signing Transactions

```rust

use branchia::TxBuilder;

let tx = TxBuilder::new(...);
let signed_tx = tx.sign(signers, latest_blockhash);
```

## Simulating transactions

To simulate a transaction on the blockchain, you can use the `simulate_transaction` method on the initialized `SolanaClient`

```rust
use branchia::{...};

let client = SolanaClient::new("devnet");
let tx = TxBuilder::new(...);
let signed_tx = tx.sign(...);

let simulation = client.rpc.simulate_transaction(signed_tx);
```

Simulation with Custom Config: **COMING SOON!**

## Sending and confirming transactions

To send and confirm a transaction to the blockchain, you can use the sendAndConfirmTransaction method from the initialized SolanaClient.
The default commitment level is processed, to set another level check out - **COMING SOON!**

```rust
use branchia::{...};

let client = SolanaClient::new("devnet");
let tx = TxBuilder::new(...);
let signed_tx = tx.sign(...);

let signature = client.rpc.send_and_confirmed_transaction(signed_tx);
```

Set custom config

**COMING SOON!**

## Get the signature from a signed transaction

After you have a transaction signed by the `feePayer` (either a partially or fully signed transaction), you can get the
transaction signature as follows:

```rust
use branchia::TxBuilder;

let tx = TxBuilder::new(...);
let signed_tx = tx.sign(...);

let signature = signed_tx.signatures[0];
```

> Note: After a transaction has been signed by the fee payer, it will have a transaction signature (aka transaction id).
> This is due to Solana transaction ids are the first item in the transaction's `signatures` array. Therefore, client
> applications can potentially know the signature before it is even sent to the network for confirmation.

## Get a Solana Explorer link for transactions, accounts, or blocks

Craft a Solana Explorer link for transactions, accounts, or blocks on any cluster.

### Get a Solana Explorer link for a transaction

To get an explorer link for a transaction's signature (aka transaction id):

```rust
use branchia::TxBuilder;

let url = TxBuilder::get_explorer_link_transaction(cluster, signature);
```

> The `cluster` parameter accepts custom urls and the known monikers(i.e devnet, mainnet, localnet)

If you have a partially or fully signed transaction, you can get the Explorer link before even sending the transaction to the network:

> See [`Get Signature from a Signed Transaction`](#get-the-signature-from-a-signed-transaction) for better understanding.

```rust
use branchia::TxBuilder;

let tx = TxBuilder::new(...);
let signed_tx = tx.sign(...);
let url = TxBuilder::get_explorer_link_transaction("localnet", &signed_tx.signatures[0].to_string())
```

**COMING SOON!**

### Get a Solana Explorer link for an account

To get an explorer link for an account on Solana's devnet:

```rust
use branchia::TxBuilder;

let url = TxBuilder::get_explorer_link_account("devnet", "Bf8PxxWt7UTvNGcrDyNwQiERSwNroa4pEo1pxwKo17Uh");
```

### Get a Solana Explorer link for a block

To get an explorer link for a block:

```rust
use branchia::TxBuilder;

let url = TxBuilder::get_explorer_link_block("localnet", 2345);
```

## Calculate minimum rent for an account

To calculate the minimum rent balance for an account (aka data storage deposit fee):

#### For default rent

```rust
use branchia::Rent;

let rent = Rent::default().minimum_balance(0);
// Expected value: 890880
```

#### Rent for specified number of bytes

```rust
use branchia::Rent;

let rent = Rent::default().minimum_balance(50);
// Expected value: 1238880
```

## Loading a keypair from a file

```rust
use branchia::{Keypair, KeypairExt};

let signer = Keypair::from_default_file();
```

> Ensure you import the `KeypairExt` trait as it contains extended functionality on the Keypair component

Load a `Keypair` from a filesystem wallet json file, like those output from the [Solana CLI](https://solana.com/docs/intro/installation#install-the-solana-cli) (i.e. a JSON array of numbers).

By default, the keypair file loaded is the Solana CLI's default keypair: ~/.config/solana/id.json

To load a Signer from a specific filepath:

```rust
use branchia::Keypair;

let signer = Keypair::from_default_file(path);
```

## Saving a keypair to a file

Save a Keypair to a local json file(e.g keypair.json)

```rust
use branchia::Keypair;

let signer = Keypair::new();
signer.write_to_file(path);
```

## Transaction Builders

### Create a token with metadata

Build a transaction that can create a token with metadata, either using the
[original token](https://github.com/solana-program/token) or
[token extensions (token22)](https://github.com/solana-program/token-2022) program. - **COMING SOON!**

- Tokens created with the original token program (`TOKEN_PROGRAM_ADDRESS`, default) will use Metaplex's Token Metadata
  program for onchain metadata
- Tokens created with the token extensions program (`TOKEN_2022_PROGRAM_ADDRESS`) will use the metadata pointer
  extensions - **COMING SOON!**
  INSTRUCTION BUILDERS - **COMING SOON!**

```rust
use branchia:{TxBuilder, MetadataArgs};

let create_token_tx = TxBuilder::create_token_transaction(
  fee_payer,
  latest_blockhash,
  mint,
  MetadataArgs{
    name
    symbol
    uri
    is_mutable // if the `updateAuthority` can change this metadata in the future
  },
  decimals,
  tokenProgram //just the regular token program, no token 2022 support yet
)

// ADD DEFAULT VALUES FOR CERTAIN FIELDS HERE? - TBD!

```

### Mint tokens to destination wallet

Build a transaction that mints new tokens to the `destination` wallet address (raising the token's overall supply).

- ensure you set the correct `tokenProgram` used by the `mint` itself
- if the `destination` owner does not have an associated token account (ata) created for the `mint`, one will be
  auto-created for them
- ensure you take into account the `decimals` for the `mint` when setting the `amount` in this transaction

```rust
use branchia::TxBuilder;

let mint_tokens_tx = TxBuilder::mint_token_transaction(
  fee_payer,
  latest_blockhash,
  mint,
  mint_authority,
  amount,// note: be sure to consider the mint's `decimals` value
  // if decimals=2 => this will mint 10.00 tokens
  // if decimals=4 => this will mint 0.100 tokens
  destination, // use the correct token program for the `mint`
  token_program //just the regular token program, no token 2022 support yet
)

```

### Transfer tokens to a destination wallet

Build a transaction that transfers tokens to the `destination` wallet address from the `source` (aka from `sourceAta` to
`destinationAta`).

- ensure you set the correct `tokenProgram` used by the `mint` itself
- if the `destination` owner does not have an associated token account (ata) created for the `mint`, one will be
  auto-created for them
- ensure you take into account the `decimals` for the `mint` when setting the `amount` in this transaction

```rust
use branchia::{TxBuilder}

let transfer_tokens_tx = TxBuilder::transfer_token_transaction(
  fee_payer,
  latest_blockhash,
  mint,
  authority,
  amount: 900, // note: be sure to consider the mint's `decimals` value
  // if decimals=2 => this will transfer 9.00 tokens
  // if decimals=4 => this will transfer 0.090 tokens
  destination,   // use the correct token program for the `mint`
  token_program
)

```
