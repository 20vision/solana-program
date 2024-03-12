// In this example the same PDA is used as both the address of the mint account and the mint authority
// This is to demonstrate that the same PDA can be used for both the address of an account and CPI signing
use {
    anchor_lang::prelude::*,
    anchor_spl::{
        metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3, Metadata},
        token::{Mint, Token},
    },
    anchor_lang::system_program,
    mpl_token_metadata::{pda::find_metadata_account, state::DataV2},
};
use fixed::types::I64F64;
use fixed_sqrt::FixedSqrt;

use crate::{state::DexInfo};

#[derive(Accounts)]
#[instruction(seed: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // Create mint account
    // Same PDA as address of the account and mint/freeze authority
    #[account(
        init,
        seeds = [seed.as_bytes()],
        bump,
        payer = payer,
        mint::decimals = 9,
        mint::authority = mint_account.key(),
        mint::freeze_authority = mint_account.key(),
    )]
    pub mint_account: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"collateral", mint_account.key().as_ref()],
        bump
    )]
    pub collateral_account: SystemAccount<'info>,

    #[account(
        init,
        payer = payer,
        space = DexInfo::LEN,
        seeds = [b"constraint_accounts", mint_account.key().as_ref()],
        bump
    )]
    pub dex_info_account: Account<'info, DexInfo>,

    /// CHECK: Address validated using constraint
    #[account(
        mut,
        address=find_metadata_account(&mint_account.key()).0
    )]
    pub metadata_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize(
    ctx: Context<Initialize>,
    seed: String,
    token_name: String,
    token_symbol: String,
    token_uri: String,
) -> Result<()> {
    msg!("Creating metadata account");

    // PDA signer seeds
    let signer_seeds: &[&[&[u8]]] = &[&[seed.as_bytes(), &[*ctx.bumps.get("mint_account").unwrap()]]];

    // Invoking the create_metadata_account_v3 instruction on the token metadata program
    create_metadata_accounts_v3(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                mint_authority: ctx.accounts.mint_account.to_account_info(), // PDA is mint authority
                update_authority: ctx.accounts.mint_account.to_account_info(), // PDA is update authority
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        )
        .with_signer(signer_seeds),
        DataV2 {
            name: token_name,
            symbol: token_symbol,
            uri: token_uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        false, // Is mutable
        true,  // Update authority is signer
        None,  // Collection details
    )?;

    // Initial Lamports rent exemption for collateral account = 890880
    let min_collateral = 890880;

    // I64F64::from_num(min_collateral)
    //     .checked_div(
    //         I64F64::from_num(51)
    //             .checked_div(I64F64::from_num(100))
    //             .unwrap(),
    //     )
    //     .unwrap()
    //     .sqrt()
    //     .to_num::<u64>();

    msg!("Initial: {}", min_token);

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.collateral_account.to_account_info(),
            },
        ),
        min_collateral,
    )?;

    msg!("Token created successfully.");

    Ok(())
}