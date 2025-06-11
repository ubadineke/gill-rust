pub mod keypair;
pub mod rpc;
pub mod solana_client;
pub mod transaction;
pub mod utils;

pub use keypair::KeypairExt;
pub use rpc::Rpc;
pub use solana_client::SolanaClient;
pub use solana_commitment_config::CommitmentConfig;
pub use solana_keypair::Keypair;
pub use solana_rent::Rent;
pub use solana_signer::Signer;
pub use spl_token::ID as TOKEN_PROGRAM_ADDRESS;
pub use spl_token_2022::ID as TOKEN_2022_PROGRAM_ADDRESS;
pub use transaction::{TxBuilder, MetadataArgs};

pub fn testing() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_this() {
        testing(); // This will execute when running `cargo test`
    }
}
