use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_hash::Hash;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_message::Message;
use solana_pubkey::Pubkey;
use solana_signature::Signature;
use solana_transaction::Transaction;
use url::Url;


pub struct TxBuilder{
    message: Message
}

impl TxBuilder{
    pub fn new( 
        fee_payer: &Pubkey,
        instructions: &[Instruction],
        compute_unit_limit: Option<u32>,
        compute_unit_price: Option<u64>,
    ) -> Self{
        let mut all_instructions = vec![];

        if let Some(units) = compute_unit_limit {
            all_instructions.push(ComputeBudgetInstruction::set_compute_unit_limit(units));
        }
        if let Some(price) = compute_unit_price {
            all_instructions.push(ComputeBudgetInstruction::set_compute_unit_price(price));
        }
    
        all_instructions.extend(instructions.iter().cloned());
    
        let message = Message::new(&all_instructions, Some(fee_payer));

        Self{ message }
    }

    pub fn sign(&self, signers: &[&Keypair], latest_blockhash: Hash) -> Transaction{
        Transaction::new(signers, self.message.clone(), latest_blockhash) 
    }


    pub fn get_explorer_link_account(cluster: &str, address: &str) -> String {
        Self::build_url(cluster, &format!("/address/{}", address))
    }

    pub fn get_explorer_link_transaction(cluster: &str, signature: &str) -> String {
        Self::build_url(cluster, &format!("/tx/{}", signature))
    }

    pub fn get_explorer_link_block(cluster: &str, slot: u64) -> String {
        Self::build_url(cluster, &format!("/block/{}", slot))
    }

    fn build_url(cluster: &str, path: &str) -> String {
        let mut url = Url::parse("https://explorer.solana.com").unwrap();
        url.set_path(path);

        match cluster {
            "mainnet" | "mainnet-beta" => {
                // Do nothing, default explorer behavior
            }
            "devnet" | "testnet" => {
                url.query_pairs_mut().append_pair("cluster", cluster);
            }
            "localhost" | "localnet" => {
                url.query_pairs_mut()
                    .append_pair("cluster", "custom")
                    .append_pair("customUrl", "http://localhost:8899");
            }
            custom_url => {
                // Treat as a full custom RPC URL
                url.query_pairs_mut()
                    .append_pair("cluster", "custom")
                    .append_pair("customUrl", custom_url);
            }
        }

        url.to_string()
    }
}

pub fn create_transaction(
    fee_payer: &Pubkey,
    instructions: &[Instruction],
    compute_unit_limit: Option<u32>,
    compute_unit_price: Option<u64>,
) -> Message {
    let mut all_instructions = vec![];

    if let Some(units) = compute_unit_limit {
        all_instructions.push(ComputeBudgetInstruction::set_compute_unit_limit(units));
    }
    if let Some(price) = compute_unit_price {
        all_instructions.push(ComputeBudgetInstruction::set_compute_unit_price(price));
    }

    all_instructions.extend(instructions.iter().cloned());

    Message::new(&all_instructions, Some(fee_payer))
}

pub fn sign_transaction(
    signers: &[&Keypair],
    message: Message,
    latest_blockhash: Hash,
) -> Transaction {
    Transaction::new(signers, message, latest_blockhash)
}
