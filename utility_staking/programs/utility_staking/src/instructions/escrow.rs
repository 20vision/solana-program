use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{mint_to, Mint, MintTo, Token, TokenAccount},
    },
};

#[derive(Accounts)]
#[instruction(seed: String)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // Mint account address is a PDA
    #[account(
        mut,
        seeds = [seed.as_bytes()],
        bump
    )]
    pub mint_account: Account<'info, Mint>,

    // Create Associated Token Account, if needed
    // This is the account that will hold the minted tokens
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn buy_request(ctx: Context<BuyRequest>, amount: u64) -> Result<()> {
    let buy_request = &mut ctx.accounts.buy_request;
    buy_request.buyer = *ctx.accounts.buyer.key;
    buy_request.amount = amount;
    buy_request.timestamp = clock::Clock::get()?.unix_timestamp;
    Ok(())
}
