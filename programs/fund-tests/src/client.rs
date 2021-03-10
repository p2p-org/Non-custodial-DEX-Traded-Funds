use std::ops::Deref;

use anyhow::Result;
use borsh::de::BorshDeserialize;
use fund::state::FundState;
use serum_pool::schema::PoolState;
use solana_client::rpc_client::RpcClient;
use solana_program::{hash::Hash, pubkey::Pubkey, system_instruction};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

pub struct Client {
    pub client: RpcClient,
    pub payer: Keypair,
}

impl Client {
    pub fn payer(&self) -> &Keypair {
        &self.payer
    }

    pub fn payer_pubkey(&self) -> Pubkey {
        self.payer.pubkey()
    }

    #[track_caller]
    pub fn recent_blockhash(&self) -> Hash {
        self.client.get_recent_blockhash().unwrap().0
    }

    #[track_caller]
    pub fn rent_minimum_balance(&mut self, data_len: usize) -> u64 {
        self.client.get_minimum_balance_for_rent_exemption(data_len).unwrap()
    }

    #[track_caller]
    pub fn process_transaction(&mut self, transaction: &Transaction) {
        self.client.send_and_confirm_transaction(transaction).unwrap();
    }

    pub fn create_account(&mut self, owner: &Pubkey, account_data_len: usize) -> Keypair {
        let account = Keypair::new();

        let mut transaction = Transaction::new_with_payer(
            &[system_instruction::create_account(
                &self.payer_pubkey(),
                &account.pubkey(),
                self.rent_minimum_balance(account_data_len),
                account_data_len as u64,
                owner,
            )],
            Some(&self.payer_pubkey()),
        );
        transaction.sign(&[self.payer(), &account], self.recent_blockhash());
        self.process_transaction(&transaction);
        account
    }

    pub fn get_fund_state(&self, fund_account: &Pubkey) -> Result<(PoolState, FundState)> {
        let fund_account = self.get_account(fund_account)?;

        let mut data = fund_account.data.as_slice();
        let pool_state: PoolState = BorshDeserialize::deserialize(&mut data)?;

        data = pool_state.custom_state.as_slice();
        let fund_state: FundState = BorshDeserialize::deserialize(&mut data)?;

        Ok((pool_state, fund_state))
    }
}

impl Deref for Client {
    type Target = RpcClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
