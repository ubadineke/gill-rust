use std::fmt;
use solana_commitment_config::CommitmentConfig;

// use solana_client::rpc_client::RpcClient;
use crate::rpc::Rpc;
pub struct SolanaClient{
  pub rpc: Rpc
}

//Commitment Level 
// Processed,
// Confirmed,
// Finalized,

enum CommitmentType{ 
  Processed,
  Confirmed,
  Finalized,
}

//usage
//const client = SolanaClient::new('devnet') //mainnet, testnet, localurl
//const client = SolanaClient::new_with_commitment('devnet', 'confirmed')

impl SolanaClient{
  pub fn new(moniker_or_url: &str) -> Self {
    let url = resolve_url(moniker_or_url);
    Self::new_with_commitment(url, "confirmed")

  }

  pub fn new_with_commitment(moniker_or_url: &str, commitment_level: &str) -> Self{
    let url = resolve_url(moniker_or_url);
    let commitment = resolve_commitment(commitment_level);
    let rpc = Rpc::new_with_commitment(url, commitment);
    Self {rpc}
  }
}

// impl fmt::Debug for SolanaClient {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//       f.debug_struct("SolanaClient")
//        .field("rpc", &self.rpc) // now prints the real debug of rpc
//        .finish()
//   }
// }

fn resolve_url(moniker_or_url: &str) -> &str {
  match moniker_or_url {
      "mainnet" | "mainnet-beta" => "https://api.mainnet-beta.solana.com",
      "testnet" => "https://api.testnet.solana.com",
      "devnet" => "https://api.devnet.solana.com",
      "localnet" | "localhost" => "http://localhost:8899",
      _ => moniker_or_url, // treat as raw custom URL
  }
}

fn resolve_commitment(level: &str) -> CommitmentConfig {
  match level {
      "processed" => CommitmentConfig::processed(),
      "confirmed" => CommitmentConfig::confirmed(),
      "finalized" => CommitmentConfig::finalized(),
      _ => CommitmentConfig::confirmed(), // fallback/default
  }
}
