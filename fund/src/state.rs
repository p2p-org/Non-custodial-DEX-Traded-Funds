use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use serum_pool::schema::{declare_tag, PoolState};
use solana_program::{msg, program_error::ProgramError};

declare_tag!(FundStateTag, u64, 0x5b2cd8a87a04f13e);

#[derive(Clone, PartialEq, Eq, Debug, BorshSerialize, BorshDeserialize, BorshSchema, Default)]
pub struct FundState {
    pub tag: FundStateTag,
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
