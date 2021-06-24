use borsh::{BorshSerialize, BorshDeserialize};
use futurecontract::{process_instruction, types};
use solana_program_test::*;
use solana_program::{
    msg
  };
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};
use std::collections::BTreeMap;
use std::mem;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::test]
async fn test_futurecontract() {
    let program_id = Pubkey::new_unique();
    let receiver_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "futurecontract", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );

    let data = types::DataAccount {
        futures: BTreeMap::new(),
        tokens: BTreeMap::new()
    };
    let encoded_data = data.try_to_vec().unwrap();

    program_test.add_account(
        receiver_pubkey,
        Account {
            lamports: 100,
            data: encoded_data,
            owner: program_id,
            ..Account::default()
        },
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Verify account has zero greetings
    let receiver_account = banks_client
        .get_account(receiver_pubkey)
        .await
        .expect("get_account")
        .expect("receiver_account not found");
    
    let decoded_data = types::DataAccount::try_from_slice(&receiver_account.data).unwrap();
    assert_eq!(decoded_data, data);

    msg!("{}", "this line will fail");

    // Greet once
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[0], // ignored but makes the instruction unique in the slot
            vec![AccountMeta::new(receiver_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    assert!(banks_client.process_transaction(transaction).await.is_ok());

    // // Verify account has one greeting
    // let receiver_account = banks_client
    //     .get_account(receiver_pubkey)
    //     .await
    //     .expect("get_account")
    //     .expect("receiver_account not found");
    // assert_eq!(
    //     types::TestData::try_from_slice(&receiver_account.data)
    //         .unwrap()
    //         .counter,
    //     1
    // );

    // // Greet again
    // let mut transaction = Transaction::new_with_payer(
    //     &[Instruction::new_with_bincode(
    //         program_id,
    //         &[1], // ignored but makes the instruction unique in the slot
    //         vec![AccountMeta::new(receiver_pubkey, false)],
    //     )],
    //     Some(&payer.pubkey()),
    // );
    // transaction.sign(&[&payer], recent_blockhash);
    // banks_client.process_transaction(transaction).await.unwrap();

    // // Verify account has two greetings
    // let receiver_account = banks_client
    //     .get_account(receiver_pubkey)
    //     .await
    //     .expect("get_account")
    //     .expect("receiver_account not found");
    // assert_eq!(
    //     types::TestData::try_from_slice(&receiver_account.data)
    //         .unwrap()
    //         .counter,
    //     2
    // );
}
