use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use serum_pool::schema::{Address, AssetInfo, PoolState};
use solana_program::{msg, program_error::ProgramError, pubkey::Pubkey};

#[derive(Clone, PartialEq, Eq, Debug, BorshSerialize, BorshDeserialize, BorshSchema, Default)]
pub struct FundState {
    pub paused: bool,
}

pub trait FundStateContainer {
    fn read_fund_state(&self) -> Result<FundState, ProgramError>;
    fn write_fund_state(&mut self, custom_state: &FundState) -> Result<(), ProgramError>;
}

impl FundStateContainer for PoolState {
    fn read_fund_state(&self) -> Result<FundState, ProgramError> {
        FundState::try_from_slice(&self.custom_state).map_err(|_| {
            msg!("Invalid fund state");
            ProgramError::InvalidAccountData
        })
    }

    fn write_fund_state(&mut self, fund_state: &FundState) -> Result<(), ProgramError> {
        self.custom_state = fund_state.try_to_vec().unwrap();
        Ok(())
    }
}

pub fn calc_len(name: impl Into<String>, assets_count: usize) -> usize {
    let default_address: Address = Pubkey::default().into();
    let mut state = PoolState {
        tag: Default::default(),
        pool_token_mint: default_address.clone(),
        assets: (0..assets_count)
            .map(|_| AssetInfo {
                mint: default_address.clone(),
                vault_address: default_address.clone(),
            })
            .collect(),
        vault_signer: default_address.clone(),
        vault_signer_nonce: 1,
        account_params: vec![],
        name: name.into(),
        lqd_fee_vault: default_address.clone(),
        initializer_fee_vault: default_address.clone(),
        fee_rate: 0,
        admin_key: Some(default_address),
        custom_state: vec![],
    };
    state
        .write_fund_state(&FundState::default())
        .expect("FundState should be writeable");
    state.try_to_vec().expect("PoolState should be serializable").len()
}
