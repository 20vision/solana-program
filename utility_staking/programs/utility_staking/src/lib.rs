#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use instructions::*;
pub mod instructions;
mod state;
// mod errors;

declare_id!("FYJN5mcoNEAisD72LWgtcLxBAeJhD4n3DQSyUHtHpptN");

#[program]
pub mod utility_staking {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: String,
        constraint_signer: Pubkey,
        admin_signer: Pubkey,
        token_name: String,
        token_symbol: String,
        token_uri: String,
    ) -> Result<()> {
        initialize::initialize(
            ctx,
            seed,
            constraint_signer,
            admin_signer,
            token_name,
            token_symbol,
            token_uri,
        )
    }

    pub fn mint_token(ctx: Context<MintToken>, seed: String, amount: u64) -> Result<()> {
        mint::mint_token(ctx, seed, amount)
    }
}
