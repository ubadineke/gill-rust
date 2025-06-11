use mpl_token_metadata::{
    accounts::Metadata,
    instructions::{CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs},
    types::DataV2,
};
use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_hash::Hash;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_message::Message;
use solana_program::{program_pack::Pack, system_instruction, system_program,};
use solana_pubkey::Pubkey;
use solana_rent::Rent;
use solana_signer::Signer;
use solana_transaction::Transaction;
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account_idempotent,
};
use spl_token::{
    instruction::{self as token_instruction, mint_to, transfer},
    state::Mint,
};
use url::Url;

pub struct TxBuilder {
    message: Message,
}

pub struct MetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub is_mutable: bool
}

impl TxBuilder {
    pub fn new(
        fee_payer: &Pubkey,
        instructions: &[Instruction],
        compute_unit_limit: Option<u32>,
        compute_unit_price: Option<u64>,
    ) -> Self {
        let mut all_instructions = vec![];

        if let Some(units) = compute_unit_limit {
            all_instructions.push(ComputeBudgetInstruction::set_compute_unit_limit(units));
        }
        if let Some(price) = compute_unit_price {
            all_instructions.push(ComputeBudgetInstruction::set_compute_unit_price(price));
        }

        all_instructions.extend(instructions.iter().cloned());

        let message = Message::new(&all_instructions, Some(fee_payer));

        Self { message }
    }

    pub fn sign(&self, signers: &[&Keypair], latest_blockhash: Hash) -> Transaction {
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

    pub fn create_token_transaction(
        fee_payer: &Keypair,
        latest_blockhash: Hash,
        mint: &Keypair,
        metadata: MetadataArgs,
        decimals: u8,
        token_program: &Pubkey,
    ) -> Transaction {
        //Instructions {
        // Regular Token program
        let instructions = [
            // Create Account
            system_instruction::create_account(
                &fee_payer.pubkey(),
                &mint.pubkey(),
                Rent::default().minimum_balance(Mint::LEN),
                Mint::LEN as u64,
                token_program,
            ),
            // Initialize as mint
            token_instruction::initialize_mint(
                token_program,
                &mint.pubkey(),
                &fee_payer.pubkey(),
                Some(&fee_payer.pubkey()),
                decimals,
            )
            .unwrap(),
            //Create Metadata instruction from metaplex
            CreateMetadataAccountV3 {
                metadata: Metadata::find_pda(&mint.pubkey()).0,
                mint: mint.pubkey(),
                mint_authority: fee_payer.pubkey(),
                payer: fee_payer.pubkey(),
                update_authority: (fee_payer.pubkey(), true),
                system_program: system_program::ID,
                rent: Some(solana_program::sysvar::rent::ID),
            }
            .instruction(CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name: metadata.name,
                    symbol: metadata.symbol,
                    uri: metadata.uri,
                    collection: None,
                    creators: None,
                    seller_fee_basis_points: 0,
                    uses: None,
                },
                is_mutable: metadata.is_mutable,
                collection_details: None,
            }),
        ];

        let message = Message::new(&instructions, Some(&fee_payer.pubkey()));
        Transaction::new(&[fee_payer, mint], message, latest_blockhash)
        // }
    }

    pub fn mint_token_transaction(
        fee_payer: &Keypair,
        latest_blockhash: Hash,
        mint: &Keypair,
        mint_authority: &Keypair,
        amount: u64,
        destination: &Pubkey,
        token_program: &Pubkey,
    ) -> Transaction {
        let ata = get_associated_token_address(destination, &mint.pubkey());
        let instructions = [
            //instructions
            // create idempotent will gracefully fail if the ata already exists. this is the gold standard!
            create_associated_token_account_idempotent(
                &fee_payer.pubkey(),
                destination,
                &mint.pubkey(),
                token_program,
            ),
            //mint to account
            mint_to(
                token_program,
                &mint.pubkey(),
                &ata,
                &mint_authority.pubkey(),
                &[],
                amount,
            )
            .unwrap(),
        ];

        let message = Message::new(&instructions, Some(&fee_payer.pubkey()));
        Transaction::new(&[fee_payer, mint_authority], message, latest_blockhash)
    }

    pub fn transfer_token_transaction(
        fee_payer: &Keypair,
        latest_blockhash: Hash,
        mint: &Keypair,
        authority: &Keypair,
        amount: u64,
        destination: &Pubkey,
        token_program: &Pubkey,
    ) -> Transaction {
        let source_ata = get_associated_token_address(&authority.pubkey(), &mint.pubkey());
        let destination_ata = get_associated_token_address(destination, &mint.pubkey());
        let instructions = [
            //instructions
            // create idempotent will gracefully fail if the ata already exists. this is the gold standard!
            create_associated_token_account_idempotent(
                &fee_payer.pubkey(),
                destination,
                &mint.pubkey(),
                token_program,
            ),
            transfer(
                token_program,
                &source_ata,
                &destination_ata,
                &authority.pubkey(),
                &[&authority.pubkey()],
                amount,
            )
            .unwrap(),
        ];
        let message = Message::new(&instructions, Some(&fee_payer.pubkey()));
        Transaction::new(&[fee_payer, authority], message, latest_blockhash)
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
