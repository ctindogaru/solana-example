use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::BTreeMap;
use solana_program::{
  pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct FutureContract {
  pub locked_token_contract_address: Pubkey, // the contract address of the token that's going to be locked in the future
  pub future_expiration: u16, // when is the future set to expire (in days)
  pub future_creation_date: u64, // creation date of the future as a unix timestamp
  pub future_contract_address: Pubkey // the contract address of the future
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct DataAccount {
  // locked token contract address to all its associated futures
  pub tokens: BTreeMap<Pubkey, Vec<Pubkey>>,
  // future contract address to future contract data
  pub futures: BTreeMap<Pubkey, FutureContract>
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct TestData {
  pub test_string: String,
  pub counter: i32
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct DoingTestData {
  // pub locked_token_contract_address: Pubkey, // the contract address of the token that's going to be locked in the future
  // pub future_expiration: u16, // when is the future set to expire (in days)
  pub future_creation_date: u64, // creation date of the future as a unix timestamp
  // pub future_contract_address: Pubkey 
}
