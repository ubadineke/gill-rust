pub mod solana_client;
pub mod rpc;
pub mod transaction;

pub use solana_client::SolanaClient;
pub use rpc::Rpc;
pub use transaction::TxBuilder;
pub use solana_keypair::Keypair;
pub use solana_signer::Signer;

pub use solana_commitment_config::CommitmentConfig;

pub fn testing() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_this() {
        testing(); // This will execute when running `cargo test`
    }
}
