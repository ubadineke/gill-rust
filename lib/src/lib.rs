pub mod solana_client;
pub mod rpc;
pub mod transaction;
pub mod keypair;
pub mod utils;

pub use solana_client::SolanaClient;
pub use rpc::Rpc;
pub use transaction::TxBuilder;
pub use solana_keypair::Keypair;
pub use solana_signer::Signer;
pub use keypair::KeypairExt;
pub use solana_commitment_config::CommitmentConfig;
pub use solana_rent::Rent;

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
