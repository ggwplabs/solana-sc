use crate::error::*;
use crate::state::{
    FreezingInfo, UserInfo, GPASS_MINT_AUTH_SEED, TREASURY_AUTH_SEED, USER_INFO_SEED,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use gpass::state::{GpassInfo, Wallet};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = FreezingInfo::LEN)]
    pub freezing_info: Box<Account<'info, FreezingInfo>>,

    /// CHECK: GPASS Mint auth PDA
    #[account(
        seeds = [
            GPASS_MINT_AUTH_SEED.as_bytes(),
            freezing_info.key().as_ref(),
            gpass_info.key().as_ref(),
        ],
        bump,
    )]
    pub gpass_mint_auth: UncheckedAccount<'info>,
    /// CHECK: Treasury auth PDA
    #[account(
        seeds = [
            TREASURY_AUTH_SEED.as_bytes(),
            freezing_info.key().as_ref(),
        ],
        bump,
    )]
    pub treasury_auth: UncheckedAccount<'info>,

    pub ggwp_token: Box<Account<'info, Mint>>,
    pub gpass_info: Box<Account<'info, GpassInfo>>,

    #[account(
        constraint = accumulative_fund.mint == ggwp_token.key()
        @FreezingError::InvalidAccumulativeFundMint,
    )]
    pub accumulative_fund: Box<Account<'info, TokenAccount>>,
    #[account(
        constraint = treasury.mint == ggwp_token.key()
        @FreezingError::InvalidTreasuryMint,
        constraint = treasury.owner == treasury_auth.key()
        @FreezingError::InvalidTreasuryOwner,
    )]
    pub treasury: Box<Account<'info, TokenAccount>>,

    // Misc.
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateParam<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub freezing_info: Account<'info, FreezingInfo>,
}

#[derive(Accounts)]
pub struct Freeze<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init_if_needed, payer = user, space = UserInfo::LEN,
        seeds = [
            USER_INFO_SEED.as_bytes(),
            freezing_info.key().as_ref(),
            user.key().as_ref(),
        ],
        bump,
    )]
    pub user_info: Box<Account<'info, UserInfo>>,

    #[account(mut)]
    pub freezing_info: Box<Account<'info, FreezingInfo>>,

    #[account(mut)]
    pub gpass_info: Box<Account<'info, GpassInfo>>,

    #[account(mut,
        constraint = user_ggwp_wallet.mint == freezing_info.ggwp_token
        @FreezingError::InvalidUserGGWPWalletMint,
        constraint = user_ggwp_wallet.owner == user.key()
        @FreezingError::InvalidUserGGWPWalletOwner,
    )]
    pub user_ggwp_wallet: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user_gpass_wallet: Box<Account<'info, Wallet>>,

    #[account(mut,
        constraint = treasury.key() == freezing_info.treasury
        @FreezingError::InvalidTreasuryPK,
    )]
    pub treasury: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        constraint = accumulative_fund.key() == freezing_info.accumulative_fund
        @FreezingError::InvalidAccumulativeFundPK,
    )]
    pub accumulative_fund: Box<Account<'info, TokenAccount>>,

    /// CHECK: Mint auth PDA
    #[account(
        seeds = [
            GPASS_MINT_AUTH_SEED.as_bytes(),
            freezing_info.key().as_ref(),
            gpass_info.key().as_ref(),
        ],
        bump = freezing_info.gpass_mint_auth_bump,
    )]
    pub gpass_mint_auth: UncheckedAccount<'info>,

    // Misc.
    /// CHECK: GPASS program
    #[account( constraint = gpass_program.key() == gpass::id() )]
    pub gpass_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub user: Signer<'info>,
    #[account(mut,
        seeds = [
            USER_INFO_SEED.as_bytes(),
            freezing_info.key().as_ref(),
            user.key().as_ref(),
        ],
        bump,
    )]
    pub user_info: Box<Account<'info, UserInfo>>,

    #[account(mut)]
    pub freezing_info: Box<Account<'info, FreezingInfo>>,

    #[account(mut)]
    pub gpass_info: Box<Account<'info, GpassInfo>>,
    #[account(mut)]
    pub user_gpass_wallet: Box<Account<'info, Wallet>>,

    /// CHECK: Mint auth PDA
    #[account(
        seeds = [
            GPASS_MINT_AUTH_SEED.as_bytes(),
            freezing_info.key().as_ref(),
            gpass_info.key().as_ref(),
        ],
        bump = freezing_info.gpass_mint_auth_bump,
    )]
    pub gpass_mint_auth: UncheckedAccount<'info>,

    // Misc.
    /// CHECK: GPASS program
    #[account( constraint = gpass_program.key() == gpass::id() )]
    pub gpass_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Unfreeze<'info> {
    pub user: Signer<'info>,
    #[account(mut,
        seeds = [
            USER_INFO_SEED.as_bytes(),
            freezing_info.key().as_ref(),
            user.key().as_ref(),
        ],
        bump,
    )]
    pub user_info: Box<Account<'info, UserInfo>>,

    #[account(mut)]
    pub freezing_info: Box<Account<'info, FreezingInfo>>,

    #[account(mut)]
    pub gpass_info: Box<Account<'info, GpassInfo>>,
    #[account(mut)]
    pub user_gpass_wallet: Box<Account<'info, Wallet>>,
    #[account(mut,
        constraint = user_ggwp_wallet.mint == freezing_info.ggwp_token
        @FreezingError::InvalidUserGGWPWalletMint,
        constraint = user_ggwp_wallet.owner == user.key()
        @FreezingError::InvalidUserGGWPWalletOwner,
    )]
    pub user_ggwp_wallet: Box<Account<'info, TokenAccount>>,

    /// CHECK: Mint auth PDA
    #[account(
        seeds = [
            GPASS_MINT_AUTH_SEED.as_bytes(),
            freezing_info.key().as_ref(),
            gpass_info.key().as_ref(),
        ],
        bump = freezing_info.gpass_mint_auth_bump,
    )]
    pub gpass_mint_auth: UncheckedAccount<'info>,

    #[account(mut,
        constraint = accumulative_fund.mint == freezing_info.ggwp_token.key()
        @FreezingError::InvalidAccumulativeFundMint,
    )]
    pub accumulative_fund: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        constraint = treasury.mint == freezing_info.ggwp_token.key()
        @FreezingError::InvalidTreasuryMint,
    )]
    pub treasury: Box<Account<'info, TokenAccount>>,

    /// CHECK: Treasury auth PDA
    #[account(
        seeds = [
            TREASURY_AUTH_SEED.as_bytes(),
            freezing_info.key().as_ref(),
        ],
        bump = freezing_info.treasury_auth_bump,
    )]
    pub treasury_auth: UncheckedAccount<'info>,

    // Misc.
    pub token_program: Program<'info, Token>,
    /// CHECK: GPASS program
    #[account( constraint = gpass_program.key() == gpass::id() )]
    pub gpass_program: AccountInfo<'info>,
}
