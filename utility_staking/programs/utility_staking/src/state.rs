use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct DexInfo {
    /// Account that has a bunch of sub accounts that have to be signers for "constrained functions"
    pub constraint_list_account: Pubkey,
    /// Account that has a bunch of sub accounts that have to be signers for "admin functions"
    pub multisig_list_account: Pubkey
}

impl DexInfo {
    pub const LEN: usize = 8 + 32 + 32;
}