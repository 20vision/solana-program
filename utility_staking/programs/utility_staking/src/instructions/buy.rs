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

pub fn buy(ctx: Context<Buy>, amount_in: u64, min_output_amount: u64) -> Result<()> {

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

    // 1/k_div * x_2^2 - c
    // p(x) = 1/k * x^2 - c
    // p(x_1, x_2) = sqrt(c*k + k*y)

    let total_token_after_purchase =  U128F0::from_num(
            (mint_account.collateral as u128).checked_mul(k_div)
            .unwrap()
            .checked_add(
                (amount_in as u128).checked_mul(k_div).unwrap()
            ).unwrap()
        as u128)
        .sqrt()
        .to_num::<u64>();

    msg!("total_token_after_purchase: {}", total_token_after_purchase);

    // Scale down as function uses collateral = adjusted collateral
    let adjusted_total = mint_account.stakes_total.checked_sub(mint_account.stakes_burnt).unwrap();
    let token = total_token_after_purchase.checked_sub(adjusted_total).unwrap();

    msg!("token: {}, {}", token, mint_account.stakes_total);

    // Scale your token up as when you sell, you will sell your total amount, not adjusted. 
    // Otherwise one would buy adjusted amount and sell another adjustment. The previous
    // Withdrawals had nothing to do with this new buyer. We need to upscale his amount to adjust for previous withdrawals
    let adjusted_token_1 = (token as u128).checked_mul(mint_account.stakes_total as u128).unwrap();
    msg!("adjusted_token_1: {}", adjusted_token_1);
    let adjusted_token_2 = (mint_account.stakes_total as u128).checked_sub(mint_account.stakes_burnt as u128).unwrap();
    msg!("adjusted_token_2: {}", adjusted_token_2);
    let adjusted_token = adjusted_token_1.checked_div(adjusted_token_2).unwrap() as u64;

    msg!("adjusted_token_3: {}", adjusted_token);

    if adjusted_token < min_output_amount {
        return Err(anchor_lang::error!(ContractError::PriceChanged)); 
    }

    msg!("min_output_amount: {}", min_output_amount);

    mint_account.stakes_total = adjusted_token.checked_add(mint_account.stakes_total).unwrap() as u64;
    mint_account.collateral = amount_in.checked_add(mint_account.collateral).unwrap() as u64;

    let associated_utility_stake_account = &mut ctx.accounts.associated_utility_stake_account;

    if associated_utility_stake_account.mint == Pubkey::default(){
        associated_utility_stake_account.mint = mint_account.key();
    }

    if associated_utility_stake_account.hodler == Pubkey::default(){
        associated_utility_stake_account.hodler = ctx.accounts.buyer.key();
    }

    if associated_utility_stake_account.amount == u64::default(){
        associated_utility_stake_account.amount = adjusted_token;
    }else{
        associated_utility_stake_account.amount = associated_utility_stake_account.amount.checked_add(adjusted_token).unwrap() as u64;
    }
    
    msg!("adjusted_token: {}", associated_utility_stake_account.amount);
    msg!("mint_account.stakes_total: {}", mint_account.stakes_total);
    msg!("mint_account.collateral: {}", mint_account.collateral);

    emit!(UtilityTradeEvent {
        stakes_total: mint_account.stakes_total,
        collateral: mint_account.collateral,
        label: "buy".to_string(),
    });

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.collateral_account.to_account_info(),
            },
        ),
        amount_in,
    )?;

    Ok(())
}
