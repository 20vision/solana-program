use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct ConstraintFunctionSignerList {
    /// Account that has a bunch of sub accounts that have to be signers for "constraint functions"
    pub constraint_account_id: Pubkey,
    // ... Extendable, in "AddSigner" Function. Probably using the realloc = <space> function from anchor
}

impl ConstraintFunctionSignerList {
    pub const LEN: usize = 8 + 32;
}

#[account]
#[derive(Default)]
pub struct MultiSigAdminList {
    /// Account that has a bunch of sub accounts that have to be signers for "constraint functions"
    pub admin_account_id: Pubkey,
    // ... Extendable, in "AddSigner" Function. Probably using the realloc = <space> function from anchor
}

impl MultiSigAdminList {
    pub const LEN: usize = 8 + 32;
}