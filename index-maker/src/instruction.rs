use std::convert::TryInto;

use solana_program::{program_error::ProgramError, pubkey::Pubkey};

use crate::{error::IndexError, state::Formula, util::Pack};

#[derive(Debug, PartialEq)]
pub enum IndexInstruction {
    /// Initializes a new index
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the creation index
    /// 1. `[writable]` The index account, it will hold all necessary info about the index
    /// 2. `[]` The rent sysvar
    CreateIndex {
        fee: u64,
        formula: Formula,
        tokens: Vec<Pubkey>,
        description: String,
    },
}

impl IndexInstruction {
    /// Pack [IndexInstruction] into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        match self {
            Self::CreateIndex {
                formula,
                tokens,
                fee,
                description,
            } => {
                let mut buf = Vec::with_capacity(1 + 8 + formula.len() + 4 + tokens.len() * 32 + 4 + description.len());

                buf.push(0);
                buf.extend_from_slice(&fee.to_le_bytes());
                formula.extend(&mut buf);

                buf.extend_from_slice(&(tokens.len() as u32).to_le_bytes());
                for token in tokens {
                    buf.extend_from_slice(token.as_ref());
                }

                buf.extend_from_slice(&(description.len() as u32).to_le_bytes());
                buf.extend_from_slice(description.as_bytes());
                buf
            }
        }
    }

    /// Unpacks a byte buffer into a [IndexInstruction].
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, src) = input.split_first().ok_or(IndexError::InvalidInstruction)?;
        Ok(match tag {
            0 => {
                let (fee_src, src) = src.split_at(8);
                let fee = u64::from_le_bytes(fee_src.try_into().map_err(|_| IndexError::InvalidInstruction)?);

                let formula = Formula::unpack(src)?;
                let src = &src[formula.len()..];

                let (tokens_len_src, mut src) = src.split_at(4);
                let tokens_len =
                    u32::from_le_bytes(tokens_len_src.try_into().map_err(|_| IndexError::InvalidInstruction)?) as usize;

                let mut tokens = Vec::with_capacity(tokens_len * 34);
                for _ in 0..tokens_len {
                    let token =
                        Pubkey::new_from_array(src[..32].try_into().map_err(|_| ProgramError::InvalidAccountData)?);
                    tokens.push(token);
                    src = &src[32..];
                }

                Self::CreateIndex {
                    fee,
                    formula,
                    tokens,
                    description: Self::unpack_description(src)?,
                }
            }
            _ => return Err(IndexError::InvalidInstruction.into()),
        })
    }

    fn unpack_description(input: &[u8]) -> Result<String, ProgramError> {
        let (description_len_src, input) = input.split_at(4);

        let description_len = u32::from_le_bytes(
            description_len_src
                .try_into()
                .map_err(|_| IndexError::InvalidInstruction)?,
        ) as usize;

        let description = std::str::from_utf8(&input[..description_len])
            .map_err(|_| IndexError::DescriptionIsNotInUtf8)?
            .to_string();

        Ok(description)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Operation::*;

    #[test]
    fn instruction_pack_unpack() {
        let source = IndexInstruction::CreateIndex {
            fee: 25,
            formula: Formula(vec![]),
            tokens: vec![],
            description: "Some description text".to_string(),
        };
        let data = source.pack();

        let instruction = IndexInstruction::unpack(&data).unwrap();
        assert_eq!(instruction, source);

        let source = IndexInstruction::CreateIndex {
            fee: 25,
            formula: Formula(vec![
                OpenPar,
                Var(0),
                Plus,
                Var(1),
                ClosePar,
                Mul,
                Number(0.11),
                Plus,
                Indicator(0, 0),
            ]),
            tokens: vec![Pubkey::new_unique()],
            description: "Some description text".to_string(),
        };
        let data = source.pack();

        let instruction = IndexInstruction::unpack(&data).unwrap();
        assert_eq!(instruction, source);
    }
}
