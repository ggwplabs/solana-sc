use crate::context::*;
use crate::error::FightingError;
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
        afk_timeout_sec: i64,
    ) -> Result<()> {
        require!(afk_timeout_sec > 0, FightingError::InvalidAFKTimeout);

        let fighting_settings = &mut ctx.accounts.fighting_settings;
        fighting_settings.admin = ctx.accounts.admin.key();
        fighting_settings.update_auth = update_auth;
        fighting_settings.afk_timeout_sec = afk_timeout_sec;

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
    pub fn update_afk_timeout_set(ctx: Context<UpdateSetting>, afk_timeout_sec: i64) -> Result<()> {
        let fighting_settings = &mut ctx.accounts.fighting_settings;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            fighting_settings.update_auth,
            FightingError::AccessDenied
        );

        fighting_settings.afk_timeout_sec = afk_timeout_sec;

        Ok(())
    }

    /// User starts new game session and pays GPASS for it.
    pub fn start_game(ctx: Context<StartGame>) -> Result<()> {
        Ok(())
    }

    /// User finalize the game. Fee payer: GGWP system account.
    pub fn finalize_game(ctx: Context<FinalizeGame>) -> Result<()> {
        Ok(())
    }
}
