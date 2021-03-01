use std::{env, str::FromStr};

use anyhow::{anyhow, Result};
use dotenv::dotenv;
use fund_tests::{client::Client, print::Print, token};
use solana_client::rpc_client::RpcClient;
use solana_program::{program_pack::Pack, pubkey::Pubkey, system_program};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Keypair, Signer},
};
use spl_token_swap::curve::fees::Fees;

fn main() -> Result<()> {
    dotenv().ok();

    let payer_account =
        read_keypair_file(env::var("PAYER_KEYPAIR_FILE")?).map_err(|err| anyhow!("Read keypair error: {}", err))?;

    let rpc_client = RpcClient::new_with_commitment(env::var("RPC_URL")?, CommitmentConfig::confirmed());
    let mut client = Client {
        client: rpc_client,
        payer: payer_account,
    };

    let initializer_account = if let Ok(base58) = env::var("initializer_account") {
        Keypair::from_base58_string(&base58)
    } else {
        client
            .create_account(&system_program::id(), 0)
            .print_in_place("initializer_account")
    };

    // Create assets
    // SOL, FTT, REN, SRM, SUSHI, RAY, FIDA, USDC

    let sol_token_mint = if let Ok(base58) = env::var("sol_token_mint") {
        Keypair::from_base58_string(&base58)
    } else {
        token::create_token(&mut client, &initializer_account.pubkey(), 6).print_in_place("sol_token_mint")
    };

    let ftt_token_mint = if let Ok(base58) = env::var("ftt_token_mint") {
        Keypair::from_base58_string(&base58)
    } else {
        token::create_token(&mut client, &initializer_account.pubkey(), 6).print_in_place("ftt_token_mint")
    };

    let ren_token_mint = if let Ok(base58) = env::var("ren_token_mint") {
        Keypair::from_base58_string(&base58)
    } else {
        token::create_token(&mut client, &initializer_account.pubkey(), 6).print_in_place("ren_token_mint")
    };

    let srm_token_mint = if let Ok(base58) = env::var("srm_token_mint") {
        Keypair::from_base58_string(&base58)
    } else {
        token::create_token(&mut client, &initializer_account.pubkey(), 6).print_in_place("srm_token_mint")
    };

    let sushi_token_mint = if let Ok(base58) = env::var("sushi_token_mint") {
        Keypair::from_base58_string(&base58)
    } else {
        token::create_token(&mut client, &initializer_account.pubkey(), 6).print_in_place("sushi_token_mint")
    };

    let ray_token_mint = if let Ok(base58) = env::var("ray_token_mint") {
        Keypair::from_base58_string(&base58)
    } else {
        token::create_token(&mut client, &initializer_account.pubkey(), 6).print_in_place("ray_token_mint")
    };

    let fida_token_mint = if let Ok(base58) = env::var("fida_token_mint") {
        Keypair::from_base58_string(&base58)
    } else {
        token::create_token(&mut client, &initializer_account.pubkey(), 6).print_in_place("fida_token_mint")
    };

    let usdc_token_mint = if let Ok(base58) = env::var("usdc_token_mint") {
        Keypair::from_base58_string(&base58)
    } else {
        token::create_token(&mut client, &initializer_account.pubkey(), 6).print_in_place("usdc_token_mint")
    };

    // Mint to initializer

    let initializer_sol_token_account = mint_to(
        &initializer_account,
        "initializer_sol_token_account",
        &sol_token_mint.pubkey(),
        &mut client,
    );

    let initializer_ftt_token_account = mint_to(
        &initializer_account,
        "initializer_ftt_token_account",
        &ftt_token_mint.pubkey(),
        &mut client,
    );

    let initializer_ren_token_account = mint_to(
        &initializer_account,
        "initializer_ren_token_account",
        &ren_token_mint.pubkey(),
        &mut client,
    );

    let initializer_srm_token_account = mint_to(
        &initializer_account,
        "initializer_srm_token_account",
        &srm_token_mint.pubkey(),
        &mut client,
    );

    let initializer_sushi_token_account = mint_to(
        &initializer_account,
        "initializer_sushi_token_account",
        &sushi_token_mint.pubkey(),
        &mut client,
    );

    let initializer_ray_token_account = mint_to(
        &initializer_account,
        "initializer_ray_token_account",
        &ray_token_mint.pubkey(),
        &mut client,
    );

    let initializer_fida_token_account = mint_to(
        &initializer_account,
        "initializer_fida_token_account",
        &fida_token_mint.pubkey(),
        &mut client,
    );

    let initializer_usdc_token_account = mint_to(
        &initializer_account,
        "initializer_usdc_token_account",
        &usdc_token_mint.pubkey(),
        &mut client,
    );

    let fees = Fees {
        trade_fee_numerator: 1,
        trade_fee_denominator: 1000,
        owner_trade_fee_numerator: 1,
        owner_trade_fee_denominator: 1000,
        owner_withdraw_fee_numerator: 1,
        owner_withdraw_fee_denominator: 1000,
        host_fee_numerator: 1,
        host_fee_denominator: 1000,
    };

    let swap_program_id = if let Ok(key) = env::var("SWAP_PROGRAM_ID") {
        Pubkey::from_str(&key).unwrap()
    } else {
        spl_token_swap::id()
    };

    let sol_usdc_swap = client.create_account(&swap_program_id, spl_token_swap::state::SwapV1::LEN);
    let (sol_usdc_swap_authority, sol_usdc_swap_authority_nonce) = if let (Ok(authority), Ok(authority_nonce)) = (
        env::var("sol_usdc_swap_authority"),
        env::var("sol_usdc_swap_authority_nonce"),
    ) {
        (Pubkey::from_str(&authority)?, authority_nonce.parse()?)
    } else {
        Pubkey::find_program_address(&[sol_usdc_swap.pubkey().as_ref()], &swap_program_id)
    };
    let swap_token_sol = token::create_account(&mut client, &sol_usdc_swap_authority, &sol_token_mint.pubkey());
    let swap_token_usdc = token::create_account(&mut client, &sol_usdc_swap_authority, &usdc_token_mint.pubkey());

    let sol_usdc_swap = token::create_swap(
        &mut client,
        &swap_program_id,
        &sol_usdc_swap,
        &sol_usdc_swap_authority,
        sol_usdc_swap_authority_nonce,
        &sol_token_mint.pubkey(),
        &usdc_token_mint.pubkey(),
        &initializer_account.pubkey(),
        fees.clone(),
    )
    .print("sol_usdc_swap");

    Ok(())
}

fn mint_to(account: &Keypair, token_account_name: &str, token_mint: &Pubkey, client: &mut Client) -> Keypair {
    if let Ok(base58) = env::var(token_account_name) {
        Keypair::from_base58_string(&base58)
    } else {
        let token_account =
            token::create_account(client, &account.pubkey(), token_mint).print_in_place(token_account_name);
        token::mint_to(client, &account, token_mint, &token_account.pubkey(), 10_000_000000, 6);
        token_account
    }
}
