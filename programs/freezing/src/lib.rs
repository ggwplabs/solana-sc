use crate::error::FreezingError;
use crate::state::{RewardTableRow, GPASS_MINT_AUTH_SEED};
use anchor_lang::prelude::*;
use anchor_spl::token::Transfer;
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
        reward_period: i64,
        royalty: u8,
        unfreeze_royalty: u8,
        unfreeze_lock_period: i64,
        reward_table: Vec<RewardTableRow>,
    ) -> Result<()> {
        require!(royalty <= 100, FreezingError::InvalidRoyaltyValue);
        require!(
            unfreeze_royalty <= 100,
            FreezingError::InvalidUnfreezeRoyaltyValue
        );
        require!(
            unfreeze_lock_period != 0,
            FreezingError::InvalidUnfreezeLockPeriod
        );
        require!(
            utils::is_reward_table_valid(&reward_table)?,
            FreezingError::InvalidRewardTable,
        );
        require!(reward_period != 0, FreezingError::InvalidRewardPeriod);

        let freezing_params = &mut ctx.accounts.freezing_params;
        freezing_params.admin = ctx.accounts.admin.key();
        freezing_params.update_auth = update_auth;

        freezing_params.ggwp_token = ctx.accounts.ggwp_token.key();
        freezing_params.gpass_settings = ctx.accounts.gpass_settings.key();
        freezing_params.gpass_mint_auth_bump = ctx.bumps["gpass_mint_auth"];

        freezing_params.accumulative_fund = ctx.accounts.accumulative_fund.key();
        freezing_params.treasury = ctx.accounts.treasury.key();

        freezing_params.total_freezed = 0;
        freezing_params.reward_period = reward_period;
        freezing_params.royalty = royalty;
        freezing_params.unfreeze_royalty = unfreeze_royalty;
        freezing_params.unfreeze_lock_period = unfreeze_lock_period;
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

    /// Update authority can set the new royalty percent value.
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

    /// Update authority can set the new unfreeze royalty percent value.
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

    /// Update authority can set the new reward table.
    pub fn update_reward_table(
        ctx: Context<UpdateParams>,
        reward_table: Vec<RewardTableRow>,
    ) -> Result<()> {
        require!(
            utils::is_reward_table_valid(&reward_table)?,
            FreezingError::InvalidRewardTable,
        );

        let freezing_params = &mut ctx.accounts.freezing_params;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            freezing_params.update_auth,
            FreezingError::AccessDenied
        );

        freezing_params.reward_table = reward_table;

        Ok(())
    }

    /// Update authority can set the new reward period value.
    pub fn update_reward_period(ctx: Context<UpdateParams>, reward_period: i64) -> Result<()> {
        require!(reward_period != 0, FreezingError::InvalidRewardPeriod,);

        let freezing_params = &mut ctx.accounts.freezing_params;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            freezing_params.update_auth,
            FreezingError::AccessDenied
        );

        freezing_params.reward_period = reward_period;

        Ok(())
    }

    /// Update authority can set the new unfreeze lock period value in seconds.
    pub fn update_unfreeze_lock_period(
        ctx: Context<UpdateParams>,
        unfreeze_lock_period: i64,
    ) -> Result<()> {
        require!(
            unfreeze_lock_period != 0,
            FreezingError::InvalidUnfreezeLockPeriod
        );

        let freezing_params = &mut ctx.accounts.freezing_params;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            freezing_params.update_auth,
            FreezingError::AccessDenied
        );

        freezing_params.unfreeze_lock_period = unfreeze_lock_period;

        Ok(())
    }

    /// User freezes his amount of GGWP token to get the GPASS.
    pub fn freeze(ctx: Context<Freeze>, amount: u64) -> Result<()> {
        let user = &ctx.accounts.user;
        let freezing_params = &mut ctx.accounts.freezing_params;
        let treasury = &mut ctx.accounts.treasury;
        let accumulative_fund = &mut ctx.accounts.accumulative_fund;
        let user_info = &mut ctx.accounts.user_info;
        let user_ggwp_wallet = &mut ctx.accounts.user_ggwp_wallet;
        let user_gpass_wallet = &mut ctx.accounts.user_gpass_wallet;
        let gpass_settings = &mut ctx.accounts.gpass_settings;
        let gpass_mint_auth = &ctx.accounts.gpass_mint_auth;
        let gpass_program = &ctx.accounts.gpass_program;
        let token_program = &ctx.accounts.token_program;
        let clock = Clock::get()?;

        require_neq!(amount, 0, FreezingError::ZeroFreezingAmount);
        if user_info.freezed_amount != 0 {
            msg!("Additional freezing is not available.");
            return Err(FreezingError::AdditionalFreezingNotAvailable.into());
        }

        // Init user info in needed
        if !user_info.is_initialized {
            user_info.is_initialized = true;
            user_info.freezed_amount = 0;
            user_info.freezed_time = 0;
            user_info.last_getting_gpass = clock.unix_timestamp;
        }

        // Calc the royalty
        let royalty_amount = utils::calc_royalty_amount(freezing_params.royalty, amount)?;
        let freezed_amount = amount
            .checked_sub(royalty_amount)
            .ok_or(FreezingError::Overflow)?;

        // Pay amount of GPASS earned by user immediately
        let gpass_earned =
            utils::earned_gpass_immediately(&freezing_params.reward_table, freezed_amount)?;
        msg!("Earned GPASS immediately: {}", gpass_earned);
        if gpass_earned > 0 {
            user_info.last_getting_gpass = clock.unix_timestamp;
            // Mint GPASS tokens to user
            let seeds = &[
                GPASS_MINT_AUTH_SEED.as_bytes(),
                freezing_params.to_account_info().key.as_ref(),
                gpass_settings.to_account_info().key.as_ref(),
                &[freezing_params.gpass_mint_auth_bump],
            ];
            let signer = &[&seeds[..]];
            gpass::cpi::mint_to(
                CpiContext::new_with_signer(
                    gpass_program.to_account_info(),
                    gpass::cpi::accounts::MintTo {
                        authority: gpass_mint_auth.to_account_info(),
                        settings: gpass_settings.to_account_info(),
                        to: user_gpass_wallet.to_account_info(),
                    },
                    signer,
                ),
                gpass_earned,
            )?;
        }

        // Transfer royalty amount into
        anchor_spl::token::transfer(
            CpiContext::new(
                token_program.to_account_info(),
                Transfer {
                    from: user_ggwp_wallet.to_account_info(),
                    to: accumulative_fund.to_account_info(),
                    authority: user.to_account_info(),
                },
            ),
            royalty_amount,
        )?;

        // Freeze GGWP, transfer to treasury
        anchor_spl::token::transfer(
            CpiContext::new(
                token_program.to_account_info(),
                Transfer {
                    from: user_ggwp_wallet.to_account_info(),
                    to: treasury.to_account_info(),
                    authority: user.to_account_info(),
                },
            ),
            freezed_amount,
        )?;

        freezing_params.total_freezed = freezing_params
            .total_freezed
            .checked_add(freezed_amount)
            .ok_or(FreezingError::Overflow)?;
        user_info.freezed_amount = freezed_amount;
        user_info.freezed_time = clock.unix_timestamp;

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
