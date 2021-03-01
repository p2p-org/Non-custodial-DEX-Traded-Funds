use solana_program::{hash::Hash, pubkey::Pubkey, system_instruction};
use solana_program_test::BanksClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    transport,
};

pub struct TestClient {
    pub client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhash: Hash,
}

impl TestClient {
    pub fn payer(&self) -> &Keypair {
        &self.payer
    }

    pub fn payer_pubkey(&self) -> Pubkey {
        self.payer.pubkey()
    }

    pub fn recent_blockhash(&self) -> Hash {
        self.recent_blockhash
    }

    pub async fn rent_minimum_balance(&mut self, data_len: usize) -> u64 {
        let rent = self.client.get_rent().await.unwrap();
        rent.minimum_balance(data_len)
    }

    pub async fn process_transaction(&mut self, transaction: Transaction) -> transport::Result<()> {
        self.client.process_transaction(transaction).await
    }

    pub async fn create_account(&mut self, owner: &Pubkey, account_data_len: usize) -> Keypair {
        let account = Keypair::new();

        let mut transaction = Transaction::new_with_payer(
            &[system_instruction::create_account(
                &self.payer_pubkey(),
                &account.pubkey(),
                self.rent_minimum_balance(account_data_len).await,
                account_data_len as u64,
                owner,
            )],
            Some(&self.payer_pubkey()),
        );
        transaction.sign(&[self.payer(), &account], self.recent_blockhash());
        self.process_transaction(transaction).await.unwrap();
        account
    }
}

impl From<(BanksClient, Keypair, Hash)> for TestClient {
    fn from((client, payer, recent_blockhash): (BanksClient, Keypair, Hash)) -> Self {
        Self {
            client,
            payer,
            recent_blockhash,
        }
    }
}
