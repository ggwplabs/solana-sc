use crate::context::*;
use crate::error::GpassError;
use crate::state::MAX_MINTERS;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::UnixTimestamp;

mod context;
mod error;
mod state;

declare_id!("Gv9WAng6iPymaDwXMQrbsh2uTkDpAPTB89Ld4ctJejMG");

#[program]
pub mod gpass {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        burn_period: UnixTimestamp,
        minters: Vec<Pubkey>,
    ) -> Result<()> {
        require!(
            minters.len() <= MAX_MINTERS,
            GpassError::MaxMintersSizeExceeded
        );
        require_neq!(burn_period, 0, GpassError::InvalidBurnPeriodValue);

        let settings = &mut ctx.accounts.settings;
        settings.admin = ctx.accounts.admin.key();
        settings.burn_period = burn_period;
        settings.minters = minters;

        Ok(())
    }

    pub fn create_wallet(ctx: Context<CreateWallet>) -> Result<()> {
        let clock = Clock::get()?;
        let wallet = &mut ctx.accounts.wallet;
        
        wallet.amount = 0;
        wallet.last_reset = clock.unix_timestamp;

        Ok(())
    }
}
