pub mod create_solana_client;
pub mod rpc;

pub use rpc::Rpc;
pub use create_solana_client::SolanaClient;

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
