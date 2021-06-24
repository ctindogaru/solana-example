use solana_program::{
    account_info::AccountInfo,
    entrypoint, entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

pub mod error;
pub mod processor;
pub mod types;
use processor::Processor;

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        msg!("Error: {}", error);
        return Err(error);
    }
    Ok(())
}
