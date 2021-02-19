use std::convert::TryInto;

use solana_program::{program_error::ProgramError, program_pack::IsInitialized, pubkey::Pubkey};

use crate::{error::IndexError, state::Formula, util::Pack};

#[derive(Debug)]
pub struct Index {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub fee: u64,
    pub formula: Formula,
    pub tokens: Vec<Pubkey>,
    pub description: String,
}

impl Index {
    pub const STATIC_DATA_LEN: usize = 1 + 32 + 8;

    pub const fn calc_len(formula_len: usize, tokens_count: usize, description_len: usize) -> usize {
        Self::STATIC_DATA_LEN + formula_len + 4 + tokens_count * 32 + 4 + description_len
    }
}

impl Pack for Index {
    const MIN_LEN: usize = Self::STATIC_DATA_LEN + Formula::MIN_LEN + 4 + 4;

    fn len(&self) -> usize {
        Self::calc_len(self.formula.len(), self.tokens.len(), self.description.len())
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let (is_initialized_src, src) = src.split_at(1);
        let is_initialized = match is_initialized_src {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let (owner_src, src) = src.split_at(32);
        let owner = Pubkey::new_from_array(owner_src.try_into().map_err(|_| ProgramError::InvalidAccountData)?);

        let (fee_src, src) = src.split_at(8);
        let fee = u64::from_le_bytes(fee_src.try_into().map_err(|_| ProgramError::InvalidAccountData)?);

        let formula = Formula::unpack(src)?;
        let src = &src[formula.len()..];

        let (token_count_src, mut src) = src.split_at(4);
        let token_count = u32::from_le_bytes(
            token_count_src
                .try_into()
                .map_err(|_| ProgramError::InvalidAccountData)?,
        );

        let mut tokens = Vec::with_capacity(token_count as usize * 32);
        for _ in 0..token_count {
            let token = Pubkey::new_from_array(src[..32].try_into().map_err(|_| ProgramError::InvalidAccountData)?);
            tokens.push(token);
            src = &src[32..];
        }

        let (description_len_src, src) = src.split_at(4);
        let description_len = u32::from_le_bytes(
            description_len_src
                .try_into()
                .map_err(|_| ProgramError::InvalidAccountData)?,
        ) as usize;
        let description =
            std::str::from_utf8(&src[..description_len]).map_err(|_| IndexError::DescriptionIsNotInUtf8)?;

        Ok(Self {
            is_initialized,
            owner,
            fee,
            formula,
            tokens,
            description: description.to_string(),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) -> Result<(), ProgramError> {
        let Self {
            is_initialized,
            owner,
            fee,
            formula,
            tokens,
            description,
        } = self;

        let (is_initialized_dst, dst) = dst.split_at_mut(1);
        is_initialized_dst[0] = *is_initialized as u8;

        let (owner_dst, dst) = dst.split_at_mut(32);
        owner_dst.copy_from_slice(owner.as_ref());

        let (fee_dst, dst) = dst.split_at_mut(8);
        fee_dst.copy_from_slice(&fee.to_le_bytes());

        let (formula_dst, dst) = dst.split_at_mut(formula.len());
        formula.pack(formula_dst)?;

        let (token_count_dst, mut dst) = dst.split_at_mut(4);
        token_count_dst.copy_from_slice(&(tokens.len() as u32).to_le_bytes());

        for token in tokens {
            dst[..32].copy_from_slice(token.as_ref());
            dst = &mut dst[32..];
        }

        let (description_len_dst, description_buf_dst) = dst.split_at_mut(4);

        description_len_dst.copy_from_slice(&(description.len() as u32).to_le_bytes());
        description_buf_dst.copy_from_slice(description.as_bytes());
        Ok(())
    }
}

impl IsInitialized for Index {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
