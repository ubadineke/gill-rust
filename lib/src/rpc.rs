use anyhow::Error;
use solana_client::{
    client_error::ClientError,
    pubsub_client::{
        AccountSubscription, BlockSubscription, LogsSubscription, PubsubClient, PubsubClientError,
    },
    rpc_client::RpcClient,
    rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter, RpcTransactionLogsFilter},
};
use solana_commitment_config::CommitmentConfig;
use solana_pubkey::Pubkey;
use solana_signature::Signature;
use solana_transaction::Transaction;
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

    pub fn send_and_confirm_transaction(
        &self,
        transaction: Transaction,
    ) -> Result<Signature, ClientError> {
        RpcClient::send_and_confirm_transaction(&self, &transaction)
    }
}

pub struct RpcSubscriptions {
    // pub client: PubsubClient
    pub url: String,
}

//Using None on RPC Configs for now
impl RpcSubscriptions {
    pub fn new(url: &str) -> Self {
        let ws_url = url.replace("https", "wss").replace("http", "ws");
        Self { url: ws_url }
    }

    //account
    pub fn account(&self, pubkey: Pubkey) -> Result<AccountSubscription, PubsubClientError> {
        PubsubClient::account_subscribe(self.url.as_str(), &pubkey, None)
    }

    //block
    pub fn block(
        &self,
        filter: RpcBlockSubscribeFilter,
    ) -> Result<BlockSubscription, PubsubClientError> {
        PubsubClient::block_subscribe(self.url.as_str(), filter, None)
    }

    //logs
    // pub fn logs(&self, filter: RpcTransactionLogsFilter) -> Result<LogsSubscription, PubsubClientError> {
    //     PubsubClient::logs_subscribe(self.url.as_str(), filter, None)
    // }

    //program

    //vote

    //root

    //signature

    //slot

    //slot_updates
}

// impl fmt::Debug for Rpc{
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//       f.debug_struct("SolanaClient")
//        .field("rpc", &self.client) // now prints the real debug of rpc
//        .finish()
//   }
// }
