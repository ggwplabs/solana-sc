use crate::error::FreezingError;
use crate::state::{RewardTableRow, GPASS_MINT_AUTH_SEED, TREASURY_AUTH_SEED};
use anchor_lang::prelude::*;
use anchor_spl::token::Transfer;
use context::*;

mod context;
mod error;
pub mod state;
mod utils;

declare_id!("ABHUowgjyTkmbMRRuMYJ5ui4wAz6Z6HE4PQMHy9YqMrQ");

#[program]
pub mod freezing {
    use super::*;

    /// Initialize new freezing info with tokens PKs, and parameters.
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

        let freezing_info = &mut ctx.accounts.freezing_info;
        freezing_info.admin = ctx.accounts.admin.key();
        freezing_info.update_auth = update_auth;

        freezing_info.ggwp_token = ctx.accounts.ggwp_token.key();
        freezing_info.gpass_info = ctx.accounts.gpass_info.key();
        freezing_info.gpass_mint_auth_bump = ctx.bumps["gpass_mint_auth"];

        freezing_info.accumulative_fund = ctx.accounts.accumulative_fund.key();
        freezing_info.treasury = ctx.accounts.treasury.key();
        freezing_info.treasury_auth_bump = ctx.bumps["treasury_auth"];

        freezing_info.total_freezed = 0;
        freezing_info.reward_period = reward_period;
        freezing_info.royalty = royalty;
        freezing_info.unfreeze_royalty = unfreeze_royalty;
        freezing_info.unfreeze_lock_period = unfreeze_lock_period;
        freezing_info.reward_table = reward_table;

        Ok(())
    }

    /// Current admin can set another admin.
    pub fn update_admin(ctx: Context<UpdateParam>, admin: Pubkey) -> Result<()> {
        let freezing_info = &mut ctx.accounts.freezing_info;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            freezing_info.admin,
            FreezingError::AccessDenied
        );

        freezing_info.admin = admin;

        Ok(())
    }

    /// Admin can set the new update authority.
    pub fn set_update_authority(ctx: Context<UpdateParam>, update_auth: Pubkey) -> Result<()> {
        let freezing_info = &mut ctx.accounts.freezing_info;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            freezing_info.admin,
            FreezingError::AccessDenied
        );

        freezing_info.update_auth = update_auth;

        Ok(())
    }

    /// Update authority can set the new royalty percent value.
    pub fn update_royalty(ctx: Context<UpdateParam>, royalty: u8) -> Result<()> {
        require!(royalty <= 100, FreezingError::InvalidRoyaltyValue);

        let freezing_info = &mut ctx.accounts.freezing_info;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            freezing_info.update_auth,
            FreezingError::AccessDenied
        );

        freezing_info.royalty = royalty;

        Ok(())
    }

    /// Update authority can set the new unfreeze royalty percent value.
    pub fn update_unfreeze_royalty(ctx: Context<UpdateParam>, unfreeze_royalty: u8) -> Result<()> {
        require!(
            unfreeze_royalty <= 100,
            FreezingError::InvalidUnfreezeRoyaltyValue
        );

        let freezing_info = &mut ctx.accounts.freezing_info;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            freezing_info.update_auth,
            FreezingError::AccessDenied
        );

        freezing_info.unfreeze_royalty = unfreeze_royalty;

        Ok(())
    }

    /// Update authority can set the new reward table.
    pub fn update_reward_table(
        ctx: Context<UpdateParam>,
        reward_table: Vec<RewardTableRow>,
    ) -> Result<()> {
        require!(
            utils::is_reward_table_valid(&reward_table)?,
            FreezingError::InvalidRewardTable,
        );

        let freezing_info = &mut ctx.accounts.freezing_info;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            freezing_info.update_auth,
            FreezingError::AccessDenied
        );

        freezing_info.reward_table = reward_table;

        Ok(())
    }

    /// Update authority can set the new reward period value.
    pub fn update_reward_period(ctx: Context<UpdateParam>, reward_period: i64) -> Result<()> {
        require!(reward_period != 0, FreezingError::InvalidRewardPeriod,);

        let freezing_info = &mut ctx.accounts.freezing_info;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            freezing_info.update_auth,
            FreezingError::AccessDenied
        );

        freezing_info.reward_period = reward_period;

        Ok(())
    }

    /// Update authority can set the new unfreeze lock period value in seconds.
    pub fn update_unfreeze_lock_period(
        ctx: Context<UpdateParam>,
        unfreeze_lock_period: i64,
    ) -> Result<()> {
        require!(
            unfreeze_lock_period != 0,
            FreezingError::InvalidUnfreezeLockPeriod
        );

        let freezing_info = &mut ctx.accounts.freezing_info;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            freezing_info.update_auth,
            FreezingError::AccessDenied
        );

        freezing_info.unfreeze_lock_period = unfreeze_lock_period;

        Ok(())
    }

    /// User freezes his amount of GGWP token to get the GPASS.
    pub fn freeze(ctx: Context<Freeze>, amount: u64) -> Result<()> {
        let user = &ctx.accounts.user;
        let freezing_info = &mut ctx.accounts.freezing_info;
        let treasury = &ctx.accounts.treasury;
        let accumulative_fund = &ctx.accounts.accumulative_fund;
        let user_info = &mut ctx.accounts.user_info;
        let user_ggwp_wallet = &ctx.accounts.user_ggwp_wallet;
        let user_gpass_wallet = &ctx.accounts.user_gpass_wallet;
        let gpass_info = &ctx.accounts.gpass_info;
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
        let royalty_amount = utils::calc_royalty_amount(freezing_info.royalty, amount)?;
        let freezed_amount = amount
            .checked_sub(royalty_amount)
            .ok_or(FreezingError::Overflow)?;

        // Pay amount of GPASS earned by user immediately
        let gpass_earned =
            utils::earned_gpass_immediately(&freezing_info.reward_table, freezed_amount)?;
        msg!("Earned GPASS immediately: {}", gpass_earned);
        if gpass_earned > 0 {
            user_info.last_getting_gpass = clock.unix_timestamp;
            // Mint GPASS tokens to user
            let seeds = &[
                GPASS_MINT_AUTH_SEED.as_bytes(),
                freezing_info.to_account_info().key.as_ref(),
                gpass_info.to_account_info().key.as_ref(),
                &[freezing_info.gpass_mint_auth_bump],
            ];
            let signer = &[&seeds[..]];
            gpass::cpi::mint_to(
                CpiContext::new_with_signer(
                    gpass_program.to_account_info(),
                    gpass::cpi::accounts::MintTo {
                        authority: gpass_mint_auth.to_account_info(),
                        gpass_info: gpass_info.to_account_info(),
                        to: user_gpass_wallet.to_account_info(),
                    },
                    signer,
                ),
                gpass_earned,
            )?;
        }

        // Transfer royalty amount into accumulative fund
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

        freezing_info.total_freezed = freezing_info
            .total_freezed
            .checked_add(freezed_amount)
            .ok_or(FreezingError::Overflow)?;
        user_info.freezed_amount = freezed_amount;
        user_info.freezed_time = clock.unix_timestamp;

        Ok(())
    }

    /// In every time user can withdraw GPASS earned.
    pub fn withdraw_gpass(ctx: Context<Withdraw>) -> Result<()> {
        let user_info = &mut ctx.accounts.user_info;
        let freezing_info = &ctx.accounts.freezing_info;
        let gpass_info = &ctx.accounts.gpass_info;
        let user_gpass_wallet = &ctx.accounts.user_gpass_wallet;
        let gpass_mint_auth = &ctx.accounts.gpass_mint_auth;
        let gpass_program = &ctx.accounts.gpass_program;
        let clock = Clock::get()?;

        let current_time = clock.unix_timestamp;
        // Pay current GPASS earned by user
        let gpass_earned = utils::calc_earned_gpass(
            &freezing_info.reward_table,
            user_info.freezed_amount,
            current_time,
            user_info.last_getting_gpass,
            freezing_info.reward_period,
        )?;
        if gpass_earned == 0 {
            msg!("GPASS is not earned yet");
            return Err(FreezingError::ZeroGpassEarned.into());
        }

        msg!("Earned GPASS: {}", gpass_earned);
        user_info.last_getting_gpass = clock.unix_timestamp;
        // Mint GPASS to user
        let seeds = &[
            GPASS_MINT_AUTH_SEED.as_bytes(),
            freezing_info.to_account_info().key.as_ref(),
            gpass_info.to_account_info().key.as_ref(),
            &[freezing_info.gpass_mint_auth_bump],
        ];
        let signer = &[&seeds[..]];
        gpass::cpi::mint_to(
            CpiContext::new_with_signer(
                gpass_program.to_account_info(),
                gpass::cpi::accounts::MintTo {
                    authority: gpass_mint_auth.to_account_info(),
                    gpass_info: gpass_info.to_account_info(),
                    to: user_gpass_wallet.to_account_info(),
                },
                signer,
            ),
            gpass_earned,
        )?;

        Ok(())
    }

    // User unfreeze full amount of GGWP token.
    pub fn unfreeze(ctx: Context<Unfreeze>) -> Result<()> {
        let freezing_info = &mut ctx.accounts.freezing_info;
        let user_info = &mut ctx.accounts.user_info;
        let user_gpass_wallet = &ctx.accounts.user_gpass_wallet;
        let user_ggwp_wallet = &ctx.accounts.user_ggwp_wallet;
        let gpass_info = &ctx.accounts.gpass_info;
        let gpass_mint_auth = &ctx.accounts.gpass_mint_auth;
        let treasury = &ctx.accounts.treasury;
        let treasury_auth = &ctx.accounts.treasury_auth;
        let accumulative_fund = &ctx.accounts.accumulative_fund;
        let token_program = &ctx.accounts.token_program;
        let gpass_program = &ctx.accounts.gpass_program;
        let clock = Clock::get()?;

        require!(
            user_info.freezed_amount != 0,
            FreezingError::ZeroUnfreezingAmount
        );

        // Pay current GPASS earned by user
        let current_time = clock.unix_timestamp;
        let gpass_earned = utils::calc_earned_gpass(
            &freezing_info.reward_table,
            user_info.freezed_amount,
            current_time,
            user_info.last_getting_gpass,
            freezing_info.reward_period,
        )?;
        msg!("Earned GPASS: {}", gpass_earned);
        if gpass_earned > 0 {
            user_info.last_getting_gpass = clock.unix_timestamp;
            // Mint GPASS tokens to user
            let seeds = &[
                GPASS_MINT_AUTH_SEED.as_bytes(),
                freezing_info.to_account_info().key.as_ref(),
                gpass_info.to_account_info().key.as_ref(),
                &[freezing_info.gpass_mint_auth_bump],
            ];
            let signer = &[&seeds[..]];
            gpass::cpi::mint_to(
                CpiContext::new_with_signer(
                    gpass_program.to_account_info(),
                    gpass::cpi::accounts::MintTo {
                        authority: gpass_mint_auth.to_account_info(),
                        gpass_info: gpass_info.to_account_info(),
                        to: user_gpass_wallet.to_account_info(),
                    },
                    signer,
                ),
                gpass_earned,
            )?;
        }

        let mut amount = user_info.freezed_amount;
        freezing_info.total_freezed = freezing_info
            .total_freezed
            .checked_sub(amount)
            .ok_or(FreezingError::Overflow)?;

        // Send royalty to accumulative fund
        let seeds = &[
            TREASURY_AUTH_SEED.as_bytes(),
            freezing_info.to_account_info().key.as_ref(),
            &[freezing_info.treasury_auth_bump],
        ];
        let treasury_auth_signer = &[&seeds[..]];

        if utils::is_withdraw_royalty(
            current_time,
            user_info.freezed_time,
            freezing_info.unfreeze_lock_period,
        )? {
            let royalty_amount = utils::calc_royalty_amount(
                freezing_info.unfreeze_royalty,
                user_info.freezed_amount,
            )?;
            msg!("Unfreeze royalty: {}", royalty_amount);

            anchor_spl::token::transfer(
                CpiContext::new_with_signer(
                    token_program.to_account_info(),
                    Transfer {
                        from: treasury.to_account_info(),
                        to: accumulative_fund.to_account_info(),
                        authority: treasury_auth.to_account_info(),
                    },
                    treasury_auth_signer,
                ),
                royalty_amount,
            )?;

            amount = amount
                .checked_sub(royalty_amount)
                .ok_or(FreezingError::Overflow)?;
        }

        // Send GGWP to user wallet
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                Transfer {
                    from: treasury.to_account_info(),
                    to: user_ggwp_wallet.to_account_info(),
                    authority: treasury_auth.to_account_info(),
                },
                treasury_auth_signer,
            ),
            amount,
        )?;

        user_info.freezed_amount = 0;
        user_info.freezed_time = 0;

        Ok(())
    }
}
