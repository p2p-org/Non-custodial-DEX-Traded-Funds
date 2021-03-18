use std::{env, str::FromStr};

pub use solana_client_helpers::token::*;
use solana_program::{program_pack::Pack, pubkey::Pubkey, system_instruction};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_token::state::Account as TokenAccount;
use spl_token_swap::curve::{base::SwapCurve, fees::Fees};

use super::client::Client;

pub fn create_swap(
    client: &mut Client,
    swap_program_id: &Pubkey,
    token_swap: &Keypair,
    swap_authority: &Pubkey,
    swap_authority_nonce: u8,
    pool_token_mint: &Pubkey,
    token_a: &Pubkey,
    token_b: &Pubkey,
    owner: &Pubkey,
    fees: Fees,
) -> (Keypair, Keypair) {
    let fee = Keypair::new();
    let fee_owner = Pubkey::from_str(&env::var("SWAP_PROGRAM_OWNER_FEE_ADDRESS").unwrap()).unwrap();
    let pool_token_initial = Keypair::new();

    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &client.payer_pubkey(),
                &fee.pubkey(),
                client.rent_minimum_balance(TokenAccount::LEN),
                TokenAccount::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_account(&spl_token::id(), &fee.pubkey(), pool_token_mint, &fee_owner)
                .unwrap(),
            system_instruction::create_account(
                &client.payer_pubkey(),
                &pool_token_initial.pubkey(),
                client.rent_minimum_balance(TokenAccount::LEN),
                TokenAccount::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_account(
                &spl_token::id(),
                &pool_token_initial.pubkey(),
                pool_token_mint,
                owner,
            )
            .unwrap(),
            spl_token_swap::instruction::initialize(
                swap_program_id,
                &spl_token::id(),
                &token_swap.pubkey(),
                swap_authority,
                token_a,
                token_b,
                pool_token_mint,
                &fee.pubkey(),
                &pool_token_initial.pubkey(),
                swap_authority_nonce,
                fees,
                SwapCurve::default(),
            )
            .unwrap(),
        ],
        Some(&client.payer_pubkey()),
    );
    transaction.sign(
        &vec![client.payer(), token_swap, &fee, &pool_token_initial],
        client.recent_blockhash(),
    );
    client.process_transaction(&transaction);

    (fee, pool_token_initial)
}
