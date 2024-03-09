#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use instructions::*;
pub mod instructions;

declare_id!("49KPpNGzdqZ1ntT7kEq1jR2x1oYKJ5y3sQG9LQJTZ6Yu");

#[program]
pub mod utility_staking {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: String,
        token_name: String,
        token_symbol: String,
        token_uri: String,
    ) -> Result<()> {
        initialize::initialize(ctx, seed, token_name, token_symbol, token_uri)
    }

    pub fn mint_token(ctx: Context<MintToken>, seed: String, amount: u64) -> Result<()> {
        mint::mint_token(ctx, seed, amount)
    }
}
