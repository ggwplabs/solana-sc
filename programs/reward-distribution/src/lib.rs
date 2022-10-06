use crate::context::*;
use crate::error::RewardDistributionError;
use crate::state::MAX_TRANSFER_AUTH_LIST;
use anchor_lang::prelude::*;

mod context;
mod error;
pub mod state;

declare_id!("5ihGT7nkjxfo1M43NZrPbbDBG4Js215ftJp6uksnNCEP");

#[program]
pub mod reward_distribution {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        update_auth: Pubkey,
        transfer_auth_list: Vec<Pubkey>,
    ) -> Result<()> {
        require!(
            transfer_auth_list.len() <= MAX_TRANSFER_AUTH_LIST,
            RewardDistributionError::InvalidTransferAuthList
        );

        let reward_distribution_info = &mut ctx.accounts.reward_distribution_info;
        reward_distribution_info.admin = ctx.accounts.admin.key();
        reward_distribution_info.update_auth = update_auth;
        reward_distribution_info.ggwp_token = ctx.accounts.ggwp_token.key();
        reward_distribution_info.play_to_earn_fund = ctx.accounts.play_to_earn_fund.key();
        reward_distribution_info.play_to_earn_fund_auth_bump = ctx.bumps["play_to_earn_fund_auth"];
        reward_distribution_info.transfer_auth_list = transfer_auth_list;

        Ok(())
    }

    /// Current admin can set another admin.
    pub fn update_admin(ctx: Context<UpdateParam>, admin: Pubkey) -> Result<()> {
        let reward_distribution_info = &mut ctx.accounts.reward_distribution_info;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            reward_distribution_info.admin,
            RewardDistributionError::AccessDenied
        );

        reward_distribution_info.admin = admin;

        Ok(())
    }

    /// Admin can set the new update authority.
    pub fn set_update_authority(ctx: Context<UpdateParam>, update_auth: Pubkey) -> Result<()> {
        let reward_distribution_info = &mut ctx.accounts.reward_distribution_info;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            reward_distribution_info.admin,
            RewardDistributionError::AccessDenied
        );

        reward_distribution_info.update_auth = update_auth;

        Ok(())
    }

    /// Update auth can set the new transfer authority list.
    pub fn update_transfer_authority_list(
        ctx: Context<UpdateParam>,
        transfer_auth_list: Vec<Pubkey>,
    ) -> Result<()> {
        let reward_distribution_info = &mut ctx.accounts.reward_distribution_info;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            reward_distribution_info.update_auth,
            RewardDistributionError::AccessDenied
        );

        require!(
            transfer_auth_list.len() <= MAX_TRANSFER_AUTH_LIST,
            RewardDistributionError::InvalidTransferAuthList
        );

        reward_distribution_info.transfer_auth_list = transfer_auth_list;

        Ok(())
    }
}
