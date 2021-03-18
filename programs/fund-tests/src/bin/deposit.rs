use std::{env, str::FromStr};

use anyhow::{anyhow, Result};
use borsh::ser::BorshSerialize;
use fund_tests::{
    client::{Client, FundClient},
    print::Print,
    token,
};
use serum_pool::schema::{PoolAction, PoolRequest, PoolRequestInner, PoolRequestTag};
use solana_client::rpc_client::RpcClient;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
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

    let initializer_sol_token_account = Keypair::from_base58_string(&env::var("initializer_sol_token_account")?);
    let initializer_ftt_token_account = Keypair::from_base58_string(&env::var("initializer_ftt_token_account")?);
    let initializer_ren_token_account = Keypair::from_base58_string(&env::var("initializer_ren_token_account")?);
    let initializer_srm_token_account = Keypair::from_base58_string(&env::var("initializer_srm_token_account")?);
    let initializer_sushi_token_account = Keypair::from_base58_string(&env::var("initializer_sushi_token_account")?);
    let initializer_ray_token_account = Keypair::from_base58_string(&env::var("initializer_ray_token_account")?);
    let initializer_fida_token_account = Keypair::from_base58_string(&env::var("initializer_fida_token_account")?);
    let initializer_assets = [
        initializer_sol_token_account.pubkey(),
        initializer_ftt_token_account.pubkey(),
        initializer_ren_token_account.pubkey(),
        initializer_srm_token_account.pubkey(),
        initializer_sushi_token_account.pubkey(),
        initializer_ray_token_account.pubkey(),
        initializer_fida_token_account.pubkey(),
    ];

    let fund_account = Keypair::from_base58_string(&env::var("fund_account")?);
    let (pool_state, fund_state) = client.get_fund_state(&fund_account.pubkey())?;

    let fund_vault_authority = pool_state.vault_signer.pubkey();
    let fund_token_mint = pool_state.pool_token_mint.pubkey();
    let initializer_fee_account = pool_state.initializer_fee_vault.pubkey();
    let lqd_fee_account = pool_state.lqd_fee_vault.pubkey();

    let initializer_fund_token_account = if let Ok(base58) = env::var("initializer_fund_token_account") {
        Keypair::from_base58_string(&base58)
    } else {
        token::create_account(&mut client, &initializer_account.pubkey(), &fund_token_mint)
            .print_in_place("initializer_fund_token_account")
    };

    let fund_sol_token_vault_account = pool_state.assets[0].vault_address.pubkey();
    let fund_ftt_token_vault_account = pool_state.assets[1].vault_address.pubkey();
    let fund_ren_token_vault_account = pool_state.assets[2].vault_address.pubkey();
    let fund_srm_token_vault_account = pool_state.assets[3].vault_address.pubkey();
    let fund_sushi_token_vault_account = pool_state.assets[4].vault_address.pubkey();
    let fund_ray_token_vault_account = pool_state.assets[5].vault_address.pubkey();
    let fund_fida_token_vault_account = pool_state.assets[6].vault_address.pubkey();
    let _fund_usdc_token_vault_account = fund_state.basic_asset.vault_address.pubkey();

    // Print balances
    {
        let fund_token_supply = client.get_token_supply(&fund_token_mint)?;
        println!("fund_token_supply: {:?}", fund_token_supply.ui_amount);

        let initializer_fee_account_balance = client.get_token_account_balance(&initializer_fee_account)?;
        println!(
            "initializer_fee_account_balance: {:?}",
            initializer_fee_account_balance.ui_amount
        );

        let lqd_fee_account_balance = client.get_token_account_balance(&lqd_fee_account)?;
        println!("lqd_fee_account_balance: {:?}", lqd_fee_account_balance.ui_amount);

        for (i, asset) in pool_state.assets.iter().enumerate() {
            let asset_balance = client.get_token_account_balance(&asset.vault_address)?;
            println!("{} asset_balance: {:?}", i, asset_balance.ui_amount);
        }

        for (i, initializer_asset) in initializer_assets.iter().enumerate() {
            let initializer_asset_balance = client.get_token_account_balance(initializer_asset)?;
            println!(
                "{} initializer_asset_balance: {:?}",
                i, initializer_asset_balance.ui_amount
            );
        }
    }

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
                AccountMeta::new(fund_token_mint, false),
                AccountMeta::new(fund_sol_token_vault_account, false),
                AccountMeta::new(fund_ftt_token_vault_account, false),
                AccountMeta::new(fund_ren_token_vault_account, false),
                AccountMeta::new(fund_srm_token_vault_account, false),
                AccountMeta::new(fund_sushi_token_vault_account, false),
                AccountMeta::new(fund_ray_token_vault_account, false),
                AccountMeta::new(fund_fida_token_vault_account, false),
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
                AccountMeta::new(lqd_fee_account, false),
                AccountMeta::new(initializer_fee_account, false),
                AccountMeta::new(initializer_fee_account, false),
                AccountMeta::new(spl_token::id(), false),
            ],
            data,
        }],
        Some(&client.payer_pubkey()),
    );
    transaction.sign(&[client.payer(), &initializer_account], client.recent_blockhash());
    client.process_transaction(&transaction);

    // Print balances
    {
        let fund_token_supply = client.get_token_supply(&fund_token_mint)?;
        println!("fund_token_supply: {:?}", fund_token_supply.ui_amount);

        let initializer_fee_account_balance = client.get_token_account_balance(&initializer_fee_account)?;
        println!(
            "initializer_fee_account_balance: {:?}",
            initializer_fee_account_balance.ui_amount
        );

        let lqd_fee_account_balance = client.get_token_account_balance(&lqd_fee_account)?;
        println!("lqd_fee_account_balance: {:?}", lqd_fee_account_balance.ui_amount);

        for (i, asset) in pool_state.assets.iter().enumerate() {
            let asset_balance = client.get_token_account_balance(&asset.vault_address)?;
            println!("{} asset_balance: {:?}", i, asset_balance.ui_amount);
        }

        for (i, initializer_asset) in initializer_assets.iter().enumerate() {
            let initializer_asset_balance = client.get_token_account_balance(initializer_asset)?;
            println!(
                "{} initializer_asset_balance: {:?}",
                i, initializer_asset_balance.ui_amount
            );
        }
    }

    Ok(())
}
