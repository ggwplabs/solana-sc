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
}
