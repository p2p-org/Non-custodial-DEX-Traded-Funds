use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{
    error::IndexError,
    instruction::IndexInstruction,
    state::{Formula, Index, Operation},
    util::Pack,
};

pub struct Processor;

impl Processor {
    // todo: use real fee account pubkey
    pub const FEE_ACCOUNT_PUBKEY: Pubkey = Pubkey::new_from_array([0; 32]);
    pub const ROOT_DOMAIN_PUBKEY: Pubkey = Pubkey::new_from_array([0; 32]);

    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = IndexInstruction::unpack(instruction_data)?;

        msg!("Instruction: {:?}", instruction);
        match instruction {
            IndexInstruction::CreateIndex {
                fee,
                formula,
                tokens,
                description,
            } => Self::process_create_index(accounts, fee, formula, tokens, description, program_id),
        }
    }

    fn process_create_index(
        accounts: &[AccountInfo],
        fee: u64,
        formula: Formula,
        tokens: Vec<Pubkey>,
        description: String,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let initializer_account = check_initializer_account(next_account_info(account_info_iter)?)?;
        let index_account = check_index_account(program_id, next_account_info(account_info_iter)?)?;
        let _rent = get_rent(index_account, next_account_info(account_info_iter)?)?;

        for operation in formula.iter().copied() {
            if let Operation::Indicator(_, token_idx) = operation {
                if token_idx as usize >= tokens.len() {
                    msg!(
                        "Formula uses unknown token index = {}, max is {}",
                        token_idx,
                        tokens.len()
                    );
                    return Err(IndexError::UnknownTokenIndex.into());
                }
            }
        }

        pack_index(
            index_account,
            *initializer_account.key,
            fee,
            formula,
            tokens,
            description,
        )?;

        Ok(())
    }
}

fn check_initializer_account<'a, 'b>(
    initializer_account: &'a AccountInfo<'b>,
) -> Result<&'a AccountInfo<'b>, ProgramError> {
    if !initializer_account.is_signer {
        Err(ProgramError::MissingRequiredSignature)
    } else {
        Ok(initializer_account)
    }
}

fn check_index_account<'a, 'b>(
    program_id: &Pubkey,
    index_account: &'a AccountInfo<'b>,
) -> Result<&'a AccountInfo<'b>, ProgramError> {
    if index_account.owner != program_id {
        Err(ProgramError::IncorrectProgramId)
    } else {
        Ok(index_account)
    }
}

fn get_rent(index_account: &AccountInfo, rent: &AccountInfo) -> Result<Rent, ProgramError> {
    let rent = Rent::from_account_info(rent)?;

    // todo: calc rent if not exempt
    if !rent.is_exempt(index_account.lamports(), index_account.data_len()) {
        Err(IndexError::NotRentExempt.into())
    } else {
        Ok(rent)
    }
}

fn pack_index(
    index_account: &AccountInfo,
    owner: Pubkey,
    fee: u64,
    formula: Formula,
    tokens: Vec<Pubkey>,
    description: String,
) -> ProgramResult {
    let mut index = Index::unpack(&index_account.data.borrow())?;
    if index.is_initialized() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    index.is_initialized = true;
    index.owner = owner;
    index.fee = fee;
    index.formula = formula;
    index.tokens = tokens;
    index.description = description;

    index.pack(&mut index_account.data.borrow_mut())
}
