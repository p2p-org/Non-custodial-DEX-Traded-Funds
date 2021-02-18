use std::convert::TryInto;

use solana_program::program_error::ProgramError;

use crate::{error::IndexError, state::Formula, util::Pack};

#[derive(Debug, PartialEq)]
pub enum IndexInstruction {
    /// Initialize the new index
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
        description: String,
    },
}

impl IndexInstruction {
    /// Pack [IndexInstruction] into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        match self {
            Self::CreateIndex {
                formula,
                fee,
                description,
            } => {
                let mut buf = Vec::with_capacity(1 + 8 + formula.len() + 4 + description.len());

                buf.push(0);
                buf.extend_from_slice(&fee.to_le_bytes());
                formula.extend(&mut buf);
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

                Self::CreateIndex {
                    fee,
                    formula,
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
            description: "Some description text".to_string(),
        };
        let data = source.pack();

        let instruction = IndexInstruction::unpack(&data).unwrap();
        assert_eq!(instruction, source);

        let source = IndexInstruction::CreateIndex {
            fee: 25,
            formula: Formula(vec![OpenPar, Var(0), Plus, Var(1), ClosePar, Mul, Number(0.11)]),
            description: "Some description text".to_string(),
        };
        let data = source.pack();

        let instruction = IndexInstruction::unpack(&data).unwrap();
        assert_eq!(instruction, source);
    }
}
