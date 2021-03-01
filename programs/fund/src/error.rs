use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum FundError {
    #[error("Operation overflow")]
    OperationOverflow,
}

impl From<FundError> for ProgramError {
    fn from(err: FundError) -> Self {
        ProgramError::Custom(err as u32)
    }
}
