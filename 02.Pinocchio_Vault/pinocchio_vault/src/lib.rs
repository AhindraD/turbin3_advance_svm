use instructions::VaultInstruction;
use pinocchio::{
    ProgramResult, account_info::AccountInfo, entrypoint, program_error::ProgramError,
    pubkey::Pubkey,
};
use pinocchio_log::log;

pub mod instructions;
pub mod state;

use pinocchio_pubkey::declare_id;
declare_id!("AHibwaXG2EVnD1jTvD93166tunfNecvMcVQhcQKL3UZv");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts_data: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    //extracting Discriminator & All rest of Instructions
    let (ix_disc, instructions) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match VaultInstruction::try_from(ix_disc)? {
        VaultInstruction::DEPOSIT => {
            log!("Ix:0 -> Deposit");
        }
        VaultInstruction::WITHDRAW => {
            log!("Ix:1 -> Withdraw");
        }
        _ => Err(ProgramError::InvalidInstructionData)?,
    };
    Ok(())
}
