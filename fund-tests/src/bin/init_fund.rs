use std::str::FromStr;

use anyhow::{anyhow, Result};
use borsh::{de::BorshDeserialize, ser::BorshSerialize};
use fund::instruction::InitializeFundData;
use fund_tests::{client::Client, token};
use serum_pool::schema::{
    fee_owner::ID as POOL_FEE_OWNER_ID, InitializePoolRequest, PoolRequest, PoolRequestInner, PoolRequestTag,
    PoolState, PoolStateTag,
};
use solana_client::rpc_client::RpcClient;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar,
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};

fn main() -> Result<()> {
    let fund_program_id = Pubkey::from_str("HWEaSAnjKNADwA2ZopatwCM5BqrnMCPQBKAphf1s6XNW")?;

    let initializer_account =
        read_keypair_file("../../solana/alice.json").map_err(|err| anyhow!("Read keypair error: {}", err))?;

    let rpc_client = RpcClient::new_with_commitment("http://localhost:8899".into(), CommitmentConfig::confirmed());
    let mut client = Client {
        client: rpc_client,
        payer: Keypair::from_bytes(&initializer_account.to_bytes()).unwrap(),
    };

    let initializer_x_token_account_pubkey = Pubkey::from_str("6F23oWcsyp7D5RkFvi4fha9oUZe1KbK1KvsmDrVS6MeL")?;
    let initializer_y_token_account_pubkey = Pubkey::from_str("Ho1Y6G3BwMHjxvFw2AVUvcPk9AcyiR6TsR3d72pLu9Me")?;

    let x_token_mint_account_pubkey = Pubkey::from_str("ENZSTnpp3FntMj9ApgypoaHY6ZJs8iosyCeFnJj81kpp")?;
    let y_token_mint_account_pubkey = Pubkey::from_str("5ZTTw1mmjcBzTYB5igjoxUCFAyw8jYczdDgfxWqzV5CS")?;

    let fund_name = "Test fund";

    // Create fund accounts
    let fund_account_data_len = fund::state::calc_len(fund_name, 2);
    let fund_account = client.create_account(&fund_program_id, fund_account_data_len);
    println!(
        "fund_account: {}, {}",
        fund_account.pubkey(),
        fund_account.to_base58_string()
    );

    let (fund_vault_authority, seed) =
        Pubkey::find_program_address(&[fund_account.pubkey().as_ref()], &fund_program_id);
    println!("fund_vault_authority: {}, seed: {}", fund_vault_authority, seed);

    // Create fund token
    let fund_token_mint = token::create_token(&mut client, &fund_vault_authority, 0);
    println!(
        "fund_token_mint: {}, {}",
        fund_token_mint.pubkey(),
        fund_token_mint.to_base58_string()
    );

    let fund_token_account = token::create_account(&mut client, &fund_vault_authority, &fund_token_mint.pubkey());
    println!(
        "fund_token_account: {}, {}",
        fund_token_account.pubkey(),
        fund_token_account.to_base58_string()
    );

    let fund_x_token_vault_account =
        token::create_account(&mut client, &fund_vault_authority, &x_token_mint_account_pubkey);
    println!(
        "fund_x_token_vault_account: {}, {}",
        fund_x_token_vault_account.pubkey(),
        fund_x_token_vault_account.to_base58_string()
    );

    let fund_y_token_vault_account =
        token::create_account(&mut client, &fund_vault_authority, &y_token_mint_account_pubkey);
    println!(
        "fund_y_token_vault_account: {}, {}",
        fund_y_token_vault_account.pubkey(),
        fund_y_token_vault_account.to_base58_string()
    );

    // Transfer initial assets to fund
    token::transfer_to(
        &mut client,
        &initializer_account,
        &x_token_mint_account_pubkey,
        &initializer_x_token_account_pubkey,
        &fund_x_token_vault_account.pubkey(),
        70,
        2,
    );
    token::transfer_to(
        &mut client,
        &initializer_account,
        &y_token_mint_account_pubkey,
        &initializer_y_token_account_pubkey,
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

    // Create fund
    let initialize_fund_request = InitializePoolRequest {
        vault_signer_nonce: seed,
        assets_length: 2,
        pool_name: fund_name.to_string(),
        fee_rate: 1000,
        custom_data: InitializeFundData {
            token_initial_amount: 100,
        }
        .try_to_vec()
        .unwrap(),
    };

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: fund_program_id,
            accounts: vec![
                AccountMeta::new(fund_account.pubkey(), false),
                AccountMeta::new(fund_token_mint.pubkey(), false),
                AccountMeta::new(fund_x_token_vault_account.pubkey(), false),
                AccountMeta::new(fund_y_token_vault_account.pubkey(), false),
                AccountMeta::new_readonly(fund_vault_authority, false),
                AccountMeta::new_readonly(lqd_fee_account.pubkey(), false),
                AccountMeta::new_readonly(initializer_fee_account.pubkey(), false),
                AccountMeta::new_readonly(sysvar::rent::id(), false),
                AccountMeta::new(initializer_account.pubkey(), false),
                AccountMeta::new(fund_token_account.pubkey(), false),
                AccountMeta::new_readonly(spl_token::id(), false),
            ],
            data: PoolRequest {
                tag: PoolRequestTag::default(),
                inner: PoolRequestInner::Initialize(initialize_fund_request),
            }
            .try_to_vec()
            .unwrap(),
        }],
        Some(&client.payer_pubkey()),
    );
    transaction.sign(&[client.payer()], client.recent_blockhash());
    client.process_transaction(&transaction);

    let fund_account = client.get_account(&fund_account.pubkey()).unwrap();

    assert_eq!(fund_account.owner, fund_program_id);
    assert_eq!(fund_account.executable, false);

    let mut data = fund_account.data.as_slice();
    let state: PoolState = BorshDeserialize::deserialize(&mut data).unwrap();
    assert_eq!(state.tag, PoolStateTag::default());
    assert_eq!(state.name.as_str(), fund_name);
    assert_eq!(state.assets.len(), 2);
    assert_eq!(state.fee_rate, 1000);

    let balance = client.get_token_account_balance(&fund_token_account.pubkey()).unwrap();
    let fund_token_account = client.get_account(&fund_token_account.pubkey()).unwrap();

    assert_eq!(balance.ui_amount, 100.0);
    assert_eq!(fund_token_account.owner, spl_token::id());
    assert_eq!(fund_token_account.executable, false);

    Ok(())
}
