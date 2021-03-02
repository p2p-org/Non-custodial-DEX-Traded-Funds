use std::{env, str::FromStr};

use anyhow::{anyhow, Result};
use dotenv::dotenv;
use fund_tests::{client::Client, print::Print, token};
use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey::Pubkey, system_program};
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

    let sol_usdc_swap_data = create_swap(
        &mut client,
        &swap_program_id,
        &initializer_account,
        "sol_usdc_swap",
        "sol",
        &sol_token_mint.pubkey(),
        1_000_000_000000,
        "usdc",
        &usdc_token_mint.pubkey(),
        15_250_000_000000,
        fees.clone(),
    )?;

    let ftt_usdc_swap_data = create_swap(
        &mut client,
        &swap_program_id,
        &initializer_account,
        "ftt_usdc_swap",
        "ftt",
        &ftt_token_mint.pubkey(),
        1_000_000_000000,
        "usdc",
        &usdc_token_mint.pubkey(),
        29_930_000_000000,
        fees.clone(),
    )?;

    let ren_usdc_swap_data = create_swap(
        &mut client,
        &swap_program_id,
        &initializer_account,
        "ren_usdc_swap",
        "ren",
        &ren_token_mint.pubkey(),
        1_000_000_000000,
        "usdc",
        &usdc_token_mint.pubkey(),
        1_170_000_000000,
        fees.clone(),
    )?;

    let srm_usdc_swap_data = create_swap(
        &mut client,
        &swap_program_id,
        &initializer_account,
        "srm_usdc_swap",
        "srm",
        &srm_token_mint.pubkey(),
        1_000_000_000000,
        "usdc",
        &usdc_token_mint.pubkey(),
        5_750_000_000000,
        fees.clone(),
    )?;

    let sushi_usdc_swap_data = create_swap(
        &mut client,
        &swap_program_id,
        &initializer_account,
        "sushi_usdc_swap",
        "sushi",
        &sushi_token_mint.pubkey(),
        1_000_000_000000,
        "usdc",
        &usdc_token_mint.pubkey(),
        17_010_000_000000,
        fees.clone(),
    )?;

    let ray_usdc_swap_data = create_swap(
        &mut client,
        &swap_program_id,
        &initializer_account,
        "ray_usdc_swap",
        "ray",
        &ray_token_mint.pubkey(),
        1_000_000_000000,
        "usdc",
        &usdc_token_mint.pubkey(),
        7_730_000_000000,
        fees.clone(),
    )?;

    let fida_usdc_swap_data = create_swap(
        &mut client,
        &swap_program_id,
        &initializer_account,
        "fida_usdc_swap",
        "fida",
        &fida_token_mint.pubkey(),
        1_000_000_000000,
        "usdc",
        &usdc_token_mint.pubkey(),
        1_640_000_000000,
        fees.clone(),
    )?;

    Ok(())
}

fn mint_to(owner: &Keypair, token_account_name: &str, token_mint: &Pubkey, client: &mut Client) -> Keypair {
    if let Ok(base58) = env::var(token_account_name) {
        Keypair::from_base58_string(&base58)
    } else {
        let token_account =
            token::create_account(client, &owner.pubkey(), token_mint).print_in_place(token_account_name);
        token::mint_to(client, &owner, token_mint, &token_account.pubkey(), 10_000_000000, 6);
        token_account
    }
}

pub struct SwapData {
    pub swap: Keypair,
    pub authority: Pubkey,
    pub authority_nonce: u8,
    pub token_a: Keypair,
    pub token_b: Keypair,
    pub pool_token_mint: Keypair,
    pub fee: Keypair,
    pub pool_token_initial: Keypair,
}

fn create_swap(
    client: &mut Client,
    swap_program_id: &Pubkey,
    owner: &Keypair,
    swap_name: &str,
    token_a_name: &str,
    token_a_mint: &Pubkey,
    token_a_amount: u64,
    token_b_name: &str,
    token_b_mint: &Pubkey,
    token_b_amount: u64,
    fees: Fees,
) -> Result<SwapData> {
    let swap = if let Ok(base58) = env::var(swap_name) {
        Keypair::from_base58_string(&base58)
    } else {
        client
            .create_account(swap_program_id, spl_token_swap::state::SwapVersion::LATEST_LEN)
            .print_in_place(swap_name)
    };

    let authority_name = format!("{}_authority", swap_name);
    let authority_nonce_name = format!("{}_authority_nonce", swap_name);
    let (authority, authority_nonce) =
        if let (Ok(authority), Ok(authority_nonce)) = (env::var(&authority_name), env::var(&authority_nonce_name)) {
            (Pubkey::from_str(&authority)?, authority_nonce.parse()?)
        } else {
            let (authority, authority_nonce) = Pubkey::find_program_address(&[swap.pubkey().as_ref()], swap_program_id);
            (authority.print_in_place(&authority_name), {
                println!("{}: {}", authority_nonce_name, authority_nonce);
                authority_nonce
            })
        };

    let swap_token_a_name = format!("{}_token_{}", swap_name, token_a_name);
    let token_a = if let Ok(base58) = env::var(&swap_token_a_name) {
        Keypair::from_base58_string(&base58)
    } else {
        let token_a = token::create_account(client, &authority, token_a_mint);
        token::mint_to(client, owner, token_a_mint, &token_a.pubkey(), token_a_amount, 6);
        token_a.print_in_place(&swap_token_a_name)
    };

    let swap_token_b_name = format!("{}_token_{}", swap_name, token_b_name);
    let token_b = if let Ok(base58) = env::var(&swap_token_b_name) {
        Keypair::from_base58_string(&base58)
    } else {
        let token_b = token::create_account(client, &authority, token_b_mint);
        token::mint_to(client, owner, token_b_mint, &token_b.pubkey(), token_b_amount, 6);
        token_b.print_in_place(&swap_token_b_name)
    };

    let pool_token_mint_name = format!("{}_pool_token_mint", swap_name);
    let fee_name = format!("{}_fee", swap_name);
    let pool_token_initial_name = format!("{}_pool_token_initial", swap_name);
    let (pool_token_mint, fee, pool_token_initial) = if let Ok(base58) = env::var(&pool_token_mint_name) {
        (
            Keypair::from_base58_string(&base58),
            Keypair::from_base58_string(&env::var(&fee_name)?),
            Keypair::from_base58_string(&env::var(&pool_token_initial_name)?),
        )
    } else {
        let token::CreateSwapResult {
            pool_token_mint,
            fee,
            pool_token_initial,
        } = token::create_swap(
            client,
            &swap_program_id,
            &swap,
            &authority,
            authority_nonce,
            &token_a.pubkey(),
            &token_b.pubkey(),
            &owner.pubkey(),
            fees,
        );
        (
            pool_token_mint.print_in_place(&pool_token_mint_name),
            fee.print_in_place(&fee_name),
            pool_token_initial.print_in_place(&pool_token_initial_name),
        )
    };

    Ok(SwapData {
        swap,
        authority,
        authority_nonce,
        token_a,
        token_b,
        pool_token_mint,
        fee,
        pool_token_initial,
    })
}
