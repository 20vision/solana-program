// In this example the same PDA is used as both the address of the mint account and the mint authority
// This is to demonstrate that the same PDA can be used for both the address of an account and CPI signing
use {
    anchor_lang::prelude::*,
    anchor_lang::system_program,
};
use fixed::types::I64F64;
use fixed_sqrt::FixedSqrt;

use crate::state::{
    ConstraintFunctionSignerList,
    MultiSigAdminList,
    UtilityStakeMint
};


#[derive(Accounts)]
#[instruction(constraint_signer: Pubkey, admin_signer: Pubkey)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + UtilityStakeMint::LEN
    )]
    pub mint_account: Account<'info, UtilityStakeMint>,

    // Some functions like the buy function can have constraints like the seller having to sign the buy request.
    #[account(
        init,
        payer = payer,
        space = 8 + ConstraintFunctionSignerList::LEN,
        seeds = [b"constraint_signer_list", mint_account.key().as_ref()],
        bump
    )]
    pub constraint_signer_list_account: Account<'info, ConstraintFunctionSignerList>,

    // Some functions like the withdrawal function can have constraints like the admins having to sign the buy request.
    #[account(
        init,
        payer = payer,
        space = 8 + MultiSigAdminList::LEN,
        seeds = [b"multi_sig_admin_list", mint_account.key().as_ref()],
        bump
    )]
    pub multi_sig_admin_list_account: Account<'info, MultiSigAdminList>,

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

    let constraint_signer_list_account = &mut ctx.accounts.constraint_signer_list_account;
    constraint_signer_list_account.constraint_account_ids = vec![
        constraint_signer.key(),
    ];

    let multi_sig_admin_list_account = &mut ctx.accounts.multi_sig_admin_list_account;
    multi_sig_admin_list_account.admin_account_ids = vec![
        admin_signer.key(),
    ];

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