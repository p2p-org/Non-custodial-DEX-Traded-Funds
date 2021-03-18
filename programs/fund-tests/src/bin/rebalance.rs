use std::{env, str::FromStr};

use anyhow::{anyhow, Result};
use borsh::ser::BorshSerialize;
use fund::instruction::{FundInstructionInner, FundRequest, FundRequestTag};
use fund_tests::client::{Client, FundClient};
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
    let swap_program_id = if let Ok(key) = env::var("SWAP_PROGRAM_ID") {
        Pubkey::from_str(&key).unwrap()
    } else {
        spl_token_swap::id()
    };

    let payer_account =
        read_keypair_file(env::var("PAYER_KEYPAIR_FILE")?).map_err(|err| anyhow!("Read keypair error: {}", err))?;

    let rpc_client = RpcClient::new_with_commitment(env::var("RPC_URL")?, CommitmentConfig::confirmed());
    let mut client = Client {
        client: rpc_client,
        payer: payer_account,
    };

    let initializer_account = Keypair::from_base58_string(&env::var("initializer_account")?);

    let fund_account = Keypair::from_base58_string(&env::var("fund_account")?);
    let (pool_state, fund_state) = client.get_fund_state(&fund_account.pubkey())?;

    let fund_vault_authority = pool_state.vault_signer.pubkey();
    let fund_token_mint = pool_state.pool_token_mint.pubkey();
    let initializer_fee_account = pool_state.initializer_fee_vault.pubkey();
    let lqd_fee_account = pool_state.lqd_fee_vault.pubkey();

    let fund_sol_token_vault_account = pool_state.assets[0].vault_address.pubkey();
    let fund_ftt_token_vault_account = pool_state.assets[1].vault_address.pubkey();
    let fund_ren_token_vault_account = pool_state.assets[2].vault_address.pubkey();
    let fund_srm_token_vault_account = pool_state.assets[3].vault_address.pubkey();
    let fund_sushi_token_vault_account = pool_state.assets[4].vault_address.pubkey();
    let fund_ray_token_vault_account = pool_state.assets[5].vault_address.pubkey();
    let fund_fida_token_vault_account = pool_state.assets[6].vault_address.pubkey();
    let fund_usdc_token_vault_account = fund_state.basic_asset.vault_address.pubkey();

    let sol_usdc_swap = Keypair::from_base58_string(&env::var("sol_usdc_swap")?);
    let ftt_usdc_swap = Keypair::from_base58_string(&env::var("ftt_usdc_swap")?);
    let ren_usdc_swap = Keypair::from_base58_string(&env::var("ren_usdc_swap")?);
    let srm_usdc_swap = Keypair::from_base58_string(&env::var("srm_usdc_swap")?);
    let sushi_usdc_swap = Keypair::from_base58_string(&env::var("sushi_usdc_swap")?);
    let ray_usdc_swap = Keypair::from_base58_string(&env::var("ray_usdc_swap")?);
    let fida_usdc_swap = Keypair::from_base58_string(&env::var("fida_usdc_swap")?);

    let sol_usdc_swap_authority = Pubkey::from_str(&env::var("sol_usdc_swap_authority")?)?;
    let ftt_usdc_swap_authority = Pubkey::from_str(&env::var("ftt_usdc_swap_authority")?)?;
    let ren_usdc_swap_authority = Pubkey::from_str(&env::var("ren_usdc_swap_authority")?)?;
    let srm_usdc_swap_authority = Pubkey::from_str(&env::var("srm_usdc_swap_authority")?)?;
    let sushi_usdc_swap_authority = Pubkey::from_str(&env::var("sushi_usdc_swap_authority")?)?;
    let ray_usdc_swap_authority = Pubkey::from_str(&env::var("ray_usdc_swap_authority")?)?;
    let fida_usdc_swap_authority = Pubkey::from_str(&env::var("fida_usdc_swap_authority")?)?;

    let sol_usdc_swap_token_sol = Keypair::from_base58_string(&env::var("sol_usdc_swap_token_sol")?);
    let ftt_usdc_swap_token_ftt = Keypair::from_base58_string(&env::var("ftt_usdc_swap_token_ftt")?);
    let ren_usdc_swap_token_ren = Keypair::from_base58_string(&env::var("ren_usdc_swap_token_ren")?);
    let srm_usdc_swap_token_srm = Keypair::from_base58_string(&env::var("srm_usdc_swap_token_srm")?);
    let sushi_usdc_swap_token_sushi = Keypair::from_base58_string(&env::var("sushi_usdc_swap_token_sushi")?);
    let ray_usdc_swap_token_ray = Keypair::from_base58_string(&env::var("ray_usdc_swap_token_ray")?);
    let fida_usdc_swap_token_fida = Keypair::from_base58_string(&env::var("fida_usdc_swap_token_fida")?);

    let sol_usdc_swap_token_usdc = Keypair::from_base58_string(&env::var("sol_usdc_swap_token_usdc")?);
    let ftt_usdc_swap_token_usdc = Keypair::from_base58_string(&env::var("ftt_usdc_swap_token_usdc")?);
    let ren_usdc_swap_token_usdc = Keypair::from_base58_string(&env::var("ren_usdc_swap_token_usdc")?);
    let srm_usdc_swap_token_usdc = Keypair::from_base58_string(&env::var("srm_usdc_swap_token_usdc")?);
    let sushi_usdc_swap_token_usdc = Keypair::from_base58_string(&env::var("sushi_usdc_swap_token_usdc")?);
    let ray_usdc_swap_token_usdc = Keypair::from_base58_string(&env::var("ray_usdc_swap_token_usdc")?);
    let fida_usdc_swap_token_usdc = Keypair::from_base58_string(&env::var("fida_usdc_swap_token_usdc")?);

    let sol_usdc_swap_pool_token_mint = Keypair::from_base58_string(&env::var("sol_usdc_swap_pool_token_mint")?);
    let ftt_usdc_swap_pool_token_mint = Keypair::from_base58_string(&env::var("ftt_usdc_swap_pool_token_mint")?);
    let ren_usdc_swap_pool_token_mint = Keypair::from_base58_string(&env::var("ren_usdc_swap_pool_token_mint")?);
    let srm_usdc_swap_pool_token_mint = Keypair::from_base58_string(&env::var("srm_usdc_swap_pool_token_mint")?);
    let sushi_usdc_swap_pool_token_mint = Keypair::from_base58_string(&env::var("sushi_usdc_swap_pool_token_mint")?);
    let ray_usdc_swap_pool_token_mint = Keypair::from_base58_string(&env::var("ray_usdc_swap_pool_token_mint")?);
    let fida_usdc_swap_pool_token_mint = Keypair::from_base58_string(&env::var("fida_usdc_swap_pool_token_mint")?);

    let sol_usdc_swap_fee = Keypair::from_base58_string(&env::var("sol_usdc_swap_fee")?);
    let ftt_usdc_swap_fee = Keypair::from_base58_string(&env::var("ftt_usdc_swap_fee")?);
    let ren_usdc_swap_fee = Keypair::from_base58_string(&env::var("ren_usdc_swap_fee")?);
    let srm_usdc_swap_fee = Keypair::from_base58_string(&env::var("srm_usdc_swap_fee")?);
    let sushi_usdc_swap_fee = Keypair::from_base58_string(&env::var("sushi_usdc_swap_fee")?);
    let ray_usdc_swap_fee = Keypair::from_base58_string(&env::var("ray_usdc_swap_fee")?);
    let fida_usdc_swap_fee = Keypair::from_base58_string(&env::var("fida_usdc_swap_fee")?);

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

        let asset_balance = client.get_token_account_balance(&fund_usdc_token_vault_account)?;
        println!("basic asset_balance: {:?}", asset_balance.ui_amount);
    }

    let pause_data = FundRequest {
        tag: FundRequestTag::default(),
        inner: FundInstructionInner::Pause,
    }
    .try_to_vec()?;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: fund_program_id,
            accounts: vec![
                AccountMeta::new(fund_account.pubkey(), false),
                AccountMeta::new_readonly(initializer_account.pubkey(), true),
            ],
            data: pause_data,
        }],
        Some(&client.payer_pubkey()),
    );
    transaction.sign(&[client.payer(), &initializer_account], client.recent_blockhash());
    client.process_transaction(&transaction);

    let rebalance_data = FundRequest {
        tag: FundRequestTag::default(),
        inner: FundInstructionInner::Rebalance,
    }
    .try_to_vec()?;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: fund_program_id,
            accounts: vec![
                AccountMeta::new(fund_account.pubkey(), false),
                AccountMeta::new_readonly(initializer_account.pubkey(), true),
                AccountMeta::new(fund_sol_token_vault_account, false),
                AccountMeta::new(fund_ftt_token_vault_account, false),
                AccountMeta::new(fund_ren_token_vault_account, false),
                AccountMeta::new(fund_srm_token_vault_account, false),
                AccountMeta::new(fund_sushi_token_vault_account, false),
                AccountMeta::new(fund_ray_token_vault_account, false),
                AccountMeta::new(fund_fida_token_vault_account, false),
                AccountMeta::new_readonly(fund_vault_authority, false),
                AccountMeta::new(fund_usdc_token_vault_account, false),
                // SOL
                AccountMeta::new_readonly(sol_usdc_swap.pubkey(), false),
                AccountMeta::new_readonly(sol_usdc_swap_authority, false),
                AccountMeta::new(sol_usdc_swap_token_sol.pubkey(), false),
                AccountMeta::new(sol_usdc_swap_token_usdc.pubkey(), false),
                AccountMeta::new(sol_usdc_swap_pool_token_mint.pubkey(), false),
                AccountMeta::new(sol_usdc_swap_fee.pubkey(), false),
                // FTT
                AccountMeta::new_readonly(ftt_usdc_swap.pubkey(), false),
                AccountMeta::new_readonly(ftt_usdc_swap_authority, false),
                AccountMeta::new(ftt_usdc_swap_token_ftt.pubkey(), false),
                AccountMeta::new(ftt_usdc_swap_token_usdc.pubkey(), false),
                AccountMeta::new(ftt_usdc_swap_pool_token_mint.pubkey(), false),
                AccountMeta::new(ftt_usdc_swap_fee.pubkey(), false),
                // REN
                AccountMeta::new_readonly(ren_usdc_swap.pubkey(), false),
                AccountMeta::new_readonly(ren_usdc_swap_authority, false),
                AccountMeta::new(ren_usdc_swap_token_ren.pubkey(), false),
                AccountMeta::new(ren_usdc_swap_token_usdc.pubkey(), false),
                AccountMeta::new(ren_usdc_swap_pool_token_mint.pubkey(), false),
                AccountMeta::new(ren_usdc_swap_fee.pubkey(), false),
                // SRM
                AccountMeta::new_readonly(srm_usdc_swap.pubkey(), false),
                AccountMeta::new_readonly(srm_usdc_swap_authority, false),
                AccountMeta::new(srm_usdc_swap_token_srm.pubkey(), false),
                AccountMeta::new(srm_usdc_swap_token_usdc.pubkey(), false),
                AccountMeta::new(srm_usdc_swap_pool_token_mint.pubkey(), false),
                AccountMeta::new(srm_usdc_swap_fee.pubkey(), false),
                // SUSHI
                AccountMeta::new_readonly(sushi_usdc_swap.pubkey(), false),
                AccountMeta::new_readonly(sushi_usdc_swap_authority, false),
                AccountMeta::new(sushi_usdc_swap_token_sushi.pubkey(), false),
                AccountMeta::new(sushi_usdc_swap_token_usdc.pubkey(), false),
                AccountMeta::new(sushi_usdc_swap_pool_token_mint.pubkey(), false),
                AccountMeta::new(sushi_usdc_swap_fee.pubkey(), false),
                // RAY
                AccountMeta::new_readonly(ray_usdc_swap.pubkey(), false),
                AccountMeta::new_readonly(ray_usdc_swap_authority, false),
                AccountMeta::new(ray_usdc_swap_token_ray.pubkey(), false),
                AccountMeta::new(ray_usdc_swap_token_usdc.pubkey(), false),
                AccountMeta::new(ray_usdc_swap_pool_token_mint.pubkey(), false),
                AccountMeta::new(ray_usdc_swap_fee.pubkey(), false),
                // FIDA
                AccountMeta::new_readonly(fida_usdc_swap.pubkey(), false),
                AccountMeta::new_readonly(fida_usdc_swap_authority, false),
                AccountMeta::new(fida_usdc_swap_token_fida.pubkey(), false),
                AccountMeta::new(fida_usdc_swap_token_usdc.pubkey(), false),
                AccountMeta::new(fida_usdc_swap_pool_token_mint.pubkey(), false),
                AccountMeta::new(fida_usdc_swap_fee.pubkey(), false),
                //
                AccountMeta::new_readonly(spl_token::id(), false),
                AccountMeta::new_readonly(swap_program_id, false),
            ],
            data: rebalance_data,
        }],
        Some(&client.payer_pubkey()),
    );
    transaction.sign(&[client.payer(), &initializer_account], client.recent_blockhash());
    client.process_transaction(&transaction);

    let unpause_data = FundRequest {
        tag: FundRequestTag::default(),
        inner: FundInstructionInner::Unpause,
    }
    .try_to_vec()?;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: fund_program_id,
            accounts: vec![
                AccountMeta::new(fund_account.pubkey(), false),
                AccountMeta::new_readonly(initializer_account.pubkey(), true),
                AccountMeta::new(fund_sol_token_vault_account, false),
                AccountMeta::new(fund_ftt_token_vault_account, false),
                AccountMeta::new(fund_ren_token_vault_account, false),
                AccountMeta::new(fund_srm_token_vault_account, false),
                AccountMeta::new(fund_sushi_token_vault_account, false),
                AccountMeta::new(fund_ray_token_vault_account, false),
                AccountMeta::new(fund_fida_token_vault_account, false),
            ],
            data: unpause_data,
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

        let asset_balance = client.get_token_account_balance(&fund_usdc_token_vault_account)?;
        println!("basic asset_balance: {:?}", asset_balance.ui_amount);
    }

    Ok(())
}
