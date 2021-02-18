use solana_program::program_error::ProgramError;

pub trait Pack: Sized {
    const MIN_LEN: usize;

    fn len(&self) -> usize;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError>;

    fn pack_into_slice(&self, dst: &mut [u8]) -> Result<(), ProgramError>;

    fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.len() < Self::MIN_LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Self::unpack_from_slice(input)
    }

    fn pack(&self, dst: &mut [u8]) -> Result<(), ProgramError> {
        if dst.len() != self.len() {
            return Err(ProgramError::InvalidAccountData);
        }
        self.pack_into_slice(dst)
    }
}
