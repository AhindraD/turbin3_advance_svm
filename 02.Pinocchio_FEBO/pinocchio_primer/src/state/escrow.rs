use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

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

impl EscrowState {
    pub const LEN: usize = 32 + 32 + 32 + 8 + 8 + 1;

    pub fn from_account_info(account_info: &AccountInfo) -> &mut Self {
        assert_eq!(account_info.data_len(), Self::LEN);
        assert_eq!(unsafe { *account_info.owner() }, crate::id());

        unsafe { &mut *(account_info.borrow_mut_data_unchecked().as_mut_ptr() as *mut Self) }
    }

    pub fn from_account_info_readable(account_info: &AccountInfo) -> &Self {
        assert_eq!(account_info.data_len(), Self::LEN);
        assert_eq!(unsafe { *account_info.owner() }, crate::id());

        unsafe { &*(account_info.borrow_data_unchecked().as_ptr() as *const Self) }
    }
}
