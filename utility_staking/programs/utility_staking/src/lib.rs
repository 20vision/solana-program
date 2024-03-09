#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use instructions::*;
pub mod instructions;

declare_id!("8QZvKkDrzU28i5sZ92Mv54vWus9FJhUbRpcKPy1AXkik");

#[program]
pub mod utility_staking {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u8,
        token_name: String,
        token_symbol: String,
        token_uri: String,
    ) -> Result<()> {
        initialize::initialize(ctx, seed, token_name, token_symbol, token_uri)
    }
}
