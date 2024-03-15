use {
    anchor_lang::prelude::*,
};

use crate::state::{
    UtilityStakeAccount,
    UtilityStakeMint,
    ConstraintFunctionSignerList
};

#[derive(Accounts)]
#[instruction(seed: String)]
pub struct Buy<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        owner = crate::ID
    )]
    pub mint_account: Account<'info, UtilityStakeMint>,

    // Mint account address is a PDA
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + UtilityStakeAccount::LEN,
        seeds = [mint_account.key().as_ref(), payer.key().as_ref()],
        bump
    )]
    pub associated_utility_stake_account: Account<'info, UtilityStakeAccount>,

    // Some functions like the buy function can have constraints like the seller having to sign the buy request.
    #[account(
        seeds = [b"constraint_signer_list", mint_account.key().as_ref()],
        bump
    )]
    pub constraint_signer_list_account: Account<'info, ConstraintFunctionSignerList>,

    pub system_program: Program<'info, System>,
}

pub fn buy(ctx: Context<Buy>, amount_in: u64, min_output_amount: u64) -> Result<()> {
    let constraint_signer_list = ctx.accounts.constraint_signer_list_account.constraint_account_ids.iter();

    let constraint_signed_count = 0;
    for account_info in constraint_signer_list {
        msg!("Account {} is a signer.", account_info.key());
        // if account_info.is_signer {
        //     constraint_signed_count = constraint_signed_count + 1;
        //     msg!("Account {} is a signer.", account_info.key());
        // }else{
        //     msg!("Account {} is not a signer.", account_info.key());
        // }
    }
    msg!("Accounts signed count: {}", constraint_signed_count);

    Ok(())
}
