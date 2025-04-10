use pinocchio::{
    ProgramResult, account_info::AccountInfo, entrypoint, program_error::ProgramError,
    pubkey::Pubkey,
};

pub mod state;

entrypoint!(process_instruction);

use pinocchio_pubkey::declare_id;
declare_id!("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (ix_disc, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    match ix_disc {
        0 => todo!(),
        _ => Err(ProgramError::InvalidInstructionData),
    }?;
    Ok(())
}
