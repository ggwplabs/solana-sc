use crate::context::*;
use crate::error::FightingError;
use crate::state::GPASS_BURN_AUTH_SEED;
use anchor_lang::prelude::*;

mod context;
mod error;
pub mod state;

declare_id!("F23aPzza8PQyFmBwPT7eKv3oabEoBwa5aSFAHwYSfam6");

#[program]
pub mod fighting {
    use super::*;

    /// Set up basic settings for game.
    pub fn initialize(
        ctx: Context<Initialize>,
        update_auth: Pubkey,
        afk_timeout: i64,
    ) -> Result<()> {
        require!(afk_timeout > 0, FightingError::InvalidAFKTimeout);

        let fighting_settings = &mut ctx.accounts.fighting_settings;
        fighting_settings.admin = ctx.accounts.admin.key();
        fighting_settings.update_auth = update_auth;
        fighting_settings.afk_timeout = afk_timeout;
        fighting_settings.gpass_burn_auth_bump = ctx.bumps["gpass_burn_auth"];

        Ok(())
    }

    /// Current admin can set another admin.
    pub fn update_admin(ctx: Context<UpdateSetting>, admin: Pubkey) -> Result<()> {
        let fighting_settings = &mut ctx.accounts.fighting_settings;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            fighting_settings.admin,
            FightingError::AccessDenied
        );

        fighting_settings.admin = admin;

        Ok(())
    }

    // Admin can set the new update authority.
    pub fn set_update_authority(ctx: Context<UpdateSetting>, update_auth: Pubkey) -> Result<()> {
        let fighting_settings = &mut ctx.accounts.fighting_settings;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            fighting_settings.admin,
            FightingError::AccessDenied
        );

        fighting_settings.update_auth = update_auth;

        Ok(())
    }

    /// Update auth can set the new AFK timeout in sec value.
    pub fn update_afk_timeout_set(ctx: Context<UpdateSetting>, afk_timeout: i64) -> Result<()> {
        let fighting_settings = &mut ctx.accounts.fighting_settings;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            fighting_settings.update_auth,
            FightingError::AccessDenied
        );

        fighting_settings.afk_timeout = afk_timeout;

        Ok(())
    }

    /// User starts new game session and pays GPASS for it.
    pub fn start_game(ctx: Context<StartGame>) -> Result<()> {
        let user_info = &mut ctx.accounts.user_info;
        let gpass_info = &ctx.accounts.gpass_info;
        let gpass_burn_auth = &ctx.accounts.gpass_burn_auth;
        let fighting_settings = &ctx.accounts.fighting_settings;
        let user_gpass_wallet = &ctx.accounts.user_gpass_wallet;
        let gpass_program = &ctx.accounts.gpass_program;
        let clock = Clock::get()?;

        if user_info.in_game == true && user_info.in_game_time != 0 {
            let spent_time = clock
                .unix_timestamp
                .checked_sub(user_info.in_game_time)
                .ok_or(FightingError::Overflow)?;
            if spent_time < fighting_settings.afk_timeout {
                msg!("AFK timeout not passed.");
                return Err(FightingError::StillInGame.into());
            }
            else {
                msg!("AFK timeout passed.");
                user_info.in_game = false;
                return Ok(());
            }
        }

        require_neq!(user_gpass_wallet.amount, 0, FightingError::NotEnoughGpass);

        // Burn 1 GPASS from user wallet
        let seeds = &[
            GPASS_BURN_AUTH_SEED.as_bytes(),
            fighting_settings.to_account_info().key.as_ref(),
            gpass_info.to_account_info().key.as_ref(),
            &[fighting_settings.gpass_burn_auth_bump],
        ];
        let signer = &[&seeds[..]];
        gpass::cpi::burn(
            CpiContext::new_with_signer(
                gpass_program.to_account_info(),
                gpass::cpi::accounts::Burn {
                    authority: gpass_burn_auth.to_account_info(),
                    gpass_info: gpass_info.to_account_info(),
                    from: user_gpass_wallet.to_account_info(),
                },
                signer,
            ),
            1,
        )?;

        user_info.in_game = true;
        user_info.in_game_time = clock.unix_timestamp;

        Ok(())
    }

    /// User finalize the game. Fee payer: GGWP system account.
    pub fn finalize_game(ctx: Context<FinalizeGame>) -> Result<()> {
        Ok(())
    }
}
