use borsh::{BorshDeserialize, BorshSerialize};
use serum_pool::{
    context::check_token_account,
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
    instruction::{FundInstruction, FundInstructionInner, InitializeFundData},
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

        let instruction: FundInstructionInner = FundInstruction::try_from_slice(instruction_data)
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
        let mut custom_state = pool_state.read_fund_state()?;

        match request {
            FundInstructionInner::Pause => {
                custom_state.paused = true;
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
                custom_state.paused = false;
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

                custom_state.paused = true;

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
            FundInstructionInner::AddAsset => {
                let vault_account = next_account_info(accounts_iter)?;

                let parsed_vault_account = parse_token_account(vault_account)?;
                if pool_state
                    .assets
                    .iter()
                    .find(|asset| {
                        asset.vault_address.as_ref() == vault_account.key
                            || asset.mint.as_ref() == &parsed_vault_account.mint
                    })
                    .is_some()
                {
                    msg!("Asset already in fund");
                    return Err(ProgramError::InvalidArgument);
                }
                if &parsed_vault_account.owner != pool_state.vault_signer.as_ref() {
                    msg!("Token account not owned by fund");
                    return Err(ProgramError::InvalidArgument);
                }

                pool_state.assets.push(AssetInfo {
                    mint: parsed_vault_account.mint.into(),
                    vault_address: vault_account.key.into(),
                });
            }
            FundInstructionInner::RemoveAsset => {
                let vault_account = next_account_info(accounts_iter)?;
                let parsed_vault_account = parse_token_account(vault_account)?;
                if parsed_vault_account.amount > 0 {
                    msg!("Vault not empty");
                    return Err(ProgramError::InvalidArgument);
                }
                let original_len = pool_state.assets.len();
                pool_state
                    .assets
                    .retain(|asset| asset.vault_address.as_ref() != vault_account.key);
                if pool_state.assets.len() == original_len {
                    msg!("Asset not found");
                    return Err(ProgramError::InvalidArgument);
                }
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

        pool_state.write_fund_state(&custom_state)?;

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
