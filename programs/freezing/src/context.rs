use crate::error::*;
use crate::state::FreezingParams;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::token::{Mint, TokenAccount, Token};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = FreezingParams::LEN)]
    pub freezing_params: Account<'info, FreezingParams>,

    /// CHECK: New mint auth PDA
    #[account(
        seeds = [
            program_id.as_ref(),
            freezing_params.to_account_info().key.as_ref(),
            gpass_mint_auth.key().as_ref()
        ],
        bump,
    )]
    pub gpass_mint_auth: UncheckedAccount<'info>,

    pub ggwp_token: Account<'info, Mint>,
    #[account(mut,
        constraint = gpass_token.mint_authority == COption::Some(admin.key())
        @ FreezingError::InvalidGPASSMintAuth
    )]
    pub gpass_token: Account<'info, Mint>,
    #[account(
        constraint = accumulative_fund.mint == ggwp_token.key()
        @ FreezingError::InvalidAccumulativeFundMint
    )]
    pub accumulative_fund: Account<'info, TokenAccount>,

    // Misc.
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ChangeAdmin<'info> {
    pub admin: Signer<'info>,
    #[account(mut,
        constraint = freezing_params.admin == admin.key()
        @ FreezingError::AccessDenied
    )]
    pub freezing_params: Account<'info, FreezingParams>,
    pub new_admin: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct ChangeParams {}

#[derive(Accounts)]
pub struct Freeze {}

#[derive(Accounts)]
pub struct Withdraw {}

#[derive(Accounts)]
pub struct Unfreeze {}
