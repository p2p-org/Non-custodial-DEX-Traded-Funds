use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use serum_pool::schema::{AssetInfo, PoolState};
use solana_program::{msg, program_error::ProgramError};

#[derive(Clone, PartialEq, Eq, Debug, BorshSerialize, BorshDeserialize, BorshSchema, Default)]
pub struct FundState {
    pub paused: bool,
    pub slippage_divider: u64,
    pub asset_weights: Vec<u32>,
    pub basic_asset: AssetInfo,
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
    let mut state = PoolState {
        tag: Default::default(),
        pool_token_mint: Default::default(),
        assets: (0..assets_count).map(|_| Default::default()).collect(),
        vault_signer: Default::default(),
        vault_signer_nonce: 1,
        account_params: vec![],
        name: name.into(),
        lqd_fee_vault: Default::default(),
        initializer_fee_vault: Default::default(),
        fee_rate: 0,
        admin_key: Some(Default::default()),
        custom_state: vec![],
    };
    state
        .write_fund_state(&FundState {
            paused: false,
            slippage_divider: 100,
            asset_weights: vec![1; assets_count],
            basic_asset: Default::default(),
        })
        .expect("FundState should be writeable");
    state.try_to_vec().expect("PoolState should be serializable").len()
}
