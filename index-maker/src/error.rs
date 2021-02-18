use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum IndexError {
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Not rent exempt")]
    NotRentExempt,

    #[error("Name is not in the UTF-8 encoding")]
    DescriptionIsNotInUtf8,
}

impl From<IndexError> for ProgramError {
    fn from(err: IndexError) -> Self {
        ProgramError::Custom(err as u32)
    }
}
