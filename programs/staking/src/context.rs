use crate::error::StakingError;
use crate::state::{
    StakingInfo, UserInfo, STAKING_FUND_AUTH_SEED, TREASURY_AUTH_SEED, USER_INFO_SEED,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = StakingInfo::LEN)]
    pub staking_info: Box<Account<'info, StakingInfo>>,

    /// CHECK: Treasury auth PDA
    #[account(
        seeds = [
            TREASURY_AUTH_SEED.as_bytes(),
            staking_info.key().as_ref(),
        ],
        bump,
    )]
    pub treasury_auth: UncheckedAccount<'info>,
    /// CHECK: Staking fund auth PDA
    #[account(
        seeds = [
            STAKING_FUND_AUTH_SEED.as_bytes(),
            staking_info.key().as_ref(),
        ],
        bump,
    )]
    pub staking_fund_auth: UncheckedAccount<'info>,

    pub ggwp_token: Box<Account<'info, Mint>>,
    #[account(
        constraint = accumulative_fund.mint == ggwp_token.key()
        @StakingError::InvalidAccumulativeFundMint,
    )]
    pub accumulative_fund: Box<Account<'info, TokenAccount>>,
    #[account(
        constraint = treasury.mint == ggwp_token.key()
        @StakingError::InvalidTreasuryMint,
        constraint = treasury.owner == treasury_auth.key()
        @StakingError::InvalidTreasuryOwner,
    )]
    pub treasury: Box<Account<'info, TokenAccount>>,
    #[account(
        constraint = staking_fund.mint == ggwp_token.key()
        @StakingError::InvalidStakingFundMint,
        constraint = staking_fund.owner == staking_fund_auth.key()
        @StakingError::InvalidStakingFundOwner,
    )]
    pub staking_fund: Box<Account<'info, TokenAccount>>,

    // Misc.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateParam<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub staking_info: Account<'info, StakingInfo>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub staking_info: Box<Account<'info, StakingInfo>>,

    #[account(init_if_needed, payer = user, space = UserInfo::LEN,
        seeds = [
            USER_INFO_SEED.as_bytes(),
            staking_info.key().as_ref(),
            user.key().as_ref(),
        ],
        bump
    )]
    pub user_info: Box<Account<'info, UserInfo>>,

    #[account(mut,
        constraint = user_ggwp_wallet.mint == staking_info.ggwp_token
        @StakingError::InvalidUserGGWPWalletMint,
        constraint = user_ggwp_wallet.owner == user.key()
        @StakingError::InvalidUserGGWPWalletOwner,
    )]
    pub user_ggwp_wallet: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        constraint = treasury.key() == staking_info.treasury
        @StakingError::InvalidTreasuryPK,
    )]
    pub treasury: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        constraint = accumulative_fund.key() == staking_info.accumulative_fund
        @StakingError::InvalidAccumulativeFundPK,
    )]
    pub accumulative_fund: Box<Account<'info, TokenAccount>>,

    // Misc.
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub staking_info: Box<Account<'info, StakingInfo>>,

    #[account(init_if_needed, payer = user, space = UserInfo::LEN,
        seeds = [
            USER_INFO_SEED.as_bytes(),
            staking_info.key().as_ref(),
            user.key().as_ref(),
        ],
        bump
    )]
    pub user_info: Box<Account<'info, UserInfo>>,

    #[account(mut,
        constraint = user_ggwp_wallet.mint == staking_info.ggwp_token
        @StakingError::InvalidUserGGWPWalletMint,
        constraint = user_ggwp_wallet.owner == user.key()
        @StakingError::InvalidUserGGWPWalletOwner,
    )]
    pub user_ggwp_wallet: Box<Account<'info, TokenAccount>>,

    /// CHECK: Treasury auth PDA
    #[account(
        seeds = [
            TREASURY_AUTH_SEED.as_bytes(),
            staking_info.key().as_ref(),
        ],
        bump,
    )]
    pub treasury_auth: UncheckedAccount<'info>,

    /// CHECK: Staking fund auth PDA
    #[account(
        seeds = [
            STAKING_FUND_AUTH_SEED.as_bytes(),
            staking_info.key().as_ref(),
        ],
        bump,
    )]
    pub staking_fund_auth: UncheckedAccount<'info>,

    #[account(mut,
        constraint = treasury.key() == staking_info.treasury
        @StakingError::InvalidTreasuryPK,
    )]
    pub treasury: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        constraint = accumulative_fund.key() == staking_info.accumulative_fund
        @StakingError::InvalidAccumulativeFundPK,
    )]
    pub accumulative_fund: Box<Account<'info, TokenAccount>>,

    #[account(
        constraint = staking_fund.mint == staking_info.ggwp_token
        @StakingError::InvalidStakingFundMint,
        constraint = staking_fund.owner == staking_fund_auth.key()
        @StakingError::InvalidStakingFundOwner,
    )]
    pub staking_fund: Box<Account<'info, TokenAccount>>,

    // Misc.
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
