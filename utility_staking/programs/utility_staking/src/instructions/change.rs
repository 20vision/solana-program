use {
    anchor_lang::prelude::*,
    anchor_lang::system_program,
};
use fixed::types::U128F0;
use crate::state::{
    UtilityStakeAccount,
    UtilityStakeMint,
};

use crate::errors::ContractError;

use crate::id;

#[derive(Accounts)]
pub struct Change<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut
    )]
    pub mint_account: Box<Account<'info, UtilityStakeMint>>,
}

pub fn admin_signer(ctx: Context<Change>, new_admin: Pubkey) -> Result<()> {

    let mint_account = &mut ctx.accounts.mint_account;

    if ctx.accounts.admin.key() != *mint_account.admin_signer {
        return Err(anchor_lang::error!(ContractError::InvalidAdminSigner));
    }
    
    if !admin.is_signer {
        return Err(anchor_lang::error!(ContractError::AdminSignerNotSigned));
    }

    mint_account.admin_signer = new_admin;

    Ok(())
}

pub fn constraint_signer(ctx: Context<Change>, new_constraint_signer: Pubkey) -> Result<()> {

    let mint_account = &mut ctx.accounts.mint_account;

    if ctx.accounts.admin.key() != *mint_account.admin_signer {
        return Err(anchor_lang::error!(ContractError::InvalidAdminSigner));
    }
    
    if !admin.is_signer {
        return Err(anchor_lang::error!(ContractError::AdminSignerNotSigned));
    }

    mint_account.constraint_signer = new_constraint_signer;

    Ok(())
}