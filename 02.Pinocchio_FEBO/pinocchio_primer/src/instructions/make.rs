use pinocchio::{
    ProgramResult,
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    sysvars::{Sysvar, rent::Rent},
};
use pinocchio_system::instructions::CreateAccount;

use crate::state::EscrowState;

pub fn process_make_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        maker_account,
        maker_ata_account,
        escrow_account,
        vault_account,
        mint_x_account,
        mint_y_account,
        token_program_account,
        system_program,
    ] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let signer_seeds = [
        Seed::from(b"escrow"),
        Seed::from(maker_account.key()),
        // Seed::from(&bump),
    ];
    let signers = [Signer::from(&signer_seeds[..])];

    CreateAccount {
        from: maker_account,
        to: escrow_account,
        lamports: Rent::get()?.minimum_balance(EscrowState::LEN),
        space: EscrowState::LEN as u64,
        owner: &crate::id(),
    }
    .invoke_signed(&signers)?;
    Ok(())
}
