use borsh::ser::BorshSerialize;
use fund::{
    processor::Fund,
    state::{self as fund_state},
};
use serum_pool::{
    pool_entrypoint,
    schema::{
        fee_owner::ID as POOL_FEE_OWNER_ID, InitializePoolRequest, PoolRequest, PoolRequestInner, PoolRequestTag,
    },
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar,
};
use solana_program_test::{processor, ProgramTest};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

use self::helpers::{client::TestClient, token};

mod helpers;

#[tokio::test]
async fn test_init_fund() {
    let program_key = Keypair::new();
    let program_id = program_key.pubkey();
    let mut client: TestClient = ProgramTest::new("fund", program_id, processor!(pool_entrypoint::<Fund>))
        .start()
        .await
        .into();

    let initializer_account = Keypair::new();

    // Create assets
    let x_token_mint = token::create_token(&mut client, &initializer_account.pubkey(), 2).await;
    let y_token_mint = token::create_token(&mut client, &initializer_account.pubkey(), 0).await;

    // Mint to user
    let initializer_x_token_account =
        token::create_account(&mut client, &initializer_account.pubkey(), &x_token_mint.pubkey()).await;
    let initializer_y_token_account =
        token::create_account(&mut client, &initializer_account.pubkey(), &y_token_mint.pubkey()).await;
    token::mint_to(
        &mut client,
        &initializer_account,
        &x_token_mint.pubkey(),
        &initializer_x_token_account.pubkey(),
        1000,
        2,
    )
    .await;
    token::mint_to(
        &mut client,
        &initializer_account,
        &y_token_mint.pubkey(),
        &initializer_y_token_account.pubkey(),
        200,
        0,
    )
    .await;

    let fund_name = "Test fund";

    // Create fund accounts
    let fund_account_data_len = fund_state::calc_len(fund_name, 2);
    println!("Data len: {}", fund_account_data_len);
    let fund_account = client.create_account(&program_id, fund_account_data_len).await;
    let (fund_vault_authority, seed) = Pubkey::find_program_address(&[fund_account.pubkey().as_ref()], &program_id);

    // Create fund token
    let fund_token_mint = token::create_token(&mut client, &fund_vault_authority, 0).await;
    let fund_token_account = token::create_account(&mut client, &fund_vault_authority, &fund_token_mint.pubkey()).await;

    let fund_x_token_vault_account =
        token::create_account(&mut client, &fund_vault_authority, &x_token_mint.pubkey()).await;
    let fund_y_token_vault_account =
        token::create_account(&mut client, &fund_vault_authority, &y_token_mint.pubkey()).await;

    // Transfer initial assets to fund
    token::transfer_to(
        &mut client,
        &initializer_account,
        &x_token_mint.pubkey(),
        &initializer_x_token_account.pubkey(),
        &fund_x_token_vault_account.pubkey(),
        70,
        2,
    )
    .await;
    token::transfer_to(
        &mut client,
        &initializer_account,
        &y_token_mint.pubkey(),
        &initializer_y_token_account.pubkey(),
        &fund_y_token_vault_account.pubkey(),
        30,
        0,
    )
    .await;

    // Fees
    let initializer_fee_account =
        token::create_account(&mut client, &initializer_account.pubkey(), &fund_token_mint.pubkey()).await;
    let lqd_fee_account = token::create_account(&mut client, &POOL_FEE_OWNER_ID, &fund_token_mint.pubkey()).await;

    // Create fund
    let initialize_fund_request = InitializePoolRequest {
        vault_signer_nonce: seed,
        assets_length: 2,
        pool_name: fund_name.to_string(),
        fee_rate: 1000,
        custom_data: vec![],
    };

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id,
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
    client.process_transaction(transaction).await.unwrap();
}
