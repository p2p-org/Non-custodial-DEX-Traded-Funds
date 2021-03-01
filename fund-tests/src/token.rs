use solana_program::{program_pack::Pack, pubkey::Pubkey, system_instruction};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_token::state::{Account as TokenAccount, Mint};

use super::client::Client;

pub fn create_token(client: &mut Client, owner: &Pubkey, decimals: u8) -> Keypair {
    let token_mint = Keypair::new();

    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &client.payer_pubkey(),
                &token_mint.pubkey(),
                client.rent_minimum_balance(Mint::LEN),
                Mint::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_mint(&spl_token::id(), &token_mint.pubkey(), owner, None, decimals)
                .unwrap(),
        ],
        Some(&client.payer_pubkey()),
    );
    transaction.sign(&[client.payer(), &token_mint], client.recent_blockhash());
    client.process_transaction(&transaction);
    token_mint
}

pub fn create_account(client: &mut Client, owner: &Pubkey, token_mint: &Pubkey) -> Keypair {
    let token_account = Keypair::new();

    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &client.payer_pubkey(),
                &token_account.pubkey(),
                client.rent_minimum_balance(TokenAccount::LEN),
                TokenAccount::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_account(&spl_token::id(), &token_account.pubkey(), token_mint, owner)
                .unwrap(),
        ],
        Some(&client.payer_pubkey()),
    );
    transaction.sign(&[client.payer(), &token_account], client.recent_blockhash());
    client.process_transaction(&transaction);
    token_account
}

pub fn mint_to(
    client: &mut Client,
    owner: &Keypair,
    token_mint: &Pubkey,
    account: &Pubkey,
    amount: u64,
    decimals: u8,
) {
    let mut transaction = Transaction::new_with_payer(
        &[spl_token::instruction::mint_to_checked(
            &spl_token::id(),
            token_mint,
            account,
            &owner.pubkey(),
            &[],
            amount,
            decimals,
        )
        .unwrap()],
        Some(&client.payer_pubkey()),
    );
    transaction.sign(&[client.payer(), &owner], client.recent_blockhash());
    client.process_transaction(&transaction);
}

pub fn transfer_to(
    client: &mut Client,
    owner: &Keypair,
    token_mint: &Pubkey,
    source: &Pubkey,
    destination: &Pubkey,
    amount: u64,
    decimals: u8,
) {
    let mut transaction = Transaction::new_with_payer(
        &[spl_token::instruction::transfer_checked(
            &spl_token::id(),
            source,
            token_mint,
            destination,
            &owner.pubkey(),
            &[],
            amount,
            decimals,
        )
        .unwrap()],
        Some(&client.payer_pubkey()),
    );
    transaction.sign(&[client.payer(), &owner], client.recent_blockhash());
    client.process_transaction(&transaction);
}
