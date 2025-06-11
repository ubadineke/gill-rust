pub fn resolve_url(moniker_or_url: &str) -> &str {
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
