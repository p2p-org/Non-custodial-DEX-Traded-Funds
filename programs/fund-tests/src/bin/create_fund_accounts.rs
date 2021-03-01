use std::{str::FromStr, thread, time::Duration};

use anyhow::{anyhow, Result};
use fund_tests::{client::Client, token};
use serum_pool::schema::fee_owner::ID as POOL_FEE_OWNER_ID;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Keypair, Signer},
};

fn main() -> Result<()> {
    let fund_program_id = Pubkey::from_str("HWEaSAnjKNADwA2ZopatwCM5BqrnMCPQBKAphf1s6XNW")?;

    let payer_account =
        read_keypair_file("../../solana/alice.json").map_err(|err| anyhow!("Read keypair error: {}", err))?;

    let rpc_client = RpcClient::new_with_commitment("http://localhost:8899".into(), CommitmentConfig::confirmed());
    let mut client = Client {
        client: rpc_client,
        payer: payer_account,
    };

    let initializer_account = Keypair::new();
    println!(
        "initializer_account: {}, {}",
        initializer_account.pubkey(),
        initializer_account.to_base58_string()
    );

    // Create assets
    let x_token_mint = token::create_token(&mut client, &initializer_account.pubkey(), 2);
    println!(
        "x_token_mint: {}, {}",
        x_token_mint.pubkey(),
        x_token_mint.to_base58_string()
    );
    let y_token_mint = token::create_token(&mut client, &initializer_account.pubkey(), 0);
    println!(
        "y_token_mint: {}, {}",
        y_token_mint.pubkey(),
        y_token_mint.to_base58_string()
    );

    // Mint to user
    let initializer_x_token_account =
        token::create_account(&mut client, &initializer_account.pubkey(), &x_token_mint.pubkey());
    println!(
        "initializer_x_token_account: {}, {}",
        initializer_x_token_account.pubkey(),
        initializer_x_token_account.to_base58_string()
    );

    let initializer_y_token_account =
        token::create_account(&mut client, &initializer_account.pubkey(), &y_token_mint.pubkey());
    println!(
        "initializer_y_token_account: {}, {}",
        initializer_y_token_account.pubkey(),
        initializer_y_token_account.to_base58_string()
    );

    token::mint_to(
        &mut client,
        &initializer_account,
        &x_token_mint.pubkey(),
        &initializer_x_token_account.pubkey(),
        1000,
        2,
    );
    token::mint_to(
        &mut client,
        &initializer_account,
        &y_token_mint.pubkey(),
        &initializer_y_token_account.pubkey(),
        200,
        0,
    );

    let fund_name = "Test fund";

    // Create fund accounts
    let fund_account_data_len = fund::state::calc_len(fund_name, 2);
    let fund_account = client.create_account(&fund_program_id, fund_account_data_len);
    println!(
        "fund_account: {}, {}",
        fund_account.pubkey(),
        fund_account.to_base58_string()
    );
    // let fund_account = Keypair::from_base58_string("2TainzHD6v9x8h6uefiS3Hk41GxWRpXq8e7fDf81gSoiuDCkurvRM23GR95aGQgZxbmsBwtUqLHVnXFg3tr415XY");

    let (fund_vault_authority, seed) = //(Pubkey::from_str("2ApMXqM6WBz59qWs5UzgnGYWeZP7eUXVZpBaWJ3GoNHf").unwrap(), 255);
        Pubkey::find_program_address(&[fund_account.pubkey().as_ref()], &fund_program_id);
    println!("fund_vault_authority: {}, seed: {}", fund_vault_authority, seed);

    // Create fund token
    let fund_token_mint = token::create_token(&mut client, &fund_vault_authority, 0);
    println!(
        "fund_token_mint: {}, {}",
        fund_token_mint.pubkey(),
        fund_token_mint.to_base58_string()
    );
    // let fund_token_mint = Keypair::from_base58_string("5cjxhKvxJuYJkc9X1GrnfPA4rJMuQwsLvhzsg8rbGos2dmMo1MvFh3WuafZyqy7CCwRteWLvZVnB2QzPjDraJ1jW");

    thread::sleep(Duration::from_secs(1));

    let fund_token_account = token::create_account(&mut client, &fund_vault_authority, &fund_token_mint.pubkey());
    println!(
        "fund_token_account: {}, {}",
        fund_token_account.pubkey(),
        fund_token_account.to_base58_string()
    );
    // let fund_token_account = Keypair::from_base58_string("GS83JUgS4Ni36SJLWdB9zVg1xSr4WEzUVxMpt5388NuneDRCuuVqiHHko1KiTqvTFUhWDoFumJJpyoRjrJZ5MDE");

    let fund_x_token_vault_account = token::create_account(&mut client, &fund_vault_authority, &x_token_mint.pubkey());
    println!(
        "fund_x_token_vault_account: {}, {}",
        fund_x_token_vault_account.pubkey(),
        fund_x_token_vault_account.to_base58_string()
    );
    // let fund_x_token_vault_account = Keypair::from_base58_string("1LxFNC6cuWzg6P8Emz6a7VTkNNLrgY1LA2d36i9KfJXH6nCLBG91vvGhx6gX43jHYwnEmTt1RMRFRy6hKgSkdYc");

    let fund_y_token_vault_account = token::create_account(&mut client, &fund_vault_authority, &y_token_mint.pubkey());
    println!(
        "fund_y_token_vault_account: {}, {}",
        fund_y_token_vault_account.pubkey(),
        fund_y_token_vault_account.to_base58_string()
    );
    // let fund_y_token_vault_account = Keypair::from_base58_string("3be2rgcaDdeHmVmkQzxrWgD1BUxQWjPhgMwJuYJSPAMw1AYLrW3VhLvtK3nw4LBsx47S7z45tnXqdF7L8Dph5fk3");

    thread::sleep(Duration::from_secs(1));

    // Transfer initial assets to fund
    token::transfer_to(
        &mut client,
        &initializer_account,
        &x_token_mint.pubkey(),
        &initializer_x_token_account.pubkey(),
        &fund_x_token_vault_account.pubkey(),
        70,
        2,
    );
    token::transfer_to(
        &mut client,
        &initializer_account,
        &y_token_mint.pubkey(),
        &initializer_y_token_account.pubkey(),
        &fund_y_token_vault_account.pubkey(),
        30,
        0,
    );

    // Fees
    let initializer_fee_account =
        token::create_account(&mut client, &initializer_account.pubkey(), &fund_token_mint.pubkey());
    println!(
        "initializer_fee_account: {}, {}",
        initializer_fee_account.pubkey(),
        initializer_fee_account.to_base58_string()
    );

    let lqd_fee_account = token::create_account(&mut client, &POOL_FEE_OWNER_ID, &fund_token_mint.pubkey());
    println!(
        "lqd_fee_account: {}, {}",
        lqd_fee_account.pubkey(),
        lqd_fee_account.to_base58_string()
    );

    Ok(())
}
