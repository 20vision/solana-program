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

    #[account(
        mut,
        seeds = [mint_account.key().as_ref(), b"Collateral"],
        bump
    )]
    pub collateral_account: AccountInfo<'info>,

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
        return Err(anchor_lang::error!(ContractError::InsufficientTokenBalance)); 
    }

    let mint_account = &mut ctx.accounts.mint_account;

    // p_sell = collateral - k*(total-amount_in)^2

    // k_div = 1/k
    let k_div = 30000000000000000 as u128;

    // sell = collateral - k * x_diff^2

    let adjusted_my_token = amount_in.checked_sub((amount_in.checked_mul(mint_account.stakes_burnt).unwrap()).checked_div(mint_account.stakes_total).unwrap()).unwrap();

    let adjusted_stakes_total = mint_account.stakes_total.checked_sub(mint_account.stakes_burnt).unwrap();

    if adjusted_my_token > adjusted_stakes_total {
        return Err(anchor_lang::error!(ContractError::InsufficientCollateralInContract)); 
    }

    let adjusted_token = adjusted_stakes_total.checked_sub(adjusted_my_token).unwrap();

    let total_collater_after = ((adjusted_token as u128).checked_mul(adjusted_token as u128).unwrap()).checked_div(k_div).unwrap();

    let sell_price = mint_account.collateral.checked_sub(total_collater_after as u64).unwrap();


    if sell_price < min_output_amount {
        return Err(anchor_lang::error!(ContractError::PriceChanged)); 
    }

    if mint_account.collateral < sell_price {
        return Err(anchor_lang::error!(ContractError::InsufficientCollateralInContract)); 
    }

    associated_utility_stake_account.amount = associated_utility_stake_account.amount.checked_sub(amount_in).unwrap();
    mint_account.stakes_total = mint_account.stakes_total.checked_sub(amount_in).unwrap();
    mint_account.collateral = mint_account.collateral.checked_sub(sell_price).unwrap();
    
    
    msg!("amount_out: {}", sell_price);
    msg!("amount_in: {}", amount_in);

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
        sell_price,
    )?;

    Ok(())
}
