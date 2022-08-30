use crate::error::DistributionError;
use crate::state::{DistributionInfo, ACCUMULATIVE_FUND_AUTH_SEED};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = DistributionInfo::LEN)]
    pub distribution_info: Box<Account<'info, DistributionInfo>>,

    pub ggwp_token: Account<'info, Mint>,
    /// CHECK: Accumulative auth account
    #[account(
        seeds = [
            ACCUMULATIVE_FUND_AUTH_SEED.as_bytes(),
            distribution_info.key().as_ref(),
            accumulative_fund.key().as_ref(),
        ],
        bump
    )]
    pub accumulative_fund_auth: UncheckedAccount<'info>,
    #[account(
        constraint = accumulative_fund.mint == ggwp_token.key()
        @DistributionError::InvalidAccumulativeFundMint,
        constraint = accumulative_fund.owner == accumulative_fund_auth.key()
        @DistributionError::InvalidAccumulativeFundOwner,
    )]
    pub accumulative_fund: Box<Account<'info, TokenAccount>>,

    #[account(
        constraint = play_to_earn_fund.mint == ggwp_token.key()
        @DistributionError::InvalidPlayToEarnFundMint,
    )]
    pub play_to_earn_fund: Box<Account<'info, TokenAccount>>,
    #[account(
        constraint = staking_fund.mint == ggwp_token.key()
        @DistributionError::InvalidStakingFundMint,
    )]
    pub staking_fund: Box<Account<'info, TokenAccount>>,
    #[account(
        constraint = company_fund.mint == ggwp_token.key()
        @DistributionError::InvalidCompanyFundMint,
    )]
    pub company_fund: Box<Account<'info, TokenAccount>>,
    #[account(
        constraint = team_fund.mint == ggwp_token.key()
        @DistributionError::InvalidTeamFundMint,
    )]
    pub team_fund: Box<Account<'info, TokenAccount>>,

    // Misc.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Distribute<'info> {
    #[account(mut)]
    pub distribution_info: Box<Account<'info, DistributionInfo>>,

    /// CHECK: Accumulative auth account
    #[account(
        seeds = [
            ACCUMULATIVE_FUND_AUTH_SEED.as_bytes(),
            distribution_info.key().as_ref(),
            accumulative_fund.key().as_ref(),
        ],
        bump = distribution_info.accumulative_fund_auth_bump,
    )]
    pub accumulative_fund_auth: UncheckedAccount<'info>,
    #[account(mut,
        constraint = accumulative_fund.mint == distribution_info.ggwp_token.key()
        @DistributionError::InvalidAccumulativeFundMint,
        constraint = accumulative_fund.owner == accumulative_fund_auth.key()
        @DistributionError::InvalidAccumulativeFundOwner,
        constraint = accumulative_fund.key() == distribution_info.accumulative_fund
        @DistributionError::InvalidFundPublicKey,
    )]
    pub accumulative_fund: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        constraint = play_to_earn_fund.mint == distribution_info.ggwp_token.key()
        @DistributionError::InvalidPlayToEarnFundMint,
        constraint = play_to_earn_fund.key() == distribution_info.play_to_earn_fund
        @DistributionError::InvalidFundPublicKey,
    )]
    pub play_to_earn_fund: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        constraint = staking_fund.mint == distribution_info.ggwp_token.key()
        @DistributionError::InvalidStakingFundMint,
        constraint = staking_fund.key() == distribution_info.staking_fund
        @DistributionError::InvalidFundPublicKey,
    )]
    pub staking_fund: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        constraint = company_fund.mint == distribution_info.ggwp_token.key()
        @DistributionError::InvalidCompanyFundMint,
        constraint = company_fund.key() == distribution_info.company_fund
        @DistributionError::InvalidFundPublicKey,
    )]
    pub company_fund: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        constraint = team_fund.mint == distribution_info.ggwp_token.key()
        @DistributionError::InvalidTeamFundMint,
        constraint = team_fund.key() == distribution_info.team_fund
        @DistributionError::InvalidFundPublicKey,
    )]
    pub team_fund: Box<Account<'info, TokenAccount>>,

    // Misc.
    pub token_program: Program<'info, Token>,
}
