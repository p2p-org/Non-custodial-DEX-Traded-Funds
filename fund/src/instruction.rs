use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use serum_pool::schema::declare_tag;

declare_tag!(FundInstructionTag, u64, 0x112ea41452f06767);

#[derive(Clone, PartialEq, Eq, Debug, Default, BorshSerialize, BorshDeserialize)]
pub struct InitializeFundData {
    pub token_initial_amount: u64,
}

#[derive(Clone, PartialEq, Eq, Debug, BorshSerialize, BorshDeserialize)]
pub struct FundInstruction {
    pub tag: FundInstructionTag,
    pub inner: FundInstructionInner,
}

#[derive(Clone, PartialEq, Eq, Debug, BorshSerialize, BorshDeserialize)]
pub enum FundInstructionInner {
    /// Pauses creations and redemptions for the pool.
    ///
    /// Accounts:
    ///
    /// - `[writable]` Pool account
    /// - `[signer]` Admin account
    Pause,

    /// Resumes creations and redemptions for the pool. Pool assets must not
    /// have any outstanding delegates.
    ///
    /// Accounts:
    ///
    /// - `[writable]` Pool account
    /// - `[signer]` Admin account
    /// - `[]` Pool vault account for each of the pool assets
    Unpause,

    /// Approves an account to spend tokens on behalf of the pool.
    ///
    /// Accounts:
    ///
    /// - `[writable]` Pool account
    /// - `[signer]` Admin account
    /// - `[writable]` Pool vault account for which to delegate access
    /// - `[]` Account to which to delegate
    /// - `[]` Pool vault signer
    /// - `[]` spl-token program ID
    ApproveDelegate { amount: u64 },

    /// Adds a new asset to the pool.
    ///
    /// Accounts:
    ///
    /// - `[writable]` Pool account
    /// - `[signer]` Admin account
    /// - `[]` Pool vault account for the new asset
    AddAsset,

    /// Removes an asset from the pool. The pool must not currently own any
    /// tokens of the asset to be removed.
    ///
    /// Accounts:
    ///
    /// - `[writable]` Pool account
    /// - `[signer]` Admin account
    /// - `[]` Pool vault account to remove
    RemoveAsset,

    /// Modifies the fee rate for the pool.
    ///
    /// Accounts:
    ///
    /// - `[writable]` Pool account
    /// - `[signer]` Admin account
    UpdateFee { fee_rate: u32 },

    /// Transfers admin permission for the pool to a new account.
    ///
    /// Accounts:
    ///
    /// - `[writable]` Pool account
    /// - `[signer]` Current admin account
    /// - `[signer]` New admin account
    UpdateAdmin,
}
