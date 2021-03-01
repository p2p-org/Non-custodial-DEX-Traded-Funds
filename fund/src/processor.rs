use std::convert::TryFrom;

use borsh::{BorshDeserialize, BorshSerialize};
use serum_pool::{
    context::{check_account_address, check_token_account},
    next_account_infos,
    schema::{AssetInfo, Basket, InitializePoolRequest, PoolState, FEE_RATE_DENOMINATOR, MIN_FEE_RATE},
    Pool, PoolContext,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
};
use spl_token::state::Account as TokenAccount;

use crate::{
    error::FundError,
    instruction::{FundInstructionInner, FundRequest, InitializeFundData},
    state::{FundState, FundStateContainer},
};

pub struct Fund;

impl Pool for Fund {
    fn initialize_pool(
        context: &PoolContext,
        state: &mut PoolState,
        request: &InitializePoolRequest,
    ) -> Result<(), ProgramError> {
        let admin_account = context.custom_accounts.get(0).ok_or_else(|| {
            msg!("Missing fund admin account");
            ProgramError::NotEnoughAccountKeys
        })?;
        let initial_supply_fund_token_account = context.custom_accounts.get(1).ok_or_else(|| {
            msg!("Missing initial supply fund token account");
            ProgramError::NotEnoughAccountKeys
        })?;
        let basic_asset_vault_account = context.custom_accounts.get(2).ok_or_else(|| {
            msg!("Missing fund vault account of basic asset");
            ProgramError::NotEnoughAccountKeys
        })?;
        let spl_token_program_account = context.custom_accounts.get(3).ok_or_else(|| {
            msg!("Missing spl-token account");
            ProgramError::NotEnoughAccountKeys
        })?;

        context.check_rent_exemption(admin_account)?;
        state.admin_key = Some(admin_account.key.into());

        context.check_rent_exemption(initial_supply_fund_token_account)?;
        check_token_account(
            initial_supply_fund_token_account,
            context.pool_token_mint.key,
            Some(state.vault_signer.as_ref()),
        )?;

        context.check_rent_exemption(basic_asset_vault_account)?;
        let basic_asset_token_account = TokenAccount::unpack(&basic_asset_vault_account.try_borrow_data()?)?;
        let basic_asset = AssetInfo {
            mint: basic_asset_token_account.mint.into(),
            vault_address: basic_asset_vault_account.key.into(),
        };
        check_token_account(
            basic_asset_vault_account,
            basic_asset.mint.as_ref(),
            Some(state.vault_signer.as_ref()),
        )?;

        let fund_data: InitializeFundData = {
            let mut data = request.custom_data.as_slice();
            BorshDeserialize::deserialize(&mut data).map_err(|err| {
                msg!("Deserialize initial fund data error: {}", err);
                ProgramError::InvalidInstructionData
            })?
        };

        if fund_data.asset_weights.len() != state.assets.len() {
            msg!(
                "Asset weights count {} does not match the assets count {}",
                fund_data.asset_weights.len(),
                state.assets.len()
            );
            return Err(ProgramError::InvalidInstructionData);
        }

        state.write_fund_state(&FundState {
            paused: false,
            slippage_divider: fund_data.slippage_divider,
            asset_weights: fund_data.asset_weights,
            basic_asset,
        })?;

        msg!("Mint initial tokens");
        invoke_signed(
            &spl_token::instruction::mint_to(
                &spl_token::id(),
                context.pool_token_mint.key,
                initial_supply_fund_token_account.key,
                context.pool_authority.key,
                &[],
                fund_data.fund_token_initial_supply,
            )?,
            &[
                initial_supply_fund_token_account.clone(),
                context.pool_token_mint.clone(),
                context.pool_authority.clone(),
                spl_token_program_account.clone(),
            ],
            &[&[context.pool_account.key.as_ref(), &[state.vault_signer_nonce]]],
        )?;

        Ok(())
    }

    fn get_creation_basket(
        context: &PoolContext,
        state: &PoolState,
        creation_size: u64,
    ) -> Result<Basket, ProgramError> {
        let custom_state = state.read_fund_state()?;
        if custom_state.paused {
            msg!("Fund is paused");
            return Err(ProgramError::InvalidArgument);
        }
        context.get_simple_basket(creation_size, true)
    }

    fn get_redemption_basket(
        context: &PoolContext,
        state: &PoolState,
        redemption_size: u64,
    ) -> Result<Basket, ProgramError> {
        let custom_state = state.read_fund_state()?;
        if custom_state.paused {
            msg!("Fund is paused");
            return Err(ProgramError::InvalidArgument);
        }
        context.get_simple_basket(redemption_size, false)
    }

    #[allow(unused_variables)]
    fn process_foreign_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.into_iter();

        let pool_account = next_account_info(accounts_iter)?;
        if pool_account.owner != program_id {
            msg!("Account not owned by fund program");
            return Err(ProgramError::IncorrectProgramId);
        }
        let mut pool_state: PoolState = {
            let data = pool_account.try_borrow_data()?;
            let mut data: &[u8] = *data;
            BorshDeserialize::deserialize(&mut data).map_err(|e| {
                msg!(&e.to_string());
                ProgramError::InvalidAccountData
            })?
        };

        let admin_account = next_account_info(accounts_iter)?;
        if pool_state.admin_key.as_ref().map(AsRef::as_ref) != Some(admin_account.key) {
            msg!("Incorrect admin account");
            return Err(ProgramError::InvalidArgument);
        }
        if !admin_account.is_signer {
            msg!("Admin account not signer");
            return Err(ProgramError::MissingRequiredSignature);
        }

        let instruction: FundInstructionInner = FundRequest::try_from_slice(instruction_data)
            .map_err(|_| {
                msg!("Invalid instruction data");
                ProgramError::InvalidInstructionData
            })?
            .inner;

        Self::process_admin_request(&pool_account, accounts_iter, &mut pool_state, &instruction)?;

        let mut buf = Vec::new();
        BorshSerialize::serialize(&pool_state, &mut buf).map_err(|_| ProgramError::AccountDataTooSmall)?;

        if pool_account.data_len() != buf.len() {
            msg!(
                "Actual pool account data len {} does not match the required {}",
                buf.len(),
                pool_account.data_len()
            );
            Err(ProgramError::InvalidAccountData)
        } else {
            pool_account.try_borrow_mut_data()?.copy_from_slice(&buf);
            Ok(())
        }
    }
}

impl Fund {
    fn process_admin_request(
        pool_account: &AccountInfo,
        accounts_iter: &mut std::slice::Iter<AccountInfo>,
        pool_state: &mut PoolState,
        request: &FundInstructionInner,
    ) -> Result<(), ProgramError> {
        let mut fund_state = pool_state.read_fund_state()?;

        match request {
            FundInstructionInner::Pause => {
                fund_state.paused = true;
            }
            FundInstructionInner::Unpause => {
                for asset in &pool_state.assets {
                    let vault_account = next_account_info(accounts_iter)?;
                    if vault_account.key != asset.vault_address.as_ref() {
                        msg!("Incorrect vault address");
                        return Err(ProgramError::InvalidArgument);
                    }
                    let parsed = parse_token_account(vault_account)?;
                    if parsed.delegate.is_some() && parsed.delegated_amount > 0 {
                        msg!("Cannot unpause fund with delegated assets");
                        return Err(ProgramError::InvalidArgument);
                    }
                }
                fund_state.paused = false;
            }
            FundInstructionInner::Rebalance => {
                if fund_state.paused {
                    msg!("Fund is paused");
                    return Err(ProgramError::InvalidArgument);
                }

                let assets_count = pool_state.assets.len();
                let pool_vaults = next_account_infos(accounts_iter, assets_count)?;
                let vault_signer = next_account_info(accounts_iter)?;
                let basic_asset_vault = next_account_info(accounts_iter)?;
                let spl_token_program = next_account_info(accounts_iter)?;
                let token_swaps = next_account_infos(accounts_iter, assets_count)?;
                let swap_authorities = next_account_infos(accounts_iter, assets_count)?;
                let swap_assets = next_account_infos(accounts_iter, assets_count)?;
                let swap_basic_asset = next_account_info(accounts_iter)?;
                let swap_pool_token_mints = next_account_infos(accounts_iter, assets_count)?;
                let swap_fees = next_account_infos(accounts_iter, assets_count)?;

                // Check the accounts
                check_account_address(vault_signer, &pool_state.vault_signer, stringify!(vault_signer))?;
                check_token_account(
                    basic_asset_vault,
                    &fund_state.basic_asset.mint,
                    Some(&pool_state.vault_signer),
                )?;
                check_account_address(vault_signer, &pool_state.vault_signer, stringify!(vault_signer))?;
                if spl_token_program.key != &spl_token::ID {
                    msg!("Incorrect spl-token program ID");
                    return Err(ProgramError::InvalidArgument);
                }
                for i in 0..assets_count {
                    let asset = &pool_state.assets[i];
                    let asset_vault = &pool_vaults[i];
                    let token_swap = &token_swaps[i];
                    let swap_asset = &swap_assets[i];

                    msg!("Check accounts for asset number {}", i);
                    check_account_address(asset_vault, &asset.vault_address, stringify!(asset_vault))?;
                    check_token_account(asset_vault, &asset.mint, Some(&pool_state.vault_signer))?;
                    if token_swap.owner != &spl_token_swap::id() {
                        msg!("Token-swap account {} not owned by spl-token-swap program", i);
                        return Err(ProgramError::InvalidAccountData);
                    }
                    check_token_account(swap_asset, &asset.mint, Some(&pool_state.vault_signer))?;
                }
                check_token_account(
                    swap_basic_asset,
                    &fund_state.basic_asset.mint,
                    Some(&pool_state.vault_signer),
                )?;

                // Calc the current amounts in the basic asset
                let basic_asset_vault_token_account = TokenAccount::unpack(&basic_asset_vault.try_borrow_data()?)?;
                let swap_basic_asset_token_account = TokenAccount::unpack(&swap_basic_asset.try_borrow_data()?)?;
                let mut current_asset_amounts = Vec::with_capacity(pool_vaults.len());
                let mut asset_vault_token_accounts = Vec::with_capacity(pool_vaults.len());
                let mut total_amount = basic_asset_vault_token_account.amount as u128;

                for (i, vault) in pool_vaults.iter().enumerate() {
                    let vault_token_account = TokenAccount::unpack(&vault.try_borrow_data()?)?;
                    let swap_asset_token_account = TokenAccount::unpack(&swap_assets[i].try_borrow_data()?)?;
                    let amount = vault_token_account.amount as u128 * swap_asset_token_account.amount as u128
                        / swap_basic_asset_token_account.amount as u128;

                    total_amount += amount;
                    let amount = u64::try_from(amount).map_err(|err| {
                        msg!("Amount overflowed: {}", err);
                        FundError::OperationOverflow
                    })?;
                    current_asset_amounts.push(amount);
                    asset_vault_token_accounts.push(vault_token_account);
                }

                // Calc the needed amounts in the basic asset
                let total_weight = fund_state
                    .asset_weights
                    .iter()
                    .fold(0_u128, |sum, weight| sum + *weight as u128);
                let mut to_buy = Vec::new();

                for (i, &amount) in current_asset_amounts.iter().enumerate() {
                    let need_amount = u64::try_from(fund_state.asset_weights[i] as u128 * total_amount / total_weight)
                        .map_err(|err| {
                            msg!("Need amount overflow: {}", err);
                            FundError::OperationOverflow
                        })?;

                    if need_amount < amount - amount / fund_state.slippage_divider {
                        msg!("To sell asset {}", i);

                        let amount_delta = amount - need_amount;
                        let amount_in = u64::try_from(
                            amount_delta as u128 * asset_vault_token_accounts[i].amount as u128 / amount as u128,
                        )
                        .map_err(|err| {
                            msg!("Sell amount_in overflowed: {}", err);
                            FundError::OperationOverflow
                        })?;
                        let minimum_amount_out = amount_delta - amount_delta / fund_state.slippage_divider;

                        let swap_instruction = spl_token_swap::instruction::swap(
                            &spl_token_swap::id(),
                            &spl_token::id(),
                            token_swaps[i].key,
                            swap_authorities[i].key,
                            &pool_state.vault_signer,
                            pool_vaults[i].key,
                            swap_assets[i].key,
                            swap_basic_asset.key,
                            basic_asset_vault.key,
                            swap_pool_token_mints[i].key,
                            swap_fees[i].key,
                            None,
                            spl_token_swap::instruction::Swap {
                                amount_in,
                                minimum_amount_out,
                            },
                        )
                        .map_err(|err| {
                            msg!("Create swap instruction error for token {}: {}", i, err);
                            err
                        })?;

                        let account_infos = vec![
                            token_swaps[i].clone(),
                            swap_authorities[i].clone(),
                            vault_signer.clone(),
                            pool_vaults[i].clone(),
                            swap_assets[i].clone(),
                            swap_basic_asset.clone(),
                            basic_asset_vault.clone(),
                            swap_pool_token_mints[i].clone(),
                            swap_fees[i].clone(),
                            spl_token_program.clone(),
                        ];

                        invoke_signed(
                            &swap_instruction,
                            &account_infos,
                            &[&[pool_account.key.as_ref(), &[pool_state.vault_signer_nonce]]],
                        )
                        .map_err(|err| {
                            msg!("Invoke swap error for token {}: {}", i, err);
                            err
                        })?;
                    } else if need_amount > amount + amount / fund_state.slippage_divider {
                        to_buy.push((i, amount, need_amount));
                    }
                }

                for (i, amount, need_amount) in to_buy {
                    msg!("To buy asset {}", i);

                    let amount_delta = need_amount - amount;
                    let asset_amount_delta = u64::try_from(
                        amount_delta as u128 * asset_vault_token_accounts[i].amount as u128 / amount as u128,
                    )
                    .map_err(|err| {
                        msg!("Buy amount_in overflowed: {}", err);
                        FundError::OperationOverflow
                    })?;

                    let swap_instruction = spl_token_swap::instruction::swap(
                        &spl_token_swap::id(),
                        &spl_token::id(),
                        token_swaps[i].key,
                        swap_authorities[i].key,
                        &pool_state.vault_signer,
                        basic_asset_vault.key,
                        swap_basic_asset.key,
                        swap_assets[i].key,
                        pool_vaults[i].key,
                        swap_pool_token_mints[i].key,
                        swap_fees[i].key,
                        None,
                        spl_token_swap::instruction::Swap {
                            amount_in: amount_delta,
                            minimum_amount_out: asset_amount_delta - asset_amount_delta / fund_state.slippage_divider,
                        },
                    )
                    .map_err(|err| {
                        msg!("Create swap instruction error for token {}: {}", i, err);
                        err
                    })?;

                    let account_infos = vec![
                        token_swaps[i].clone(),
                        swap_authorities[i].clone(),
                        vault_signer.clone(),
                        basic_asset_vault.clone(),
                        swap_basic_asset.clone(),
                        swap_assets[i].clone(),
                        pool_vaults[i].clone(),
                        swap_pool_token_mints[i].clone(),
                        swap_fees[i].clone(),
                        spl_token_program.clone(),
                    ];

                    invoke_signed(
                        &swap_instruction,
                        &account_infos,
                        &[&[pool_account.key.as_ref(), &[pool_state.vault_signer_nonce]]],
                    )
                    .map_err(|err| {
                        msg!("Invoke swap error for token {}: {}", i, err);
                        err
                    })?;
                }
            }
            FundInstructionInner::ApproveDelegate { amount } => {
                let vault_account = next_account_info(accounts_iter)?;
                let delegate_account = next_account_info(accounts_iter)?;
                let vault_signer_account = next_account_info(accounts_iter)?;
                let spl_token_program = next_account_info(accounts_iter)?;

                let asset = pool_state
                    .assets
                    .iter()
                    .find(|asset| asset.vault_address.as_ref() == vault_account.key)
                    .ok_or_else(|| {
                        msg!("Asset not found");
                        ProgramError::InvalidArgument
                    })?;
                if vault_signer_account.key != pool_state.vault_signer.as_ref() {
                    msg!("Incorrect vault signer account");
                    return Err(ProgramError::InvalidArgument);
                }
                if spl_token_program.key != &spl_token::ID {
                    msg!("Incorrect spl-token program ID");
                    return Err(ProgramError::InvalidArgument);
                }

                fund_state.paused = true;

                let instruction = spl_token::instruction::approve(
                    &spl_token::ID,
                    asset.vault_address.as_ref(),
                    delegate_account.key,
                    pool_state.vault_signer.as_ref(),
                    &[],
                    *amount,
                )?;
                let account_infos = &[
                    vault_account.clone(),
                    delegate_account.clone(),
                    vault_signer_account.clone(),
                    spl_token_program.clone(),
                ];
                invoke_signed(
                    &instruction,
                    account_infos,
                    &[&[pool_account.key.as_ref(), &[pool_state.vault_signer_nonce]]],
                )?;
            }
            FundInstructionInner::UpdateFee { fee_rate } => {
                pool_state.fee_rate = *fee_rate;
                if pool_state.fee_rate < MIN_FEE_RATE {
                    msg!("Fee too low");
                    return Err(ProgramError::InvalidArgument);
                }
                if pool_state.fee_rate >= FEE_RATE_DENOMINATOR {
                    msg!("Fee too high");
                    return Err(ProgramError::InvalidArgument);
                }
            }
            FundInstructionInner::UpdateAdmin => {
                let new_admin_account = next_account_info(accounts_iter)?;
                if !new_admin_account.is_signer {
                    msg!("New admin account not signer");
                    return Err(ProgramError::MissingRequiredSignature);
                }
                pool_state.admin_key = Some(new_admin_account.key.into());
            }
        };

        pool_state.write_fund_state(&fund_state)?;

        Ok(())
    }
}

fn parse_token_account(account_info: &AccountInfo) -> Result<TokenAccount, ProgramError> {
    if account_info.owner != &spl_token::ID {
        msg!("Account not owned by spl-token program");
        return Err(ProgramError::IncorrectProgramId);
    }
    let parsed = TokenAccount::unpack(&account_info.try_borrow_data()?)?;
    if !parsed.is_initialized() {
        msg!("Token account not initialized");
        return Err(ProgramError::UninitializedAccount);
    }
    Ok(parsed)
}
