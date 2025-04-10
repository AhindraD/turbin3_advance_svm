use pinocchio::pubkey::Pubkey;

#[repr(C)] //to stop rust from changing the order for memory optimization
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct EscrowState {
    pub maker: Pubkey,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub amount_offered: u64,
    pub amount_wanted: u64,
    pub bump: u8,
}

impl EscrowState {}
