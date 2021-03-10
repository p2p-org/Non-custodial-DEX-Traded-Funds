use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use serum_pool::schema::declare_tag;

declare_tag!(FundRequestTag, u64, 0x112ea41452f06767);

/// Additional data for `PoolRequestInner::Initialize`.
///
/// Additional accounts:
///
/// - `[writable]` Fund admin account
/// - `[writable]` Initial supply fund token account
/// - `[writable]` Fund vault account of basic asset
/// - `[]` spl-token program ID
#[derive(Clone, PartialEq, Eq, Debug, Default, BorshSerialize, BorshDeserialize)]
pub struct InitializeFundData {
    pub slippage_divider: u64,
    pub asset_weights: Vec<u32>,
    pub fund_token_initial_supply: u64,
}

#[derive(Clone, PartialEq, Eq, Debug, BorshSerialize, BorshDeserialize)]
pub struct FundRequest {
    pub tag: FundRequestTag,
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

    /// Rebalances the fund assets.
    ///
    /// Accounts:
    ///
    /// - `[writable]` Pool account
    /// - `[signer]` Admin account
    /// - `[writable]` Pool vault account for each of the N pool assets
    /// - `[]` Pool vault authority
    /// - `[writable]` Fund vault account of basic asset
    /// For each of the N pool assets:
    ///   * `[]` Token-swap account
    ///   * `[]` Swap authority
    ///   * `[writable]` Swap asset account
    ///   * `[writable]` Swap basic asset account
    ///   * `[writable]` Swap pool token mint, to generate trading fees
    ///   * `[writable]` Swap fee account, to receive trading fees
    /// - `[]` spl-token program ID
    /// - `[]` spl-token-swap program ID
    Rebalance,

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
