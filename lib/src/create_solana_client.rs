use crate::rpc::Rpc;
use anyhow::{Context, Result};
use solana_commitment_config::CommitmentConfig;
use std::fmt;
pub struct SolanaClient {
    pub rpc: Rpc,
}

//usage
//const client = SolanaClient::new('devnet') //mainnet, testnet, localurl
// let commitment = CommitmentConfig::proccessed()
//
//const client = SolanaClient::new_with_commitment('mainnet', CommitmentConfig::processed())
//const client = SolanaClient::new_with_commitment('devnet', 'confirmed') --- tbd!
//

impl SolanaClient {
    pub fn new(moniker_or_url: &str) -> Self {
        let url = resolve_url(moniker_or_url);
        Self::new_with_commitment(url, CommitmentConfig::confirmed())
    }

    pub fn new_with_commitment(moniker_or_url: &str, commitment_level: CommitmentConfig) -> Self {
        let url = resolve_url(moniker_or_url);
        let rpc = Rpc::new_with_commitment(url, commitment_level);
        Self { rpc }
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

// fn resolve_commitment(level: &str) -> Result<CommitmentConfig> {
//   match level {
//       "processed" => Ok(CommitmentConfig::processed()),
//       "confirmed" => Ok(CommitmentConfig::confirmed()),
//       "finalized" => Ok(CommitmentConfig::finalized()),
//       _ => Err(anyhow::anyhow!(
//         "Invalid commitment level '{}'. Use: processed, confirmed, finalized.",
//         level
//     )),
//   }
// }
