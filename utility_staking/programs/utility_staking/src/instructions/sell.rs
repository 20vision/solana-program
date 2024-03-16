use {
    anchor_lang::prelude::*,
    anchor_lang::system_program,
};
use fixed::types::I64F64;
use crate::state::{
    UtilityStakeAccount,
    UtilityStakeMint,
    ConstraintFunctionSignerList,
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

    let mint_account = &mut ctx.accounts.mint_account;

    let stakes = I64F64::from_num(mint_account.stakes_total).checked_sub(I64F64::from_num(mint_account.stakes_burnt)).unwrap();

    let collateral = I64F64::from_num(mint_account.collateral);

    let burnt_ratio = I64F64::from_num(mint_account.stakes_burnt).checked_div(I64F64::from_num(mint_account.stakes_total)).unwrap();
    
    let my_stakes_burnt = I64F64::from_num(amount_in).checked_mul(burnt_ratio).unwrap();

    let my_stakes = I64F64::from_num(amount_in).checked_sub(my_stakes_burnt).unwrap();

    // let subtracted_total_stakes = stakes.checked_sub(my_stakes).unwrap();

    // let subtracted_total_stakes_squared = subtracted_total_stakes.checked_mul(subtracted_total_stakes).unwrap();
    // P*(x) = 0.075 * x^2
    // let collateral_rest = I64F64::from_num(3)
    //         .checked_div(I64F64::from_num(40))
    //         .unwrap()
    //     .checked_mul(subtracted_total_stakes_squared)
    //     .unwrap();

    // let my_collateral = collateral.checked_sub(collateral_rest).unwrap();
    
    // let my_collateral_u64 = my_collateral.to_num::<u64>();

    // if my_collateral_u64 < min_output_amount {
    //     return Err(anchor_lang::error!(ContractError::PriceChanged)); 
    // }

    // let associated_utility_stake_account = &mut ctx.accounts.associated_utility_stake_account;

    // if associated_utility_stake_account.amount < my_collateral_u64{
    //     return Err(anchor_lang::error!(ContractError::InsufficientTokenBalance)); 
    // }

    // associated_utility_stake_account.amount = I64F64::from_num(associated_utility_stake_account.amount).checked_sub(my_collateral).unwrap().to_num::<u64>();

    // mint_account.stakes_total = I64F64::from_num(mint_account.stakes_total).checked_sub(I64F64::from_num(amount_in)).unwrap().to_num::<u64>();
    // mint_account.collateral = I64F64::from_num(mint_account.collateral).checked_sub(my_collateral).unwrap().to_num::<u64>();
    
    
    // msg!("amount_out: {}", my_collateral_u64);
    // msg!("amount_in: {}", amount_in);

    // system_program::transfer(
    //     CpiContext::new(
    //         ctx.accounts.system_program.to_account_info(),
    //         system_program::Transfer {
    //             from: ctx.accounts.mint_account.to_account_info(),
    //             to: ctx.accounts.seller.to_account_info(),
    //         },
    //     ),
    //     my_collateral_u64,
    // )?;

    Ok(())
}
