use std::{env, str::FromStr};

use anyhow::{anyhow, Result};
use borsh::{de::BorshDeserialize, ser::BorshSerialize};
use fund::instruction::InitializeFundData;
use fund::state::FundState;
use fund_tests::{client::Client, print::Print, token};
use serum_pool::schema::{
    fee_owner::ID as POOL_FEE_OWNER_ID, InitializePoolRequest, PoolAction, PoolRequest, PoolRequestInner,
    PoolRequestTag, PoolState, PoolStateTag,
};
use solana_client::rpc_client::RpcClient;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};

fn main() -> Result<()> {
    dotenv::from_filename(".env.local").ok();

    let fund_program_id = Pubkey::from_str(&env::var("FUND_PROGRAM_ID")?)?;
    let payer_account =
        read_keypair_file(env::var("PAYER_KEYPAIR_FILE")?).map_err(|err| anyhow!("Read keypair error: {}", err))?;

    let rpc_client = RpcClient::new_with_commitment(env::var("RPC_URL")?, CommitmentConfig::confirmed());
    let mut client = Client {
        client: rpc_client,
        payer: payer_account,
    };

    let initializer_account = Keypair::from_base58_string(&env::var("initializer_account")?);

    let sol_token_mint = Keypair::from_base58_string(&env::var("sol_token_mint")?);
    let ftt_token_mint = Keypair::from_base58_string(&env::var("ftt_token_mint")?);
    let ren_token_mint = Keypair::from_base58_string(&env::var("ren_token_mint")?);
    let srm_token_mint = Keypair::from_base58_string(&env::var("srm_token_mint")?);
    let sushi_token_mint = Keypair::from_base58_string(&env::var("sushi_token_mint")?);
    let ray_token_mint = Keypair::from_base58_string(&env::var("ray_token_mint")?);
    let fida_token_mint = Keypair::from_base58_string(&env::var("fida_token_mint")?);
    let usdc_token_mint = Keypair::from_base58_string(&env::var("usdc_token_mint")?);

    let initializer_sol_token_account = Keypair::from_base58_string(&env::var("initializer_sol_token_account")?);
    let initializer_ftt_token_account = Keypair::from_base58_string(&env::var("initializer_ftt_token_account")?);
    let initializer_ren_token_account = Keypair::from_base58_string(&env::var("initializer_ren_token_account")?);
    let initializer_srm_token_account = Keypair::from_base58_string(&env::var("initializer_srm_token_account")?);
    let initializer_sushi_token_account = Keypair::from_base58_string(&env::var("initializer_sushi_token_account")?);
    let initializer_ray_token_account = Keypair::from_base58_string(&env::var("initializer_ray_token_account")?);
    let initializer_fida_token_account = Keypair::from_base58_string(&env::var("initializer_fida_token_account")?);

    let initializer_fund_token_account = if let Ok(base58) = env::var("initializer_fund_token_account") {
        Keypair::from_base58_string(&base58)
    } else {
        token::create_account(&mut client, &initializer_account.pubkey(), &fund_token_mint.pubkey())
            .print_in_place("initializer_fund_token_account")
    };
    let initializer_fee_account = Keypair::from_base58_string(&env::var("initializer_fee_account")?);
    let lqd_fee_account = Keypair::from_base58_string(&env::var("lqd_fee_account")?);

    let fund_account = Keypair::from_base58_string(&env::var("fund_account")?);
    let fund_vault_authority = Pubkey::from_str(&env::var("fund_vault_authority")?)?;
    let fund_token_mint = Keypair::from_base58_string(&env::var("fund_token_mint")?);

    let fund_sol_token_vault_account = Keypair::from_base58_string(&env::var("fund_account")?);
    let fund_ftt_token_vault_account = Keypair::from_base58_string(&env::var("fund_account")?);
    let fund_ren_token_vault_account = Keypair::from_base58_string(&env::var("fund_account")?);
    let fund_srm_token_vault_account = Keypair::from_base58_string(&env::var("fund_account")?);
    let fund_sushi_token_vault_account = Keypair::from_base58_string(&env::var("fund_account")?);
    let fund_ray_token_vault_account = Keypair::from_base58_string(&env::var("fund_account")?);
    let fund_fida_token_vault_account = Keypair::from_base58_string(&env::var("fund_account")?);
    let fund_usdc_token_vault_account = Keypair::from_base58_string(&env::var("fund_account")?);

    let (pool_state, fund_state) = get_fund_state(&client, &fund_account.pubkey())?;

    let data = PoolRequest {
        tag: PoolRequestTag::default(),
        inner: PoolRequestInner::Execute(PoolAction::Create(10000)),
    }
    .try_to_vec()?;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: fund_program_id,
            accounts: vec![
                AccountMeta::new(fund_account.pubkey(), false),
                AccountMeta::new(fund_token_mint.pubkey(), false),
                AccountMeta::new(fund_sol_token_vault_account.pubkey(), false),
                AccountMeta::new(fund_ftt_token_vault_account.pubkey(), false),
                AccountMeta::new(fund_ren_token_vault_account.pubkey(), false),
                AccountMeta::new(fund_srm_token_vault_account.pubkey(), false),
                AccountMeta::new(fund_sushi_token_vault_account.pubkey(), false),
                AccountMeta::new(fund_ray_token_vault_account.pubkey(), false),
                AccountMeta::new(fund_fida_token_vault_account.pubkey(), false),
                AccountMeta::new_readonly(fund_vault_authority, false),
                AccountMeta::new(initializer_fund_token_account.pubkey(), false),
                AccountMeta::new(initializer_sol_token_account.pubkey(), false),
                AccountMeta::new(initializer_ftt_token_account.pubkey(), false),
                AccountMeta::new(initializer_ren_token_account.pubkey(), false),
                AccountMeta::new(initializer_srm_token_account.pubkey(), false),
                AccountMeta::new(initializer_sushi_token_account.pubkey(), false),
                AccountMeta::new(initializer_ray_token_account.pubkey(), false),
                AccountMeta::new(initializer_fida_token_account.pubkey(), false),
                AccountMeta::new(initializer_account.pubkey(), true),
                AccountMeta::new(lqd_fee_account.pubkey(), false),
                AccountMeta::new(initializer_fee_account.pubkey(), false),
                AccountMeta::new(initializer_fee_account.pubkey(), false),
                AccountMeta::new(spl_token::id(), false),
            ],
            data,
        }],
        Some(&client.payer_pubkey()),
    );
    transaction.sign(&[client.payer(), &initializer_account], client.recent_blockhash());
    client.process_transaction(&transaction);

    let fund_account = client.get_account(&fund_account.pubkey())?;

    assert_eq!(fund_account.owner, fund_program_id);
    assert_eq!(fund_account.executable, false);

    let mut data = fund_account.data.as_slice();
    let pool_state: PoolState = BorshDeserialize::deserialize(&mut data)?;
    assert_eq!(pool_state.tag, PoolStateTag::default());
    assert_eq!(pool_state.pool_token_mint.as_ref(), &fund_token_mint.pubkey());
    assert_eq!(pool_state.assets.len(), 7);
    assert_eq!(pool_state.vault_signer.as_ref(), &fund_vault_authority);
    assert_eq!(pool_state.vault_signer_nonce, seed);
    assert_eq!(pool_state.name.as_str(), fund_name);
    assert_eq!(pool_state.fee_rate, 1000);

    let mut data = pool_state.custom_state.as_slice();
    let fund_state: FundState = BorshDeserialize::deserialize(&mut data)?;
    assert_eq!(fund_state.paused, false);
    assert_eq!(fund_state.asset_weights, asset_weights);
    assert_eq!(fund_state.basic_asset.mint.as_ref(), &usdc_token_mint.pubkey());
    assert_eq!(
        fund_state.basic_asset.vault_address.as_ref(),
        &fund_usdc_token_vault_account.pubkey()
    );

    let balance = client.get_token_account_balance(&initial_supply_fund_token_account.pubkey())?;
    let fund_token_account = client.get_account(&initial_supply_fund_token_account.pubkey())?;

    assert_eq!(balance.ui_amount, fund_token_initial_supply as f64 / 1000000.0);
    assert_eq!(fund_token_account.owner, spl_token::id());
    assert_eq!(fund_token_account.executable, false);

    Ok(())
}

fn get_fund_state(client: &Client, fund_account: &Pubkey) -> Result<(PoolState, FundState)> {
    let fund_account = client.get_account(fund_account)?;

    let mut data = fund_account.data.as_slice();
    let pool_state: PoolState = BorshDeserialize::deserialize(&mut data)?;

    data = pool_state.custom_state.as_slice();
    let fund_state: FundState = BorshDeserialize::deserialize(&mut data)?;

    Ok((pool_state, fund_state))
}
