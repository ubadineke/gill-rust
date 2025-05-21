pub mod create_solana_client;
pub mod rpc;
pub mod transaction;

pub use create_solana_client::SolanaClient;
pub use rpc::Rpc;

pub use solana_commitment_config::CommitmentConfig;

pub fn testing() {
    let client = SolanaClient::new("http:://localhost:3040");
    println!("LEt's play");
    let blockhash = client.rpc.get_latest_blockhash().unwrap();
    println!("{:?}", blockhash);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_this() {
        testing(); // This will execute when running `cargo test`
    }
}
