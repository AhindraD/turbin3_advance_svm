use pinocchio::program_error::ProgramError;
pub mod deposit;
pub mod withdraw;
pub use deposit::*;
pub use withdraw::*;

#[repr(C)]
pub enum VaultInstruction {
    DEPOSIT,
    WITHDRAW,
}

impl TryFrom<&u8> for VaultInstruction {
    type Error = ProgramError;
    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(VaultInstruction::DEPOSIT),
            1 => Ok(VaultInstruction::WITHDRAW),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
