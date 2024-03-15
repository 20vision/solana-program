use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct ConstraintFunctionSignerList {
    /// Account that has a bunch of sub accounts that have to be signers for "constraint functions"
    pub constraint_account_ids: Vec<Pubkey>,
    // ... Extendable, in "AddSigner" Function. Probably using the realloc = <space> function from anchor
}

// According to Anchor docs: https://book.anchor-lang.com/anchor_references/space.html Vec size is 4 + (#vectors * eg. size of pubkey)
impl ConstraintFunctionSignerList {
    pub const LEN: usize = 4 + 1 * 32;
}

#[account]
#[derive(Default)]
pub struct MultiSigAdminList {
    /// Account that has a bunch of sub accounts that have to be signers for "constraint functions"
    pub admin_account_ids: Vec<Pubkey>,
    // ... Extendable, in "AddSigner" Function. Probably using the realloc = <space> function from anchor
}

impl MultiSigAdminList {
    pub const LEN: usize = 4 + 1 * 32;
}

#[account]
#[derive(Default)]
pub struct UtilityStakeMint {
    // stakes are 10^9 just like collateral lamports to SOL conversion
    pub stakes_total: u64,
    pub stakes_burnt: u64,
    pub collateral: u64,
}

impl UtilityStakeMint {
    pub const LEN: usize = 8 + 8 + 8;
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