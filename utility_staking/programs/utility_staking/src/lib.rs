#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use instructions::*;
pub mod instructions;

declare_id!("8N7gcxdxmUNHhR5WS6kYqWbazpqc4cULWeJ5NWwreXE8");

#[program]
pub mod utility_staking {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        token_name: String,
        token_symbol: String,
        token_uri: String,
    ) -> Result<()> {
        initialize::initialize(ctx, token_name, token_symbol, token_uri)
    }
}
