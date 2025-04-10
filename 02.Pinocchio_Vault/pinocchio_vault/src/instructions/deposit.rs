use crate::state::DataLen;

pub const LAMPORTS_PER_SOL: u32 = 1_000_000_000;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DepositIxData {
    pub amount: u8,
    pub state_bump: u8,
    pub vault_bump: u8,
}

impl DataLen for DepositIxData {
    const LEN: usize = core::mem::size_of::<DepositIxData>();
}
