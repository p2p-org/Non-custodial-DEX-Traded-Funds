use std::str::FromStr;

use anyhow::{anyhow, Result};
use borsh::{de::BorshDeserialize, ser::BorshSerialize};
use fund::instruction::InitializeFundData;
use fund::state::FundState;
use fund_tests::{client::Client, token};
use serum_pool::schema::{
    fee_owner::ID as POOL_FEE_OWNER_ID, InitializePoolRequest, PoolRequest, PoolRequestInner, PoolRequestTag,
    PoolState, PoolStateTag,
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

trait Print {
    fn print_in_place(self, msg: impl AsRef<str>) -> Self
    where
        Self: Sized,
    {
        self.print(msg);
        self
    }

    fn print(&self, msg: impl AsRef<str>);
}

impl Print for Keypair {
    fn print(&self, msg: impl AsRef<str>) {
        println!(
            "{}: {}, base58 = {}",
            msg.as_ref(),
            self.pubkey(),
            self.to_base58_string()
        );
    }
}

fn main() -> Result<()> {
    let fund_program_id = Pubkey::from_str("HWEaSAnjKNADwA2ZopatwCM5BqrnMCPQBKAphf1s6XNW")?;

    let payer_account =
        read_keypair_file("../../solana/alice.json").map_err(|err| anyhow!("Read keypair error: {}", err))?;

    let rpc_client = RpcClient::new_with_commitment("http://localhost:8899".into(), CommitmentConfig::confirmed());
    let mut client = Client {
        client: rpc_client,
        payer: payer_account,
    };

    let initializer_account = client
        .create_account(&system_program::id(), 0)
        .print_in_place("initializer_account");

    // Create assets
    let x_token_mint =
        token::create_token(&mut client, &initializer_account.pubkey(), 2).print_in_place("x_token_mint");

    let y_token_mint =
        token::create_token(&mut client, &initializer_account.pubkey(), 0).print_in_place("y_token_mint");

    let basic_token_mint =
        token::create_token(&mut client, &initializer_account.pubkey(), 2).print_in_place("basic_token_mint");

    // Mint to user
    let initializer_x_token_account =
        token::create_account(&mut client, &initializer_account.pubkey(), &x_token_mint.pubkey())
            .print_in_place("initializer_x_token_account");

    let initializer_y_token_account =
        token::create_account(&mut client, &initializer_account.pubkey(), &y_token_mint.pubkey())
            .print_in_place("initializer_y_token_account");

    let initializer_basic_token_account =
        token::create_account(&mut client, &initializer_account.pubkey(), &basic_token_mint.pubkey())
            .print_in_place("initializer_basic_token_account");

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
    token::mint_to(
        &mut client,
        &initializer_account,
        &basic_token_mint.pubkey(),
        &initializer_basic_token_account.pubkey(),
        100000,
        2,
    );

    let fund_name = "Test fund";

    // Create fund accounts
    let fund_account_data_len = fund::state::calc_len(fund_name, 2);
    let fund_account = client
        .create_account(&fund_program_id, fund_account_data_len)
        .print_in_place("fund_account");

    let (fund_vault_authority, seed) =
        Pubkey::find_program_address(&[fund_account.pubkey().as_ref()], &fund_program_id);
    println!("fund_vault_authority: {}, seed: {}", fund_vault_authority, seed);

    // Create fund token
    let fund_token_mint = token::create_token(&mut client, &fund_vault_authority, 0).print_in_place("fund_token_mint");

    let initial_supply_fund_token_account =
        token::create_account(&mut client, &fund_vault_authority, &fund_token_mint.pubkey())
            .print_in_place("initial_supply_fund_token_account");

    let fund_x_token_vault_account = token::create_account(&mut client, &fund_vault_authority, &x_token_mint.pubkey())
        .print_in_place("fund_x_token_vault_account");

    let fund_y_token_vault_account = token::create_account(&mut client, &fund_vault_authority, &y_token_mint.pubkey())
        .print_in_place("fund_y_token_vault_account");

    let fund_basic_token_vault_account =
        token::create_account(&mut client, &fund_vault_authority, &basic_token_mint.pubkey())
            .print_in_place("fund_basic_token_vault_account");

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
        token::create_account(&mut client, &initializer_account.pubkey(), &fund_token_mint.pubkey())
            .print_in_place("initializer_fee_account");

    let lqd_fee_account = token::create_account(&mut client, &POOL_FEE_OWNER_ID, &fund_token_mint.pubkey())
        .print_in_place("lqd_fee_account");

    // Create fund
    let initialize_fund_request = InitializePoolRequest {
        vault_signer_nonce: seed,
        assets_length: 2,
        pool_name: fund_name.to_string(),
        fee_rate: 1000,
        custom_data: InitializeFundData {
            asset_weights: vec![7, 3],
            fund_token_initial_supply: 100,
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
                AccountMeta::new(initial_supply_fund_token_account.pubkey(), false),
                AccountMeta::new_readonly(fund_basic_token_vault_account.pubkey(), false),
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
    let pool_state: PoolState = BorshDeserialize::deserialize(&mut data).unwrap();
    assert_eq!(pool_state.tag, PoolStateTag::default());
    assert_eq!(pool_state.pool_token_mint.as_ref(), &fund_token_mint.pubkey());
    assert_eq!(pool_state.assets.len(), 2);
    assert_eq!(pool_state.vault_signer.as_ref(), &fund_vault_authority);
    assert_eq!(pool_state.vault_signer_nonce, seed);
    assert_eq!(pool_state.name.as_str(), fund_name);
    assert_eq!(pool_state.fee_rate, 1000);

    let mut data = pool_state.custom_state.as_slice();
    let fund_state: FundState = BorshDeserialize::deserialize(&mut data).unwrap();
    assert_eq!(fund_state.paused, false);
    assert_eq!(fund_state.asset_weights, vec![7, 3]);
    assert_eq!(fund_state.basic_asset.mint.as_ref(), &basic_token_mint.pubkey());
    assert_eq!(
        fund_state.basic_asset.vault_address.as_ref(),
        &fund_basic_token_vault_account.pubkey()
    );

    let balance = client
        .get_token_account_balance(&initial_supply_fund_token_account.pubkey())
        .unwrap();
    let fund_token_account = client.get_account(&initial_supply_fund_token_account.pubkey()).unwrap();

    assert_eq!(balance.ui_amount, 100.0);
    assert_eq!(fund_token_account.owner, spl_token::id());
    assert_eq!(fund_token_account.executable, false);

    Ok(())
}
