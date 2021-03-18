use anyhow::Result;
use borsh::de::BorshDeserialize;
use fund::state::FundState;
use serum_pool::schema::PoolState;
pub use solana_client_helpers::Client;
use solana_program::pubkey::Pubkey;

pub trait FundClient {
    fn get_fund_state(&self, fund_account: &Pubkey) -> Result<(PoolState, FundState)>;
}

impl FundClient for Client {
    fn get_fund_state(&self, fund_account: &Pubkey) -> Result<(PoolState, FundState)> {
        let fund_account = self.get_account(fund_account)?;

        let mut data = fund_account.data.as_slice();
        let pool_state: PoolState = BorshDeserialize::deserialize(&mut data)?;

        data = pool_state.custom_state.as_slice();
        let fund_state: FundState = BorshDeserialize::deserialize(&mut data)?;

        Ok((pool_state, fund_state))
    }
}
