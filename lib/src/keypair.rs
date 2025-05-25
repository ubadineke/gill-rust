use solana_keypair::Keypair;
use solana_signer::EncodableKey;
use std::{fs::File, io::Read, path::Path};

pub trait KeypairExt {
  fn from_default_file() -> Result<Self, Box<dyn std::error::Error>>
  where
      Self: Sized;
}

impl KeypairExt for Keypair {
  fn from_default_file() -> Result<Self, Box<dyn std::error::Error>> {
      let default_path = dirs::home_dir()
          .ok_or("Could not find home directory")?
          .join(".config/solana/id.json");
      let mut file = File::open(&default_path)?;
      Ok(Keypair::read(&mut file)?)
  }
}