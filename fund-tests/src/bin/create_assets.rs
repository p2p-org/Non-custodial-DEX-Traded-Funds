use std::env;

use anyhow::{anyhow, Result};
use dotenv::dotenv;
use fund_tests::{client::Client, print::Print, token};
use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey::Pubkey, system_program};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Keypair, Signer},
};

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
    // MakerDAO, AAVE, Compound, Curve, UniSwap, Synthetix, Balancer, USDC

    let maker_dao_token_mint = if let Ok(base58) = env::var("maker_dao_token_mint") {
        Keypair::from_base58_string(&base58)
    } else {
        token::create_token(&mut client, &initializer_account.pubkey(), 6).print_in_place("maker_dao_token_mint")
    };

    let aave_token_mint = if let Ok(base58) = env::var("aave_token_mint") {
        Keypair::from_base58_string(&base58)
    } else {
        token::create_token(&mut client, &initializer_account.pubkey(), 6).print_in_place("aave_token_mint")
    };

    let compound_token_mint = if let Ok(base58) = env::var("compound_token_mint") {
        Keypair::from_base58_string(&base58)
    } else {
        token::create_token(&mut client, &initializer_account.pubkey(), 6).print_in_place("compound_token_mint")
    };

    let curve_token_mint = if let Ok(base58) = env::var("curve_token_mint") {
        Keypair::from_base58_string(&base58)
    } else {
        token::create_token(&mut client, &initializer_account.pubkey(), 6).print_in_place("curve_token_mint")
    };

    let uni_swap_token_mint = if let Ok(base58) = env::var("uni_swap_token_mint") {
        Keypair::from_base58_string(&base58)
    } else {
        token::create_token(&mut client, &initializer_account.pubkey(), 6).print_in_place("uni_swap_token_mint")
    };

    let synthetix_token_mint = if let Ok(base58) = env::var("synthetix_token_mint") {
        Keypair::from_base58_string(&base58)
    } else {
        token::create_token(&mut client, &initializer_account.pubkey(), 6).print_in_place("synthetix_token_mint")
    };

    let balancer_token_mint = if let Ok(base58) = env::var("balancer_token_mint") {
        Keypair::from_base58_string(&base58)
    } else {
        token::create_token(&mut client, &initializer_account.pubkey(), 6).print_in_place("balancer_token_mint")
    };

    let usdc_token_mint = if let Ok(base58) = env::var("usdc_token_mint") {
        Keypair::from_base58_string(&base58)
    } else {
        token::create_token(&mut client, &initializer_account.pubkey(), 6).print_in_place("usdc_token_mint")
    };

    // Mint to initializer

    let initializer_maker_dao_token_account = mint_to(
        &initializer_account,
        "initializer_maker_dao_token_account",
        &maker_dao_token_mint.pubkey(),
        &mut client,
    );

    let initializer_aave_token_account = mint_to(
        &initializer_account,
        "initializer_aave_token_account",
        &aave_token_mint.pubkey(),
        &mut client,
    );

    let initializer_compound_token_account = mint_to(
        &initializer_account,
        "initializer_compound_token_account",
        &compound_token_mint.pubkey(),
        &mut client,
    );

    let initializer_curve_token_account = mint_to(
        &initializer_account,
        "initializer_curve_token_account",
        &curve_token_mint.pubkey(),
        &mut client,
    );

    let initializer_uni_swap_token_account = mint_to(
        &initializer_account,
        "initializer_uni_swap_token_account",
        &uni_swap_token_mint.pubkey(),
        &mut client,
    );

    let initializer_synthetix_token_account = mint_to(
        &initializer_account,
        "initializer_synthetix_token_account",
        &synthetix_token_mint.pubkey(),
        &mut client,
    );

    let initializer_balancer_token_account = mint_to(
        &initializer_account,
        "initializer_balancer_token_account",
        &balancer_token_mint.pubkey(),
        &mut client,
    );

    let initializer_usdc_token_account = mint_to(
        &initializer_account,
        "initializer_usdc_token_account",
        &usdc_token_mint.pubkey(),
        &mut client,
    );

    let balance = client.get_token_account_balance(&initializer_maker_dao_token_account.pubkey())?;
    assert_eq!(balance.ui_amount, 10_000.0);
    let balance = client.get_token_account_balance(&initializer_aave_token_account.pubkey())?;
    assert_eq!(balance.ui_amount, 10_000.0);
    let balance = client.get_token_account_balance(&initializer_compound_token_account.pubkey())?;
    assert_eq!(balance.ui_amount, 10_000.0);
    let balance = client.get_token_account_balance(&initializer_curve_token_account.pubkey())?;
    assert_eq!(balance.ui_amount, 10_000.0);
    let balance = client.get_token_account_balance(&initializer_uni_swap_token_account.pubkey())?;
    assert_eq!(balance.ui_amount, 10_000.0);
    let balance = client.get_token_account_balance(&initializer_synthetix_token_account.pubkey())?;
    assert_eq!(balance.ui_amount, 10_000.0);
    let balance = client.get_token_account_balance(&initializer_balancer_token_account.pubkey())?;
    assert_eq!(balance.ui_amount, 10_000.0);
    let balance = client.get_token_account_balance(&initializer_usdc_token_account.pubkey())?;
    assert_eq!(balance.ui_amount, 10_000.0);

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
