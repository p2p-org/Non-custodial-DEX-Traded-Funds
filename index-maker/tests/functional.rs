use std::borrow::Borrow;

use index_maker::{
    instruction::IndexInstruction,
    processor::Processor,
    state::{Formula, Index, Operation::*},
    util::Pack,
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_instruction, sysvar,
};
use solana_program_test::{processor, ProgramTest};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

#[tokio::test]
async fn test_create_index() {
    let program_id = Pubkey::new_unique();
    let (mut banks_client, payer, recent_blockhash) =
        ProgramTest::new("index_maker", program_id, processor!(Processor::process))
            .start()
            .await;

    let initializer_account = Keypair::new();
    let fee = 10;
    let formula = Formula(vec![
        OpenPar,
        Indicator(0, 0),
        Mul,
        Indicator(1, 0),
        Plus,
        Indicator(0, 1),
        Mul,
        Indicator(1, 1),
        ClosePar,
        Div,
        Number(0.123),
    ]);
    let tokens = vec![Pubkey::new_unique(), Pubkey::new_unique()];
    let description = "DeFi Gainers (DFG) index by Alice";

    let index_account = Keypair::new();
    let index_account_data_len = Index::calc_len(formula.len(), tokens.len(), description.len());
    let rent = banks_client.get_rent().await.unwrap();
    let index_account_rent = rent.minimum_balance(index_account_data_len);

    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &index_account.pubkey(),
                index_account_rent,
                index_account_data_len as u64,
                &program_id,
            ),
            Instruction {
                program_id,
                accounts: vec![
                    AccountMeta::new_readonly(initializer_account.pubkey(), true),
                    AccountMeta::new(index_account.pubkey(), false),
                    AccountMeta::new_readonly(sysvar::rent::id(), false),
                ],
                data: IndexInstruction::CreateIndex {
                    fee,
                    formula: formula.clone(),
                    tokens: tokens.clone(),
                    description: description.to_string(),
                }
                .pack(),
            },
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &initializer_account, &index_account], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let index_account = banks_client
        .get_account(index_account.pubkey())
        .await
        .unwrap()
        .expect("Index account should be created");

    assert_eq!(index_account.owner, program_id);
    assert_eq!(index_account.executable, false);

    let index = Index::unpack(&index_account.data.borrow()).unwrap();
    assert!(index.is_initialized);
    assert_eq!(index.owner, initializer_account.pubkey());
    assert_eq!(index.fee, 10);
    assert_eq!(index.formula, formula);
    assert_eq!(index.description.as_str(), description);
}
