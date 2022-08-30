use crate::error::DistributionError;
use crate::state::{DistributionInfo, ACCUMULATIVE_FUND_AUTH_SEED};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = DistributionInfo::LEN)]
    pub distribution_info: Account<'info, DistributionInfo>,

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
    pub accumulative_auth: UncheckedAccount<'info>,
    #[account(
        constraint = accumulative_fund.mint == ggwp_token.key()
        @DistributionError::InvalidAccumulativeFundMint,
        constraint = accumulative_fund.owner == accumulative_auth.key()
        @DistributionError::InvalidAccumulativeFundOwner,
    )]
    pub accumulative_fund: Account<'info, TokenAccount>,

    #[account(
        constraint = play_to_earn_fund.mint == ggwp_token.key()
        @DistributionError::InvalidPlayToEarnFundMint,
    )]
    pub play_to_earn_fund: Account<'info, TokenAccount>,
    #[account(
        constraint = staking_fund.mint == ggwp_token.key()
        @DistributionError::InvalidStakingFundMint,
    )]
    pub staking_fund: Account<'info, TokenAccount>,
    #[account(
        constraint = company_fund.mint == ggwp_token.key()
        @DistributionError::InvalidCompanyFundMint,
    )]
    pub company_fund: Account<'info, TokenAccount>,
    #[account(
        constraint = team_fund.mint == ggwp_token.key()
        @DistributionError::InvalidTeamFundMint,
    )]
    pub team_fund: Account<'info, TokenAccount>,

    // Misc.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Distribute {}
