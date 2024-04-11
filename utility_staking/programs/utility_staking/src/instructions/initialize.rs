// In this example the same PDA is used as both the address of the mint account and the mint authority
// This is to demonstrate that the same PDA can be used for both the address of an account and CPI signing
use {
    anchor_lang::prelude::*,
    anchor_lang::system_program,
};
use fixed::types::U128F0;
use fixed_sqrt::FixedSqrt;

use crate::state::{
    UtilityStakeMint,
};


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        signer,
        payer = payer,
        space = 8 + UtilityStakeMint::LEN
    )]
    pub mint_account: Box<Account<'info, UtilityStakeMint>>,

    #[account(
        seeds = [mint_account.key().as_ref(), b"Collateral"],
        bump
    )]
    pub collateral_account: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize(
    ctx: Context<Initialize>,
    constraint_signer: Pubkey,
    admin_signer: Pubkey,
) -> Result<()> {

    // Min lamports to be rent exempt
    let min_mint_lamports = (Rent::get()?).minimum_balance(8 + UtilityStakeMint::LEN) as u64;
    let min_collateral_lamports = (Rent::get()?).minimum_balance(0) as u64;

    let min_lamports = min_mint_lamports.checked_add(min_collateral_lamports).unwrap();

    // k_div = 1/k
    let k_div = 30000000000000000 as u128;

    // overflow - can handle up to sqrt 2^128 -1  / 10^9 = 18446744073 SOL = greater than total supply
    let token_product = (min_lamports as u128).checked_mul(k_div).unwrap();

    let initial_token = U128F0::from_num(token_product as u128)
        .sqrt()
        .to_num::<u64>();

    msg!("Initial lamports: {} | Initial token: {}", min_lamports, initial_token);

    let mint = &mut ctx.accounts.mint_account;
    mint.stakes_total = initial_token;
    mint.stakes_burnt = 0;
    mint.collateral = min_lamports;

    mint.admin_signer = admin_signer;
    mint.constraint_signer = constraint_signer;


    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.mint_account.to_account_info(),
            },
        ),
        min_mint_lamports,
    )?;

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.collateral_account.to_account_info(),
            },
        ),
        min_collateral_lamports,
    )?;

    msg!("Token created successfully.");

    Ok(())
}