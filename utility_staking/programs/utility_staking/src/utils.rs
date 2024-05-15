use anchor_lang::prelude::*;
use crate::state::{
    UtilityStakeMint
};

use crate::errors::ContractError;

// Described in README.md
fn withdrawal_adjustment_downscale(stake: u128, mint_account: UtilityStakeMint) -> u64 {
    // s * ((total - withdrawn) / total)
    (stake.checked_mul(
        mint_account.stakes_total as u128
    ).unwrap()
    .checked_sub(
        stake.checked_mul(mint_account.stakes_burnt as u128).unwrap()
    ).unwrap()
    .checked_div(
        mint_account.stakes_total as u128
    ).unwrap()) as u64
}

// Described in README.md
fn withdrawal_adjustment_upscale(stake: u128, mint_account: UtilityStakeMint) -> u128 {
    // s * (total / (total - withdrawn))
    (stake.checked_mul(
        mint_account.stakes_total as u128
    ).unwrap()
    .checked_div(
        (mint_account.stakes_total as u128)
        .checked_sub(mint_account.stakes_burnt as u128)
        .unwrap()
    ).unwrap()) as u128
}