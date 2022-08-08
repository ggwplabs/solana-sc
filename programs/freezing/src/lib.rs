use crate::error::FreezingError;
use crate::state::RewardTableRow;
use anchor_lang::prelude::*;
use anchor_spl::token::{MintTo, SetAuthority, Transfer};
use context::*;

mod context;
mod error;
mod state;
mod utils;

declare_id!("ABHUowgjyTkmbMRRuMYJ5ui4wAz6Z6HE4PQMHy9YqMrQ");

#[program]
pub mod freezing {
    use super::*;

    /// Initialize new freezing params with tokens PKs, and parameters.
    /// Note: Need to add the mint auth into minters list in GPASS
    pub fn initialize(
        ctx: Context<Initialize>,
        update_auth: Pubkey,
        royalty: u8,
        unfreeze_royalty: u8,
        unfreeze_lock_time: i64,
        reward_table: Vec<RewardTableRow>,
    ) -> Result<()> {
        require!(royalty <= 100, FreezingError::InvalidRoyaltyValue);
        require!(
            unfreeze_royalty <= 100,
            FreezingError::InvalidUnfreezeRoyaltyValue
        );
        require!(
            unfreeze_lock_time != 0,
            FreezingError::InvalidUnfreezeLockTime
        );

        let freezing_params = &mut ctx.accounts.freezing_params;
        freezing_params.admin = ctx.accounts.admin.key();
        freezing_params.update_auth = update_auth;

        freezing_params.ggwp_token = ctx.accounts.ggwp_token.key();
        freezing_params.gpass_settings = ctx.accounts.gpass_settings.key();
        freezing_params.gpass_mint_auth_bump = ctx.bumps["gpass_mint_auth"];

        freezing_params.accumulative_fund = ctx.accounts.accumulative_fund.key();
        freezing_params.treasury = ctx.accounts.treasury.key();

        freezing_params.total_freezed = 0;
        freezing_params.royalty = royalty;
        freezing_params.unfreeze_royalty = unfreeze_royalty;
        freezing_params.unfreeze_lock_time = unfreeze_lock_time;
        freezing_params.reward_table = reward_table;

        Ok(())
    }

    /// Current admin can set another admin.
    pub fn change_admin(ctx: Context<UpdateParams>, admin: Pubkey) -> Result<()> {
        let freezing_params = &mut ctx.accounts.freezing_params;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            freezing_params.admin,
            FreezingError::AccessDenied
        );

        freezing_params.admin = admin;

        Ok(())
    }

    /// Admin can set the new update authority.
    pub fn set_update_authority(ctx: Context<UpdateParams>, update_auth: Pubkey) -> Result<()> {
        let freezing_params = &mut ctx.accounts.freezing_params;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            freezing_params.admin,
            FreezingError::AccessDenied
        );

        freezing_params.update_auth = update_auth;

        Ok(())
    }

    pub fn update_royalty(ctx: Context<UpdateParams>, royalty: u8) -> Result<()> {
        require!(royalty <= 100, FreezingError::InvalidRoyaltyValue);

        let freezing_params = &mut ctx.accounts.freezing_params;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            freezing_params.update_auth,
            FreezingError::AccessDenied
        );

        freezing_params.royalty = royalty;

        Ok(())
    }

    pub fn update_unfreeze_royalty(ctx: Context<UpdateParams>, unfreeze_royalty: u8) -> Result<()> {
        require!(
            unfreeze_royalty <= 100,
            FreezingError::InvalidUnfreezeRoyaltyValue
        );

        let freezing_params = &mut ctx.accounts.freezing_params;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            freezing_params.update_auth,
            FreezingError::AccessDenied
        );

        freezing_params.unfreeze_royalty = unfreeze_royalty;

        Ok(())
    }

    pub fn update_reward_table(ctx: Context<UpdateParams>, reward_table: Vec<RewardTableRow>) -> Result<()> {
        let freezing_params = &mut ctx.accounts.freezing_params;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            freezing_params.update_auth,
            FreezingError::AccessDenied
        );

        freezing_params.reward_table = reward_table;

        Ok(())
    }

    /// User freezes his amount of GGWP token to get the GPASS tokens.
    pub fn freeze(ctx: Context<Freeze>, amount: u64) -> Result<()> {
        let user = &ctx.accounts.user;
        // let freezing_params = &ctx.accounts.freezing_params;
        // let treasury = &mut ctx.accounts.treasury;
        // let user_info = &mut ctx.accounts.user_info;
        // let user_ggwp_wallet = &mut ctx.accounts.user_ggwp_wallet;
        // let user_gpass_wallet = &mut ctx.accounts.user_gpass_wallet;
        // let gpass_token = &mut ctx.accounts.gpass_token;
        // let gpass_mint_auth = &ctx.accounts.gpass_mint_auth;
        // let token_program = &ctx.accounts.token_program;
        // let clock = Clock::get()?;

        // require_neq!(amount, 0, FreezingError::ZeroFreezingAmount);

        // // TODO: fix freeze

        // // Init user info in needed
        // if !user_info.is_initialized {
        //     user_info.is_initialized = true;
        //     user_info.freezed_amount = 0;
        //     user_info.freezed_time = 0;
        //     user_info.last_getting_gpass = clock.unix_timestamp;
        // }

        // // Pay current GPASS earned by user
        // let gpass_earned = utils::calc_earned_gpass(&clock, user_info.last_getting_gpass)?;
        // msg!("Earned GPASS: {}", gpass_earned);
        // if gpass_earned > 0 {
        //     user_info.last_getting_gpass = clock.unix_timestamp;
        //     // Mint GPASS tokens to user
        //     let seeds = &[
        //         ctx.program_id.as_ref(),
        //         freezing_params.to_account_info().key.as_ref(),
        //         &[freezing_params.gpass_mint_auth_bump],
        //     ];
        //     let signer = &[&seeds[..]];
        //     anchor_spl::token::mint_to(
        //         CpiContext::new_with_signer(
        //             token_program.to_account_info(),
        //             MintTo {
        //                 mint: gpass_token.to_account_info(),
        //                 authority: gpass_mint_auth.to_account_info(),
        //                 to: user_gpass_wallet.to_account_info(),
        //             },
        //             signer,
        //         ),
        //         gpass_earned,
        //     )?;
        // }

        // // TODO: royalty

        // // Freeze additional GGWP
        // anchor_spl::token::transfer(
        //     CpiContext::new(
        //         token_program.to_account_info(),
        //         Transfer {
        //             from: user_ggwp_wallet.to_account_info(),
        //             to: treasury.to_account_info(),
        //             authority: user.to_account_info(),
        //         },
        //     ),
        //     amount,
        // )?;

        // user_info.freezed_amount = user_info
        //     .freezed_amount
        //     .checked_add(amount)
        //     .ok_or(FreezingError::Overflow)?;
        // // TODO: second deposit?
        // user_info.freezed_time = clock.unix_timestamp;

        Ok(())
    }

    /// In every time user can withdraw GPASS earned.
    pub fn withdraw_gpass(ctx: Context<Withdraw>) -> Result<()> {
        // let freezing_params = &ctx.accounts.freezing_params;
        // let user_info = &mut ctx.accounts.user_info;
        // let user_gpass_wallet = &mut ctx.accounts.user_gpass_wallet;
        // let gpass_token = &mut ctx.accounts.gpass_token;
        // let gpass_mint_auth = &ctx.accounts.gpass_mint_auth;
        // let token_program = &ctx.accounts.token_program;
        // let clock = Clock::get()?;

        // // Pay current GPASS earned by user
        // let gpass_earned = utils::calc_earned_gpass(&clock, user_info.last_getting_gpass)?;
        // // TODO: if gpass_earned == 0 -> error?
        // msg!("Earned GPASS: {}", gpass_earned);
        // if gpass_earned > 0 {
        //     user_info.last_getting_gpass = clock.unix_timestamp;
        //     // Mint GPASS tokens to user
        //     let seeds = &[
        //         ctx.program_id.as_ref(),
        //         freezing_params.to_account_info().key.as_ref(),
        //         &[freezing_params.gpass_mint_auth_bump],
        //     ];
        //     let signer = &[&seeds[..]];
        //     anchor_spl::token::mint_to(
        //         CpiContext::new_with_signer(
        //             token_program.to_account_info(),
        //             MintTo {
        //                 mint: gpass_token.to_account_info(),
        //                 authority: gpass_mint_auth.to_account_info(),
        //                 to: user_gpass_wallet.to_account_info(),
        //             },
        //             signer,
        //         ),
        //         gpass_earned,
        //     )?;
        // }

        Ok(())
    }

    // User unfreeze his amount of GGWP token.
    pub fn unfreeze(ctx: Context<Unfreeze>) -> Result<()> {
        // TODO
        // let freezing_params = &ctx.accounts.freezing_params;
        // let user_info = &mut ctx.accounts.user_info;
        // let user_gpass_wallet = &mut ctx.accounts.user_gpass_wallet;
        // let gpass_token = &mut ctx.accounts.gpass_token;
        // let gpass_mint_auth = &ctx.accounts.gpass_mint_auth;
        // let token_program = &ctx.accounts.token_program;
        // let clock = Clock::get()?;

        // // Pay current GPASS earned by user
        // let gpass_earned = utils::calc_earned_gpass(&clock, user_info.last_getting_gpass)?;
        // msg!("Earned GPASS: {}", gpass_earned);
        // if gpass_earned > 0 {
        //     user_info.last_getting_gpass = clock.unix_timestamp;
        //     // Mint GPASS tokens to user
        //     let seeds = &[
        //         ctx.program_id.as_ref(),
        //         freezing_params.to_account_info().key.as_ref(),
        //         &[freezing_params.gpass_mint_auth_bump],
        //     ];
        //     let signer = &[&seeds[..]];
        //     anchor_spl::token::mint_to(
        //         CpiContext::new_with_signer(
        //             token_program.to_account_info(),
        //             MintTo {
        //                 mint: gpass_token.to_account_info(),
        //                 authority: gpass_mint_auth.to_account_info(),
        //                 to: user_gpass_wallet.to_account_info(),
        //             },
        //             signer,
        //         ),
        //         gpass_earned,
        //     )?;
        // }

        // // Unfreeze with conditions
        // if utils::is_withdraw_royalty(&clock, user_info.freezed_time)? {
        //     // TODO: get 15% royalty
        // }

        Ok(())
    }

    // TODO: viewers
}
