use {
    anchor_lang::prelude::*,
    anchor_lang::system_program,
};
use fixed::types::U64F64;
use crate::state::{
    UtilityStakeAccount,
    UtilityStakeMint,
};

use crate::errors::ContractError;

use crate::id;

#[derive(Accounts)]
pub struct Sell<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut
    )]
    pub mint_account: Box<Account<'info, UtilityStakeMint>>,

    // Mint account address is a PDA
    #[account(
        seeds = [mint_account.key().as_ref(), seller.key().as_ref()],
        bump
    )]
    pub associated_utility_stake_account: Box<Account<'info, UtilityStakeAccount>>,

    pub system_program: Program<'info, System>,
}

pub fn sell(ctx: Context<Sell>, amount_in: u64, min_output_amount: u64) -> Result<()> {

    let associated_utility_stake_account = &mut ctx.accounts.associated_utility_stake_account;

    // Token are always saved as full token amount, true token amount = after burnt = only for selling/buying price.
    // Otherwise I would have to change balances realtime after withdrawal request = high compute
    if associated_utility_stake_account.amount < amount_in{
        return Err(anchor_lang::error!(ContractError::InsufficientTokenBalance)); 
    }

    let mint_account = &mut ctx.accounts.mint_account;

    // p_sell = collateral - k*(total-amount_in)^2

    // k_div = 1/k
    let k_div = 30000000000000000 as u128;


    // p(x_1, x_2) = k * x_2^2 - k * x_1^2
    // p(x_1, x_2) = collateral - 1/k_div * x_2^2

    let x_2 = (
        mint_account.stakes_total
        // Adjustment
        .checked_sub(mint_account.stakes_burnt).unwrap()
        // x_2 = x + amount_in
        .checked_sub(amount_in)
    )

    let price = (mint_account.collateral as u128).sub(
        (x_2.mul(x_2))
        .div(k_div))


    if lamports_returned < min_output_amount {
        return Err(anchor_lang::error!(ContractError::PriceChanged)); 
    }

    if mint_account.collateral < lamports_returned {
        return Err(anchor_lang::error!(ContractError::InsufficientCollateralInContract)); 
    }

    associated_utility_stake_account.amount = associated_utility_stake_account.amount.checked_sub(amount_in).unwrap();
    mint_account.stakes_total = mint_account.stakes_total.checked_sub(amount_in).unwrap();
    mint_account.collateral = mint_account.collateral.checked_sub(lamports_returned).unwrap();
    
    
    msg!("amount_out: {}", lamports_returned);
    msg!("amount_in: {}", amount_in);

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.seller.to_account_info(),
            },
        ),
        lamports_returned,
    )?;

    Ok(())
}
