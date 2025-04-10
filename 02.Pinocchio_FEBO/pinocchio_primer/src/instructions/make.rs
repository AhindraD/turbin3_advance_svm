use pinocchio::{ProgramResult, account_info::AccountInfo, program_error::ProgramError};

pub fn process_make_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        maker_ata_account,
        escrow_account,
        vault_account,
        mint_x_account,
        mint_y_account,
        token_program_account,
    ] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    Ok(())
}
