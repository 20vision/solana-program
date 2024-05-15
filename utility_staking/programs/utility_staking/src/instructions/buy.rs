use {
    anchor_lang::prelude::*,
    anchor_lang::system_program,
};
use fixed::types::U128F0;
use crate::state::{
    UtilityStakeAccount,
    UtilityStakeMint,
    UtilityTradeEvent
};

use crate::utils::{
    withdrawal_adjustment_downscale,
    withdrawal_adjustment_upscale
}

use crate::errors::ContractError;

use crate::id;

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub constraint_signer: Signer<'info>,

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
        init_if_needed,
        payer = buyer,
        space = 8 + UtilityStakeAccount::LEN,
        seeds = [mint_account.key().as_ref(), buyer.key().as_ref()],
        bump
    )]
    pub associated_utility_stake_account: Box<Account<'info, UtilityStakeAccount>>,

    pub system_program: Program<'info, System>,
}

pub fn buy(ctx: Context<Buy>, amount_in: u64, min_output_amount: u128) -> Result<()> {

    let mint_account = &mut ctx.accounts.mint_account;

    let required_signer = mint_account.constraint_signer;

    if ctx.accounts.constraint_signer.key() != required_signer {
        return Err(anchor_lang::error!(ContractError::InvalidConstraintSigner));
    }
    
    if !ctx.accounts.constraint_signer.is_signer {
        return Err(anchor_lang::error!(ContractError::ConstraintSignerNotSigned));
    }

    if amount_in <= 0 {
        return Err(anchor_lang::error!(ContractError::InvalidInputAmount));
    }

    // k_div = 1/k
    let k_div = 30000000000000000 as u128;

    // Price formula's Integral: 
    // k * x^2 = P(x)
    // x = sqrt(P(x) / k)
    // x = sqrt(EndCollateral / k)

    // x = sqrt(EndCollateral / k):
    let existing_token_after_purchase =  U128F0::from_num(
            (mint_account.collateral as u128).checked_add(amount_in as u128).unwrap()
            .checked_mul(k_div).unwrap()
        as u128)
        .sqrt()
        .to_num::<u64>();

    let upscaled_existing_token_after_purchase = withdrawal_adjustment_upscale(existing_token_after_purchase, mint_account);

    // as collateral calculates for existing token, have to adjust to total
    // total = existing + burnt
    let total_token_after_purchase = upscaled_existing_token_after_purchase.checked_add(mint_account.stakes_burnt).unwrap();

    
    let my_token = total_token_after_purchase.checked_sub(mint_account.stakes_total).unwrap();

    msg!("new_total: {}, You are getting: {} stakes", total_token_after_purchase, my_token);

    if my_token < min_output_amount {
        return Err(anchor_lang::error!(ContractError::PriceChanged)); 
    }

    mint_account.stakes_total = total_token_after_purchase as u64;
    mint_account.collateral = amount_in.checked_add(mint_account.collateral).unwrap() as u64;

    let associated_utility_stake_account = &mut ctx.accounts.associated_utility_stake_account;

    if associated_utility_stake_account.mint == Pubkey::default(){
        associated_utility_stake_account.mint = mint_account.key();
    }

    if associated_utility_stake_account.hodler == Pubkey::default(){
        associated_utility_stake_account.hodler = ctx.accounts.buyer.key();
    }

    if associated_utility_stake_account.amount == u64::default(){
        associated_utility_stake_account.amount = my_token;
    }else{
        associated_utility_stake_account.amount = associated_utility_stake_account.amount.checked_add(my_burn_adjusted_token).unwrap() as u64;
    }

    // To even out if there is a difference between saved value and
    // actual lamports owned. Saving lamports instead of querying has
    // the reason to ignore people from sending lamports to the collateral account
    // to prevent manipulation. So ignore more but fill up if less in account.
    let fill_up_collateral_account = amount_in;

    // (after amount in)
    let account_actual_balance = (ctx.accounts.collateral_account.lamports() as u64);
    let account_actual_balance_amount_in_adjusted = account_actual_balance.checked_add(fill_up_collateral_account).unwrap();

    if account_actual_balance_amount_in_adjusted < mint_account.collateral{
        fill_up_collateral_account = mint_account.collateral.checked_sub(account_actual_balance).unwrap();
    }

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.collateral_account.to_account_info(),
            },
        ),
        fill_up_collateral_account,
    )?;

    emit!(UtilityTradeEvent {
        stakes_total: ctx.accounts.mint_account.stakes_total,
        collateral: ctx.accounts.mint_account.collateral
    });

    Ok(())
}
