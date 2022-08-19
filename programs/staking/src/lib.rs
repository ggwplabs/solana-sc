use crate::context::*;
use crate::error::StakingError;
use anchor_lang::prelude::*;

mod context;
mod error;
pub mod state;

declare_id!("DJQcSKGPXre9ZMJHGxdZhGYwKGBpEaQHPUpRzLuqhYWY");

#[program]
pub mod staking {
    use super::*;

    /// Initialize new staking info account with params.
    pub fn initialize(
        ctx: Context<Initialize>,
        update_auth: Pubkey,
        epoch_period_days: u16,
        min_stake_amount: u64,
        hold_period_days: u16,
        hold_royalty: u8,
        royalty: u8,
        apr_start: u8,
        apr_step: u8,
        apr_end: u8,
    ) -> Result<()> {
        require_neq!(epoch_period_days, 0, StakingError::InvalidEpochPeriodDays);
        require_neq!(min_stake_amount, 0, StakingError::InvalidMinStakeAmount);
        require_neq!(hold_period_days, 0, StakingError::InvalidHoldPeriodDays);
        require_neq!(hold_royalty, 0, StakingError::InvalidHoldRoyalty);
        require!(hold_royalty <= 100, StakingError::InvalidHoldRoyalty);
        require_neq!(royalty, 0, StakingError::InvalidRoyalty);
        require!(royalty <= 100, StakingError::InvalidRoyalty);
        require_neq!(apr_start, 0, StakingError::InvalidAPR);
        require_neq!(apr_step, 0, StakingError::InvalidAPR);
        require_neq!(apr_end, 0, StakingError::InvalidAPR);

        let staking_info = &mut ctx.accounts.staking_info;
        staking_info.admin = ctx.accounts.admin.key();
        staking_info.update_auth = update_auth;

        staking_info.ggwp_token = ctx.accounts.ggwp_token.key();

        staking_info.accumulative_fund = ctx.accounts.accumulative_fund.key();
        staking_info.staking_fund = ctx.accounts.staking_fund.key();
        staking_info.staking_fund_auth_bump = ctx.bumps["staking_fund_auth"];
        staking_info.treasury = ctx.accounts.treasury.key();
        staking_info.treasury_auth_bump = ctx.bumps["treasury_auth"];

        staking_info.total_staked = 0;
        staking_info.epoch = 1;
        staking_info.epoch_period_days = epoch_period_days;
        staking_info.min_stake_amount = min_stake_amount;
        staking_info.hold_period_days = hold_period_days;
        staking_info.hold_royalty = hold_royalty;
        staking_info.royalty = royalty;
        staking_info.apr_start = apr_start;
        staking_info.apr_step = apr_step;
        staking_info.apr_end = apr_end;

        Ok(())
    }

    /// Current admin can set the new admin.
    pub fn update_admin(ctx: Context<UpdateParam>, admin: Pubkey) -> Result<()> {
        let authority = &ctx.accounts.authority;
        let staking_info = &mut ctx.accounts.staking_info;

        require_keys_eq!(
            authority.key(),
            staking_info.admin,
            StakingError::AccessDenied
        );
        staking_info.admin = admin;

        Ok(())
    }

    /// Admin can set the new update authority
    pub fn set_update_authority(ctx: Context<UpdateParam>, update_auth: Pubkey) -> Result<()> {
        let authority = &ctx.accounts.authority;
        let staking_info = &mut ctx.accounts.staking_info;

        require_keys_eq!(
            authority.key(),
            staking_info.admin,
            StakingError::AccessDenied
        );
        staking_info.update_auth = update_auth;

        Ok(())
    }

    /// Update authority can set new epoch period in days.
    pub fn update_epoch_period_days(
        ctx: Context<UpdateParam>,
        epoch_period_days: u16,
    ) -> Result<()> {
        let authority = &ctx.accounts.authority;
        let staking_info = &mut ctx.accounts.staking_info;

        require!(epoch_period_days != 0, StakingError::InvalidEpochPeriodDays);
        require_keys_eq!(
            authority.key(),
            staking_info.update_auth,
            StakingError::AccessDenied
        );
        staking_info.epoch_period_days = epoch_period_days;

        Ok(())
    }

    /// Update authority can set new min stake amount.
    pub fn update_min_stake_amount(ctx: Context<UpdateParam>, min_stake_amount: u64) -> Result<()> {
        let authority = &ctx.accounts.authority;
        let staking_info = &mut ctx.accounts.staking_info;

        require!(min_stake_amount != 0, StakingError::InvalidMinStakeAmount);
        require_keys_eq!(
            authority.key(),
            staking_info.update_auth,
            StakingError::AccessDenied
        );
        staking_info.min_stake_amount = min_stake_amount;

        Ok(())
    }

    /// Update authority can set new hold period in days.
    pub fn update_hold_period_days(ctx: Context<UpdateParam>, hold_period_days: u16) -> Result<()> {
        let authority = &ctx.accounts.authority;
        let staking_info = &mut ctx.accounts.staking_info;

        require!(hold_period_days != 0, StakingError::InvalidHoldPeriodDays);
        require_keys_eq!(
            authority.key(),
            staking_info.update_auth,
            StakingError::AccessDenied
        );
        staking_info.hold_period_days = hold_period_days;

        Ok(())
    }

    /// Update authority can set new hold royalty in percent.
    pub fn update_hold_royalty(ctx: Context<UpdateParam>, hold_royalty: u8) -> Result<()> {
        let authority = &ctx.accounts.authority;
        let staking_info = &mut ctx.accounts.staking_info;

        require!(hold_royalty != 0, StakingError::InvalidHoldRoyalty);
        require!(hold_royalty <= 100, StakingError::InvalidHoldRoyalty);
        require_keys_eq!(
            authority.key(),
            staking_info.update_auth,
            StakingError::AccessDenied
        );
        staking_info.hold_royalty = hold_royalty;

        Ok(())
    }

    /// Update authority can set new royalty in percent.
    pub fn update_royalty(ctx: Context<UpdateParam>, royalty: u8) -> Result<()> {
        let authority = &ctx.accounts.authority;
        let staking_info = &mut ctx.accounts.staking_info;

        require!(royalty != 0, StakingError::InvalidRoyalty);
        require!(royalty <= 100, StakingError::InvalidRoyalty);
        require_keys_eq!(
            authority.key(),
            staking_info.update_auth,
            StakingError::AccessDenied
        );
        staking_info.royalty = royalty;

        Ok(())
    }

    // TODO: updates, stake, unstake
}
