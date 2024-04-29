use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // automatically calculate the space required for the struct
pub struct WithdrawalAccount {
    pub amount: u64,
    pub deadline: u64,
    #[max_len(200)] // set a max length for the string
    pub description: String,
}

#[account]
#[derive(Default)]
pub struct UtilityStakeMint {
    // stakes are 10^9 just like collateral lamports to SOL conversion
    pub stakes_total: u64,
    pub stakes_burnt: u64,
    pub collateral: u64,
    pub admin_signer: Pubkey,
    pub constraint_signer: Pubkey
}

impl UtilityStakeMint {
    pub const LEN: usize = 8 + 8 + 8 + 32 + 32;
}

#[event]
pub struct UtilityTradeEvent {
    pub stakes_total: u64,
    pub collateral: u64
}

#[event]
pub struct UtilityWithdrawEvent {
    pub stakes_burnt: u64,
    pub collateral: u64
}

#[account]
#[derive(Default)]
pub struct UtilityStakeAccount {
    /// Account that has a bunch of sub accounts that have to be signers for "constraint functions"
    pub mint: Pubkey,
    pub hodler: Pubkey,
    pub amount: u64
}

impl UtilityStakeAccount {
    pub const LEN: usize = 32 + 32 + 8;
}