use {
    anchor_lang::prelude::*,
    anchor_lang::system_program,
};
use fixed::types::U64F64;
use crate::state::{
    UtilityStakeAccount,
    UtilityStakeMint,
    UtilityTradeEvent
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

    #[account(
        mut,
        seeds = [mint_account.key().as_ref(), b"Collateral"],
        bump
    )]
    pub collateral_account: SystemAccount<'info>,

    // Mint account address is a PDA
    #[account(
        mut,
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
    if associated_utility_stake_account.amount < amount_in {
        return Err(anchor_lang::error!(ContractError::InsufficientStakeBalance)); 
    }

    let mint_account = &mut ctx.accounts.mint_account;

    // p_sell = collateral - k*(total-amount_in)^2

    // k_div = 1/k
    let k_div = 30000000000000000 as u128;

    // k * x^2 = collateral
    // x = total - burnt - burn_adjusted_amount_in

    // @20vision review this part ! Burn adjustment or not and if, is that right ?!?!
    // let burn_adjusted_amount_in = (amount_in as u128).checked_sub(
    //     (amount_in as u128)
    //     .checked_mul(mint_account.stakes_burnt as u128).unwrap()
    //     .checked_div(mint_account.stakes_total as u128).unwrap()
    // ) as u128;

    let token_in_pool_after_sell =  mint_account.stakes_total
        .checked_sub(mint_account.stakes_burnt).unwrap()
        .checked_sub(burn_adjusted_amount_in as u64).unwrap() as u64;

    let token_in_pool_after_sell_squared = (token_in_pool_after_sell as u128)
        .checked_mul(token_in_pool_after_sell as u128)
        .unwrap() as u128;

    let collateral_after_sell = token_in_pool_after_sell_squared
        .checked_div(k_div)
        .unwrap() as u64;

    // collateral - collateral_after_sell = my_collateral
    let my_collateral = mint_account.collateral.checked_sub(collateral_after_sell).unwrap();

    if my_collateral < min_output_amount {
        return Err(anchor_lang::error!(ContractError::PriceChanged)); 
    }

    if mint_account.collateral < my_collateral {
        return Err(anchor_lang::error!(ContractError::InsufficientCollateralInContract)); 
    }

    associated_utility_stake_account.amount = associated_utility_stake_account.amount.checked_sub(burn_adjusted_amount_in).unwrap();

    let authority_bump = *ctx.bumps.get("collateral_account").unwrap();
    let authority_seeds = &[
        &ctx.accounts.mint_account.key().to_bytes(),
        "Collateral".as_bytes(),
        &[authority_bump],
    ];
    let signer_seeds = &[&authority_seeds[..]];

    system_program::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.collateral_account.to_account_info(),
                to: ctx.accounts.seller.to_account_info(),
            },
            signer_seeds,
        ),
        my_collateral,
    )?;

    emit!(UtilityTradeEvent {
        stakes_total: ctx.accounts.mint_account.stakes_total,
        collateral: ctx.accounts.mint_account.collateral
    });

    Ok(())
}
