use crate::error::*;
use crate::state::{FreezingParams, UserInfo, USER_INFO_SEED};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::token::{Mint, Token, TokenAccount};

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
        @FreezingError::InvalidGPASSMintAuth
    )]
    pub gpass_token: Account<'info, Mint>,

    #[account(
        constraint = accumulative_fund.mint == ggwp_token.key()
        @FreezingError::InvalidAccumulativeFundMint,
    )]
    pub accumulative_fund: Account<'info, TokenAccount>,
    #[account(
        constraint = treasury.mint == ggwp_token.key()
        @FreezingError::InvalidTreasuryMint,
    )]
    pub treasury: Account<'info, TokenAccount>,

    // Misc.
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ChangeAdmin<'info> {
    pub admin: Signer<'info>,
    #[account(mut,
        constraint = freezing_params.admin == admin.key()
        @FreezingError::AccessDenied,
    )]
    pub freezing_params: Account<'info, FreezingParams>,
    pub new_admin: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct ChangeUpdateAuth<'info> {
    pub admin: Signer<'info>,
    #[account(mut,
        constraint = freezing_params.admin == admin.key()
        @FreezingError::AccessDenied,
    )]
    pub freezing_params: Account<'info, FreezingParams>,
}

#[derive(Accounts)]
pub struct ChangeParams<'info> {
    pub update_auth: Signer<'info>,
    #[account(mut,
        constraint = freezing_params.update_auth == update_auth.key()
        @FreezingError::AccessDenied,
    )]
    pub freezing_params: Account<'info, FreezingParams>,
}

#[derive(Accounts)]
pub struct Freeze<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init_if_needed, payer = user, space = UserInfo::LEN,
        seeds = [
            program_id.as_ref(),
            freezing_params.key().as_ref(),
            user.key().as_ref(),
            USER_INFO_SEED.as_bytes(),
        ],
        bump,
    )]
    pub user_info: Account<'info, UserInfo>,

    pub freezing_params: Account<'info, FreezingParams>,

    #[account(mut,
        constraint = gpass_token.key() == freezing_params.gpass_token,
    )]
    pub gpass_token: Account<'info, Mint>,

    #[account(mut,
        constraint = user_ggwp_wallet.mint == freezing_params.ggwp_token
        @FreezingError::InvalidUserGGWPWalletMint,
        constraint = user_ggwp_wallet.owner == user.key()
        @FreezingError::InvalidUserGGWPWalletOwner,
    )]
    pub user_ggwp_wallet: Account<'info, TokenAccount>,
    #[account(mut,
        constraint = user_gpass_wallet.mint == freezing_params.gpass_token
        @FreezingError::InvalidUserGPASSWalletMint,
        constraint = user_gpass_wallet.owner == user.key()
        @FreezingError::InvalidUserGPASSWalletOwner,
    )]
    pub user_gpass_wallet: Account<'info, TokenAccount>,

    #[account(mut,
        constraint = treasury.key() == freezing_params.treasury
        @FreezingError::InvalidTreasuryPK,
    )]
    pub treasury: Account<'info, TokenAccount>,

    /// CHECK: Mint auth PDA
    #[account(
        seeds = [
            program_id.as_ref(),
            freezing_params.to_account_info().key.as_ref(),
            gpass_mint_auth.key().as_ref()
        ],
        bump = freezing_params.gpass_mint_auth_bump,
    )]
    pub gpass_mint_auth: UncheckedAccount<'info>,

    // Misc.
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub user: Signer<'info>,
    #[account(mut,
        seeds = [
            program_id.as_ref(),
            freezing_params.key().as_ref(),
            user.key().as_ref(),
            USER_INFO_SEED.as_bytes(),
        ],
        bump,
    )]
    pub user_info: Account<'info, UserInfo>,

    pub freezing_params: Account<'info, FreezingParams>,

    #[account(mut,
        constraint = gpass_token.key() == freezing_params.gpass_token,
    )]
    pub gpass_token: Account<'info, Mint>,

    #[account(mut,
        constraint = user_gpass_wallet.mint == freezing_params.gpass_token
        @FreezingError::InvalidUserGPASSWalletMint,
        constraint = user_gpass_wallet.owner == user.key()
        @FreezingError::InvalidUserGPASSWalletOwner,
    )]
    pub user_gpass_wallet: Account<'info, TokenAccount>,

    /// CHECK: Mint auth PDA
    #[account(
        seeds = [
            program_id.as_ref(),
            freezing_params.to_account_info().key.as_ref(),
            gpass_mint_auth.key().as_ref()
        ],
        bump = freezing_params.gpass_mint_auth_bump,
    )]
    pub gpass_mint_auth: UncheckedAccount<'info>,

    // Misc.
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Unfreeze<'info> {
    pub user: Signer<'info>,
    #[account(mut,
        seeds = [
            program_id.as_ref(),
            freezing_params.key().as_ref(),
            user.key().as_ref(),
            USER_INFO_SEED.as_bytes(),
        ],
        bump,
    )]
    pub user_info: Account<'info, UserInfo>,

    pub freezing_params: Account<'info, FreezingParams>,

    #[account(mut,
        constraint = gpass_token.key() == freezing_params.gpass_token,
    )]
    pub gpass_token: Account<'info, Mint>,

    #[account(mut,
        constraint = user_gpass_wallet.mint == freezing_params.gpass_token
        @FreezingError::InvalidUserGPASSWalletMint,
        constraint = user_gpass_wallet.owner == user.key()
        @FreezingError::InvalidUserGPASSWalletOwner,
    )]
    pub user_gpass_wallet: Account<'info, TokenAccount>,

    /// CHECK: Mint auth PDA
    #[account(
        seeds = [
            program_id.as_ref(),
            freezing_params.to_account_info().key.as_ref(),
            gpass_mint_auth.key().as_ref()
        ],
        bump = freezing_params.gpass_mint_auth_bump,
    )]
    pub gpass_mint_auth: UncheckedAccount<'info>,

    // Misc.
    pub token_program: Program<'info, Token>,
}
