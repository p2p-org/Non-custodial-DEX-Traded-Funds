use std::convert::TryInto;

use solana_program::program_error::ProgramError;

use crate::util::Pack;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operation {
    Number(f64),
    Plus,
    Minus,
    Mul,
    Div,
    Pow,
    OpenPar,
    ClosePar,
    Var(u32),
    // Indicator { id: u32, token_idx: u32 },
}

impl Operation {
    pub fn to_bytes(&self) -> OperationBytes {
        match self {
            Operation::Number(number) => {
                let mut bytes = [0; 9];
                bytes[1..9].copy_from_slice(&number.to_le_bytes());
                OperationBytes::Nine(bytes)
            }
            Operation::Plus => OperationBytes::One([1]),
            Operation::Minus => OperationBytes::One([2]),
            Operation::Mul => OperationBytes::One([3]),
            Operation::Div => OperationBytes::One([4]),
            Operation::Pow => OperationBytes::One([5]),
            Operation::OpenPar => OperationBytes::One([6]),
            Operation::ClosePar => OperationBytes::One([7]),
            Operation::Var(index) => {
                let mut bytes = [0; 5];
                bytes[0] = 8;
                bytes[1..5].copy_from_slice(&index.to_le_bytes());
                OperationBytes::Five(bytes)
            }
        }
    }
}

impl Pack for Operation {
    const MIN_LEN: usize = 1;

    fn len(&self) -> usize {
        self.to_bytes().len()
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let (tag, src) = src.split_at(1);
        match tag[0] {
            0 => src[..8]
                .try_into()
                .map(f64::from_le_bytes)
                .map(Self::Number)
                .map_err(|_| ProgramError::InvalidAccountData),
            1 => Ok(Self::Plus),
            2 => Ok(Self::Minus),
            3 => Ok(Self::Mul),
            4 => Ok(Self::Div),
            5 => Ok(Self::Pow),
            6 => Ok(Self::OpenPar),
            7 => Ok(Self::ClosePar),
            8 => src[..4]
                .try_into()
                .map(u32::from_le_bytes)
                .map(Self::Var)
                .map_err(|_| ProgramError::InvalidAccountData),
            _ => Err(ProgramError::InvalidAccountData),
        }
    }

    fn pack_into_slice(&self, dst: &mut [u8]) -> Result<(), ProgramError> {
        let bytes = self.to_bytes();
        let slice = bytes.as_slice();
        dst[..slice.len()].copy_from_slice(slice);
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum OperationBytes {
    One([u8; 1]),
    Five([u8; 5]),
    Nine([u8; 9]),
}

impl OperationBytes {
    pub fn len(&self) -> usize {
        match self {
            OperationBytes::One(bytes) => bytes.len(),
            OperationBytes::Five(bytes) => bytes.len(),
            OperationBytes::Nine(bytes) => bytes.len(),
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        match self {
            OperationBytes::One(bytes) => &bytes[..],
            OperationBytes::Five(bytes) => &bytes[..],
            OperationBytes::Nine(bytes) => &bytes[..],
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Formula(pub Vec<Operation>);

impl Formula {
    pub fn extend(&self, dst: &mut Vec<u8>) {
        dst.extend_from_slice(&(self.0.len() as u32).to_le_bytes());
        for operation in &self.0 {
            dst.extend_from_slice(operation.to_bytes().as_slice());
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.extend(&mut bytes);
        bytes
    }
}

impl Pack for Formula {
    const MIN_LEN: usize = 4;

    fn len(&self) -> usize {
        Self::MIN_LEN + self.0.iter().fold(0, |len, operation| len + operation.len())
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let (operation_count_src, src) = src.split_at(4);

        let operations_count = u32::from_le_bytes(
            operation_count_src
                .try_into()
                .map_err(|_| ProgramError::InvalidAccountData)?,
        ) as usize;

        let mut operations = Vec::new();
        let mut offset = 0;

        for _ in 0..operations_count {
            let operation = Operation::unpack(&src[offset..])?;
            offset += operation.len();
            operations.push(operation);
        }

        Ok(Self(operations))
    }

    fn pack_into_slice(&self, dst: &mut [u8]) -> Result<(), ProgramError> {
        let (operation_count_dst, mut dst) = dst.split_at_mut(4);

        operation_count_dst.copy_from_slice(&(self.0.len() as u32).to_le_bytes());

        for operation in &self.0 {
            operation.pack(&mut dst[..operation.len()])?;
            dst = &mut dst[operation.len()..];
        }
        Ok(())
    }
}
