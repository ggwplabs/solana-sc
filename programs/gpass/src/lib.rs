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

    /// First time initialization of contract parameters.
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

    /// Creating the new wallet for user by payer (can be same).
    pub fn create_wallet(ctx: Context<CreateWallet>) -> Result<()> {
        let clock = Clock::get()?;
        let wallet = &mut ctx.accounts.wallet;

        wallet.amount = 0;
        wallet.last_burned = clock.unix_timestamp;

        msg!(
            "Wallet {} created. Last reset: {}",
            wallet.key(),
            wallet.last_burned
        );

        Ok(())
    }

    /// Mint the amount of GPASS to user. Available only for minters.
    /// There is trying to burn overdues before minting.
    pub fn mint_to(ctx: Context<MintTo>, amount: u64) -> Result<()> {
        let settings = &ctx.accounts.settings;
        let authority = &ctx.accounts.authority;
        let to = &mut ctx.accounts.to;
        let clock = Clock::get()?;

        require_neq!(amount, 0, GpassError::ZeroMintAmount);
        if !settings.minters.contains(authority.key) {
            return Err(GpassError::InvalidMintAuthority.into());
        }

        // Try to burn amount before mint
        let time_passed = clock
            .unix_timestamp
            .checked_sub(to.last_burned)
            .ok_or(GpassError::Overflow)?;

        if time_passed < settings.burn_period {
            msg!("Burn period not yet passed, GPASS not burned");
        } else {
            msg!("Burn period passed, {} of GPASS burned", to.amount);
            to.amount = 0;
            to.last_burned = clock.unix_timestamp;
        }

        msg!("Mint {} gpass to wallet {}", amount, to.key());
        to.amount = to.amount.checked_add(amount).ok_or(GpassError::Overflow)?;

        Ok(())
    }

    /// Everyone in any time can synchronize user GPASS balance and burn overdues.
    pub fn try_burn(ctx: Context<Burn>) -> Result<()> {
        let settings = &ctx.accounts.settings;
        let wallet = &mut ctx.accounts.wallet;
        let clock = Clock::get()?;

        let time_passed = clock
            .unix_timestamp
            .checked_sub(wallet.last_burned)
            .ok_or(GpassError::Overflow)?;

        if time_passed < settings.burn_period {
            msg!("Burn period not yet passed, GPASS not burned");
        } else {
            msg!("Burn period passed, {} of GPASS burned", wallet.amount);
            wallet.amount = 0;
            wallet.last_burned = clock.unix_timestamp;
        }

        Ok(())
    }

    // TODO: viewers
}
