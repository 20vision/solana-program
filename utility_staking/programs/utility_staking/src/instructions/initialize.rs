// In this example the same PDA is used as both the address of the mint account and the mint authority
// This is to demonstrate that the same PDA can be used for both the address of an account and CPI signing
use {
    anchor_lang::prelude::*,
    anchor_lang::system_program,
};
use fixed::types::I64F64;
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

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize(
    ctx: Context<Initialize>,
    constraint_signer: Pubkey,
    admin_signer: Pubkey,
) -> Result<()> {

    // Min lamports to be rent exempt
    let min_collateral_lamports = (Rent::get()?).minimum_balance(8 + UtilityStakeMint::LEN);

    let initial_token = I64F64::from_num(min_collateral_lamports)
        .checked_div(
            I64F64::from_num(3)
                .checked_div(I64F64::from_num(40))
                .unwrap(),
        )
        .unwrap()
        .sqrt()
        .to_num::<u64>();

    msg!("Initial lamports: {} | Initial token: {}", min_collateral_lamports, initial_token);

    let mint = &mut ctx.accounts.mint_account;
    mint.stakes_total = initial_token;
    mint.stakes_burnt = 0;
    mint.collateral = min_collateral_lamports;

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
        min_collateral_lamports,
    )?;

    msg!("Token created successfully.");

    Ok(())
}