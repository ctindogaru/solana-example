use borsh::{BorshSerialize, BorshDeserialize};
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  msg,
  program_error::ProgramError,
  program_option::COption,
  pubkey::Pubkey
};

use crate::error::FutureContractError;
use crate::types;

use spl_token::instruction::TokenInstruction;
use spl_token::processor::Processor as SplTokenProcessor;

pub struct Processor { }

impl Processor {
  pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Receiver account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // // Increment and store the number of times the account has been greeted
    let mut receiver_account = types::DataAccount::try_from_slice(&account.data.borrow())?;
    // let new_future = types::FutureContract {
    //   locked_token_contract_address: Pubkey::new_unique(),
    //   future_expiration: 25,
    //   future_creation_date: 321,
    //   future_contract_address: Pubkey::new_unique()
    // };
    let new_testdata = types::DoingTestData {
      // future_expiration: 30,
      future_creation_date: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    };
    // receiver_account.futures.insert(Pubkey::new_unique(), new_future);
    // receiver_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
    // let instruction = TokenInstruction::unpack(input)?;

    // match instruction {
    //   TokenInstruction::InitializeAccount => {
    //     msg!("Instruction: InitializeAccount");
    //     Self::create_new_future_contract(accounts)
    //   }
    //   TokenInstruction::InitializeMint {
    //     decimals,
    //     mint_authority,
    //     freeze_authority,
    //   } => {
    //     msg!("Instruction: InitializeMint");
    //     Self::initialize_new_future_contract(accounts, decimals, mint_authority, freeze_authority)
    //   }
    //   TokenInstruction::MintTo { amount } => {
    //     msg!("Instruction: MintTo");
    //     Self::mint_into_future_contract(program_id, accounts, amount)
    //   }
    //   TokenInstruction::Burn { amount } => {
    //     msg!("Instruction: Burn");
    //     Self::burn_from_future_contract(program_id, accounts, amount)
    //   }
    //   _ => {
    //     msg!("Instruction: Not handled");
    //     return Err(FutureContractError::InstructionNotHandled.into())
    //   }
    // }
  }

  fn create_new_future_contract(
    accounts: &[AccountInfo]
  ) -> ProgramResult {
    // TODO: extract the tokens, futures and locked_token_contract_address from 'accounts'
    let mut tokens: BTreeMap<Pubkey, Vec<Pubkey>> = BTreeMap::new();
    let mut futures: BTreeMap<Pubkey, types::FutureContract> = BTreeMap::new();
    let locked_token_contract_address = Pubkey::new_unique();

    // check if the token has existing futures
    if tokens.get(&locked_token_contract_address).is_some() {
      let last_future = tokens.get(&locked_token_contract_address).unwrap().last().unwrap();
      let last_future_data = futures.get(&last_future).unwrap();

      let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
      let days_diff = (current_time - last_future_data.future_creation_date) / 86400; // 1day = 86400s
      if days_diff < 30 {
        return Err(FutureContractError::FutureCooldownNotElapsed.into())
      }
    }

    // TODO: change this to be the address of the future contract
    let future_contract_address = Pubkey::new_unique();
    // TODO: change this to be the future expiration
    let future_expiration = 20;

    let new_future = types::FutureContract {
      locked_token_contract_address: locked_token_contract_address,
      future_expiration: future_expiration,
      future_creation_date: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
      future_contract_address: future_contract_address
    };

    if tokens.get(&locked_token_contract_address).is_none() {
      tokens.insert(
        locked_token_contract_address,
        Vec::new()
      );
    }
    tokens.get_mut(&locked_token_contract_address).unwrap().push(future_contract_address);

    futures.insert(
      future_contract_address,
      new_future
    );

    // TODO: include useful data in the log (ex: accounts)
    msg!("CREATE instruction triggered");
    SplTokenProcessor::process_initialize_account(accounts)
  }

  fn initialize_new_future_contract(
    accounts: &[AccountInfo],
    decimals: u8,
    mint_authority: Pubkey,
    freeze_authority: COption<Pubkey>,
  ) -> ProgramResult {
    // TODO: include useful data in the log (ex: accounts, mint_authority, freeze_authority)
    msg!("INITIALIZE instruction triggered");
    SplTokenProcessor::process_initialize_mint(accounts, decimals, mint_authority, freeze_authority)
  }

  fn mint_into_future_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64
  ) -> ProgramResult {
    // TODO: include useful data in the log (ex: program_id, accounts, amount)
    msg!("MINT instruction triggered");
    SplTokenProcessor::process_mint_to(program_id, accounts, amount, None)
  }

  fn burn_from_future_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64
  ) -> ProgramResult {
    // TODO: replace this with the actual future contract address
    // (it should be part of the 'accounts' variable)
    let future_contract_address = Pubkey::new_unique();
    // TODO: replace this with the futures data stored in 'accounts'
    // (it should be part of the 'accounts' variable)
    let futures: BTreeMap<Pubkey, types::FutureContract> = BTreeMap::new();

    let future_data = futures.get(&future_contract_address).unwrap();

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let days_diff = (current_time - future_data.future_creation_date) / 86400; // 1day = 86400s
    if days_diff < future_data.future_expiration as u64 {
      return Err(FutureContractError::MaturityDateNotReached.into());
    }

    // TODO: include useful data in the log (ex: program_id, accounts, amount)
    msg!("BURN instruction triggered");
    SplTokenProcessor::process_burn(program_id, accounts, amount, None)
  }
}
