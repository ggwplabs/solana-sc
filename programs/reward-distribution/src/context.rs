use crate::error::RewardDistributionError;
use crate::state::{RewardDistributionInfo, PLAY_TO_EARN_FUND_AUTH_SEED};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = RewardDistributionInfo::LEN)]
    pub reward_distribution_info: Account<'info, RewardDistributionInfo>,

    pub ggwp_token: Account<'info, Mint>,
    /// CHECK: Accumulative auth account
    #[account(
        seeds = [
            PLAY_TO_EARN_FUND_AUTH_SEED.as_bytes(),
            reward_distribution_info.key().as_ref(),
        ],
        bump
    )]
    pub play_to_earn_fund_auth: UncheckedAccount<'info>,
    #[account(
        constraint = play_to_earn_fund.mint == ggwp_token.key()
        @RewardDistributionError::InvalidPlayToEarnFundMint,
        constraint = play_to_earn_fund.owner == play_to_earn_fund_auth.key()
        @RewardDistributionError::InvalidPlayToEarnFundOwner,
    )]
    pub play_to_earn_fund: Box<Account<'info, TokenAccount>>,

    // Misc.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateParam<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub reward_distribution_info: Account<'info, RewardDistributionInfo>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    pub authority: Signer<'info>,

    pub reward_distribution_info: Box<Account<'info, RewardDistributionInfo>>,

    #[account(mut,
        constraint = play_to_earn_fund.mint == reward_distribution_info.ggwp_token
        @RewardDistributionError::InvalidPlayToEarnFundMint,
        constraint = play_to_earn_fund.owner == play_to_earn_fund_auth.key()
        @RewardDistributionError::InvalidPlayToEarnFundOwner,
    )]
    pub play_to_earn_fund: Box<Account<'info, TokenAccount>>,
    /// CHECK: Accumulative auth account
    #[account(
        seeds = [
            PLAY_TO_EARN_FUND_AUTH_SEED.as_bytes(),
            reward_distribution_info.key().as_ref(),
        ],
        bump = reward_distribution_info.play_to_earn_fund_auth_bump,
    )]
    pub play_to_earn_fund_auth: UncheckedAccount<'info>,

    #[account(mut,
        constraint = to.mint == reward_distribution_info.ggwp_token,
    )]
    pub to: Box<Account<'info, TokenAccount>>,

    // Misc.
    pub token_program: Program<'info, Token>,
}
