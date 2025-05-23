use solana_client::rpc_client::RpcClient;
use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_hash::Hash;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_message::Message;
use solana_pubkey::Pubkey;
use solana_transaction::Transaction;

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
