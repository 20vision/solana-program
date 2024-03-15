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
pub struct Buy<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut
    )]
    pub mint_account: Box<Account<'info, UtilityStakeMint>>,

    // Mint account address is a PDA
    #[account(
        init_if_needed,
        payer = buyer,
        space = 8 + UtilityStakeAccount::LEN,
        seeds = [mint_account.key().as_ref(), buyer.key().as_ref()],
        bump
    )]
    pub associated_utility_stake_account: Box<Account<'info, UtilityStakeAccount>>,

    // Some functions like the buy function can have constraints like the seller having to sign the buy request.
    #[account(
        seeds = [b"constraint_signer_list", mint_account.key().as_ref()],
        bump
    )]
    pub constraint_signer_list_account: Box<Account<'info, ConstraintFunctionSignerList>>,

    pub system_program: Program<'info, System>,
}

pub fn buy(ctx: Context<Buy>, amount_in: u64, min_output_amount: u64) -> Result<()> {
    let constraint_signer_list = &ctx.accounts.constraint_signer_list_account;

    // Check if all pubkeys in the constraint_signer_list have signed the transaction
    for (i, required_signer) in constraint_signer_list.constraint_account_ids.iter().enumerate() {
        let signer = &ctx.remaining_accounts.get(i).ok_or_else(|| 
            anchor_lang::error!(ContractError::MissingConstraintSigner))?;
        
        if signer.key() != *required_signer {
            return Err(anchor_lang::error!(ContractError::IncorrectOrderOfSigners));
        }
        
        if !signer.is_signer {
            return Err(anchor_lang::error!(ContractError::MissingSignatureConstraintSigner));
        }
    }

    let mint_account = &mut ctx.accounts.mint_account;

    let collateral = I64F64::from_num(mint_account.collateral);

    

    let sum_collateral = I64F64::from_num(amount_in).checked_add(collateral).unwrap();


    let sum_collateral_squared = I64F64::from_num(sum_collateral).checked_mul(I64F64::from_num(sum_collateral)).unwrap();

    let sum_token = sum_collateral_squared.checked_mul(
        I64F64::from_num(3)
            .checked_div(I64F64::from_num(40))
            .unwrap()
    ).unwrap();

    let existing_token = I64F64::from_num(mint_account.stakes_total).checked_sub(I64F64::from_num(mint_account.stakes_burnt)).unwrap();
    let buyer_token = sum_token.checked_sub(existing_token).unwrap().to_num::<u64>();

   

    if buyer_token < min_output_amount {
        return Err(anchor_lang::error!(ContractError::PriceChanged)); 
    }

    mint_account.stakes_total = I64F64::from_num(buyer_token).checked_add(I64F64::from_num(mint_account.stakes_total)).unwrap().to_num::<u64>();
    mint_account.collateral = sum_collateral.to_num::<u64>();
    

    let associated_utility_stake_account = &mut ctx.accounts.associated_utility_stake_account;

    if associated_utility_stake_account.mint == Pubkey::default(){
        associated_utility_stake_account.mint = mint_account.key();
    }

    if associated_utility_stake_account.hodler == Pubkey::default(){
        associated_utility_stake_account.hodler = ctx.accounts.buyer.key();
    }

    if associated_utility_stake_account.amount == u64::default(){
        associated_utility_stake_account.amount = buyer_token;
    }else{
        associated_utility_stake_account.amount = I64F64::from_num(associated_utility_stake_account.amount).checked_add(I64F64::from_num(buyer_token)).unwrap().to_num::<u64>();;
    }
    
    
    msg!("sum_token: {}", sum_token.to_num::<u64>());
    msg!("collateral: {}", collateral.to_num::<u64>());
    msg!("sum_collateral: {}", sum_collateral.to_num::<u64>());
    msg!("buyer_token: {}", buyer_token);
    msg!("mint_account.stakes_total: {}", mint_account.stakes_total);
    msg!("mint_account.collateral: {}", mint_account.collateral);

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.mint_account.to_account_info(),
            },
        ),
        amount_in,
    )?;

    Ok(())
}
