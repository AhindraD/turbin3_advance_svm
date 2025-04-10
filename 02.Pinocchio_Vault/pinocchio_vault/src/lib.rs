use pinocchio::{
    ProgramResult, account_info::AccountInfo, entrypoint, program_error::ProgramError,
    pubkey::Pubkey,
};

pub mod state;
pub mod instructions;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts_data: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (ix_disc, instructions) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match ix_disc{
        0=>todo!()
        _=>Err(ProgramError::InvalidInstructionData)?
    };
    Ok(())
}
