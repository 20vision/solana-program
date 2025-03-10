use {
    anchor_lang::prelude::*,
    anchor_lang::system_program,
};
use fixed::types::{
    U128F0,
    U64F64,
};
use crate::state::{
    UtilityStakeMint,
    WithdrawalAccount,
    UtilityWithdrawEvent
};

use crate::errors::ContractError;

use crate::id;

#[derive(Accounts)]
pub struct WithdrawalInit<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut
    )]
    pub mint_account: Box<Account<'info, UtilityStakeMint>>,

    #[account(
        init,
        payer = admin,
        space = WithdrawalAccount::INIT_SPACE,
        seeds = [mint_account.key().as_ref(), b"Withdrawal"],
        bump
    )]
    pub withdrawal_account: Box<Account<'info, WithdrawalAccount>>,

    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<WithdrawalInit>, amount: u64, description: String) -> Result<()> {

    let mint_account = &mut ctx.accounts.mint_account;

    if ctx.accounts.admin.key() != mint_account.admin_signer {
        return Err(anchor_lang::error!(ContractError::InvalidAdminSigner));
    }
    
    if !ctx.accounts.admin.is_signer {
        return Err(anchor_lang::error!(ContractError::AdminSignerNotSigned));
    }

    if mint_account.collateral < amount {
        return Err(anchor_lang::error!(ContractError::InsufficientCollateralInContract));
    }

    // ((9)/(42968750)) x + 864000

    // 10 Days minimum
    let mut wait_time = 864000 as u64;
    
    // smaller than 100 000$/300 estimated solana price
    if amount > 333333333333 {
        if amount < 33333333333333 {
            // smaller than 10 million $/300 estimated solana price

            // Linear growth between roughly 100k - 10 million $ => 10 days - 60 days
            // (9/42968750) x + 864000

            let mut product = (9 as u64).checked_mul(amount).unwrap();

            product = U64F64::from_num(product)
            .checked_div(U64F64::from_num(68750000))
            .unwrap()
            .to_num::<u64>();

            wait_time = product.checked_add(864000 as u64).unwrap();
        }else{
            // 60 days in seconds = 5184000
            wait_time = 5184000 as u64;
        }
    }

    let withdrawal = &mut ctx.accounts.withdrawal_account;

    withdrawal.amount = amount;

    let clock = Clock::get()?;

    withdrawal.deadline = (clock.unix_timestamp as u64).checked_add(wait_time).unwrap();
    
    // @20vision remove:
    withdrawal.deadline = clock.unix_timestamp as u64;

    withdrawal.description = description;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawalClosure<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut
    )]
    pub mint_account: Box<Account<'info, UtilityStakeMint>>,

    // Mint account address is a PDA
    #[account(
        mut,
        seeds = [mint_account.key().as_ref(), b"Withdrawal"],
        bump,
        close = admin,
    )]
    pub withdrawal_account: Box<Account<'info, WithdrawalAccount>>,

    pub system_program: Program<'info, System>,
}

pub fn abort(ctx: Context<WithdrawalClosure>) -> Result<()> {

    let mint_account = &mut ctx.accounts.mint_account;

    if ctx.accounts.admin.key() != mint_account.admin_signer {
        return Err(anchor_lang::error!(ContractError::InvalidAdminSigner));
    }
    
    if !ctx.accounts.admin.is_signer {
        return Err(anchor_lang::error!(ContractError::AdminSignerNotSigned));
    }

    Ok(())
}

#[derive(Accounts)]
pub struct Withdrawal<'info> {
    #[account(mut)]
    pub admin: SystemAccount<'info>,

    #[account(
        mut
    )]
    pub mint_account: Box<Account<'info, UtilityStakeMint>>,

    // Mint account address is a PDA
    #[account(
        mut,
        seeds = [mint_account.key().as_ref(), b"Withdrawal"],
        bump,
        close = admin,
    )]
    pub withdrawal_account: Box<Account<'info, WithdrawalAccount>>,

    #[account(
        mut,
        seeds = [mint_account.key().as_ref(), b"Collateral"],
        bump
    )]
    pub collateral_account: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn withdraw(ctx: Context<Withdrawal>) -> Result<()> {

    let mint_account = &mut ctx.accounts.mint_account;
    let withdrawal = &mut ctx.accounts.withdrawal_account;

    if ctx.accounts.admin.key() != mint_account.admin_signer {
        return Err(anchor_lang::error!(ContractError::InvalidAdminSigner));
    }

    if mint_account.collateral < withdrawal.amount {
        return Err(anchor_lang::error!(ContractError::InsufficientCollateralInContract));
    }

    let clock = Clock::get()?;

    if (clock.unix_timestamp as u64) < withdrawal.deadline {
        return Err(anchor_lang::error!(ContractError::StillTimeLeft));
    }

    // Have to withdraw within 10 days after withdrawal deadline.
    let margin_withdrawal_deadline = withdrawal.deadline.checked_add(864000 as u64).unwrap();

    if (clock.unix_timestamp as u64) > margin_withdrawal_deadline {
        return Err(anchor_lang::error!(ContractError::TooLate));
    }

    // Find out s_w:
    // s = stakes; w = withdrawn; c = collateral
    // s_total - s_previous_w - s_w = c - c_w
    // express as stakes not collateral: c - c_w = k * x^2 ; x = sqrt((c - c_w) / k)
    // s_total - s_previous_w - s_w = sqrt((c - c_w) / k)
    // s_w = (s_total - s_previous_w) - sqrt((c - c_w) / k)

    // @20vision change to the above formula. Blow has not been checked

    // k_div = 1/k
    let k_div = 30000000000000000 as u128;

    let adjusted_stakes_total = mint_account.stakes_total.checked_sub(mint_account.stakes_burnt).unwrap();

    let collateral_after_withdraw = mint_account.collateral.checked_sub(withdrawal.amount).unwrap();

    let stakes_rest = U128F0::from_num(
        (collateral_after_withdraw as u128).checked_mul(k_div).unwrap()
    ).sqrt().to_num::<u64>();

    let stakes_withdrawn = adjusted_stakes_total.checked_sub(stakes_rest).unwrap();

    mint_account.stakes_burnt = mint_account.stakes_burnt.checked_add(stakes_withdrawn).unwrap();

    mint_account.collateral = mint_account.collateral.checked_sub(withdrawal.amount).unwrap();

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
                to: ctx.accounts.admin.to_account_info(),
            },
            signer_seeds,
        ),
        withdrawal.amount,
    )?;

    msg!("stakes burnt {} | collateral {}", ctx.accounts.mint_account.stakes_burnt, ctx.accounts.mint_account.collateral);

    emit!(UtilityWithdrawEvent {
        stakes_burnt: ctx.accounts.mint_account.stakes_burnt,
        collateral: ctx.accounts.mint_account.collateral
    });

    Ok(())
}