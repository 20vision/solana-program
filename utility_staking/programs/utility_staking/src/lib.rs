#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use instructions::*;
pub mod instructions;
mod state;
mod utils;
mod errors;
// mod errors;

declare_id!("3ufPJGjtBysKYtgu1p4A2mgjWzkcUyn7V32Crr2vuxXw");

#[program]
pub mod utility_staking {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        constraint_signer: Pubkey,
        admin_signer: Pubkey
    ) -> Result<()> {
        initialize::initialize(
            ctx,
            constraint_signer,
            admin_signer
        )
    }

    pub fn buy(ctx: Context<Buy>, amount_in: u64, min_output_amount: u64) -> Result<()> {
        buy::buy(ctx, amount_in, min_output_amount)
    }

    pub fn sell(ctx: Context<Sell>, amount_in: u64, min_output_amount: u64) -> Result<()> {
        sell::sell(ctx, amount_in, min_output_amount)
    }
}
