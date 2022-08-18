use crate::error::StakingError;
use crate::state::{StakingInfo, STAKING_FUND_AUTH_SEED, TREASURY_AUTH_SEED};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = StakingInfo::LEN)]
    pub staking_info: Account<'info, StakingInfo>,

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
