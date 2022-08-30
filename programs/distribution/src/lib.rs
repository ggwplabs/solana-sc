use crate::context::*;
use crate::error::DistributionError;
use anchor_lang::prelude::*;

declare_id!("79GShMQgEBcfpiiwkBxv3yBxHqCN8J2E8DhivatqpfYC");

mod context;
mod error;
pub mod state;

#[program]
pub mod distribution {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        update_auth: Pubkey,
        play_to_earn_fund_share: u8,
        staking_fund_share: u8,
        company_fund_share: u8,
        team_fund_share: u8,
    ) -> Result<()> {
        require!(
            play_to_earn_fund_share <= 100,
            DistributionError::InvalidShare
        );
        require!(staking_fund_share <= 100, DistributionError::InvalidShare);
        require!(company_fund_share <= 100, DistributionError::InvalidShare);
        require!(team_fund_share <= 100, DistributionError::InvalidShare);

        let distribution_info = &mut ctx.accounts.distribution_info;
        distribution_info.admin = ctx.accounts.admin.key();
        distribution_info.update_auth = update_auth;

        distribution_info.ggwp_token = ctx.accounts.ggwp_token.key();
        distribution_info.accumulative_fund = ctx.accounts.accumulative_fund.key();
        distribution_info.accumulative_fund_auth_bump = ctx.bumps["accumulative_fund_auth"];
        
        distribution_info.play_to_earn_fund = ctx.accounts.play_to_earn_fund.key();
        distribution_info.play_to_earn_fund_share = play_to_earn_fund_share;
        distribution_info.staking_fund = ctx.accounts.staking_fund.key();
        distribution_info.staking_fund_share = staking_fund_share;
        distribution_info.company_fund = ctx.accounts.company_fund.key();
        distribution_info.company_fund_share = company_fund_share;
        distribution_info.team_fund = ctx.accounts.team_fund.key();
        distribution_info.team_fund_share = team_fund_share;

        Ok(())
    }

    pub fn distribute(ctx: Context<Distribute>) -> Result<()> {
        Ok(())
    }
}
