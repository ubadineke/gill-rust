use crate::rpc::{Rpc, RpcSubscriptions};
use crate::utils::resolve_url;
use anyhow::{Context, Result};
use solana_commitment_config::CommitmentConfig;
use std::fmt;
pub struct SolanaClient {
    pub rpc: Rpc,
    pub rpc_subscriptions: RpcSubscriptions,
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
        let rpc_subscriptions = RpcSubscriptions::new(url);
        Self {
            rpc,
            rpc_subscriptions,
        }
    }
}

// impl fmt::Debug for SolanaClient {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//       f.debug_struct("SolanaClient")
//        .field("rpc", &self.rpc) // now prints the real debug of rpc
//        .finish()
//   }
// }
