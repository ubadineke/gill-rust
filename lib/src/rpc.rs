use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use std::{fmt, ops::Deref};

pub struct Rpc {
    pub client: RpcClient,
}

impl Deref for Rpc {
    type Target = RpcClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl Rpc {
    pub fn new(url: &str) -> Self {
        Self::new_with_commitment(url, CommitmentConfig::confirmed())
    }

    pub fn new_with_commitment(url: &str, commitment: CommitmentConfig) -> Self {
        let client = RpcClient::new_with_commitment(url.to_string(), commitment);
        Self { client }
    }
}

// impl fmt::Debug for Rpc{
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//       f.debug_struct("SolanaClient")
//        .field("rpc", &self.client) // now prints the real debug of rpc
//        .finish()
//   }
// }
