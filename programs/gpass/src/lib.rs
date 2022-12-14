use crate::context::*;
use crate::error::GpassError;
use crate::state::{MAX_BURNERS, MAX_MINTERS};
use anchor_lang::prelude::*;

pub mod context;
mod error;
pub mod state;
mod utils;

declare_id!("Gv9WAng6iPymaDwXMQrbsh2uTkDpAPTB89Ld4ctJejMG");

#[program]
pub mod gpass {
    use super::*;

    /// First time initialization of contract parameters.
    /// burn_period - period in seconds.
    /// update_auth - authority for update instructions.
    /// minters - list of minters.
    /// burners - list of burners.
    pub fn initialize(
        ctx: Context<Initialize>,
        burn_period: u64,
        update_auth: Pubkey,
        minters: Vec<Pubkey>,
        burners: Vec<Pubkey>,
    ) -> Result<()> {
        require!(
            minters.len() <= MAX_MINTERS,
            GpassError::MaxMintersSizeExceeded
        );
        require!(
            burners.len() <= MAX_BURNERS,
            GpassError::MaxBurnersSizeExceeded
        );
        require_neq!(burn_period, 0, GpassError::InvalidBurnPeriodValue);

        let gpass_info = &mut ctx.accounts.gpass_info;
        gpass_info.admin = ctx.accounts.admin.key();
        gpass_info.update_auth = update_auth;
        gpass_info.burn_period = burn_period;
        gpass_info.total_amount = 0;
        gpass_info.minters = minters;
        gpass_info.burners = burners;

        Ok(())
    }

    /// Current admin can set the new admin.
    pub fn update_admin(ctx: Context<UpdateParam>, admin: Pubkey) -> Result<()> {
        let authority = &ctx.accounts.authority;
        let gpass_info = &mut ctx.accounts.gpass_info;

        require_keys_eq!(authority.key(), gpass_info.admin, GpassError::AccessDenied);
        gpass_info.admin = admin;

        Ok(())
    }

    /// Admin cat set the new update authority
    pub fn set_update_authority(ctx: Context<UpdateParam>, update_auth: Pubkey) -> Result<()> {
        let authority = &ctx.accounts.authority;
        let gpass_info = &mut ctx.accounts.gpass_info;

        require_keys_eq!(authority.key(), gpass_info.admin, GpassError::AccessDenied);
        gpass_info.update_auth = update_auth;

        Ok(())
    }

    /// Update authority can set the new burn period value.
    pub fn update_burn_period(ctx: Context<UpdateParam>, burn_period: u64) -> Result<()> {
        let authority = &ctx.accounts.authority;
        let gpass_info = &mut ctx.accounts.gpass_info;

        require_keys_eq!(
            authority.key(),
            gpass_info.update_auth,
            GpassError::AccessDenied
        );
        require_neq!(burn_period, 0, GpassError::InvalidBurnPeriodValue);

        gpass_info.burn_period = burn_period;

        Ok(())
    }

    /// Update authority can set the new minters list.
    pub fn update_minters(ctx: Context<UpdateParam>, minters: Vec<Pubkey>) -> Result<()> {
        let authority = &ctx.accounts.authority;
        let gpass_info = &mut ctx.accounts.gpass_info;

        require_keys_eq!(
            authority.key(),
            gpass_info.update_auth,
            GpassError::AccessDenied
        );
        require!(
            minters.len() <= MAX_MINTERS,
            GpassError::MaxMintersSizeExceeded
        );

        gpass_info.minters = minters;

        Ok(())
    }

    /// Update authority can set the new burner list.
    pub fn update_burners(ctx: Context<UpdateParam>, burners: Vec<Pubkey>) -> Result<()> {
        let authority = &ctx.accounts.authority;
        let gpass_info = &mut ctx.accounts.gpass_info;

        require_keys_eq!(
            authority.key(),
            gpass_info.update_auth,
            GpassError::AccessDenied
        );
        require!(
            burners.len() <= MAX_BURNERS,
            GpassError::MaxBurnersSizeExceeded
        );

        gpass_info.burners = burners;

        Ok(())
    }

    /// Creating the new wallet for user by payer (can be same).
    pub fn create_wallet(ctx: Context<CreateWallet>) -> Result<()> {
        let clock = Clock::get()?;
        let wallet = &mut ctx.accounts.wallet;

        wallet.amount = 0;
        wallet.last_burned = clock.unix_timestamp;

        msg!(
            "Wallet {} created. Last burned: {}",
            wallet.key(),
            wallet.last_burned
        );

        Ok(())
    }

    /// Mint the amount of GPASS to user wallet. Available only for minters.
    /// There is trying to burn overdues before minting.
    pub fn mint_to(ctx: Context<MintTo>, amount: u64) -> Result<()> {
        let gpass_info = &mut ctx.accounts.gpass_info;
        let authority = &ctx.accounts.authority;
        let to = &mut ctx.accounts.to;
        let clock = Clock::get()?;

        require_neq!(amount, 0, GpassError::ZeroMintAmount);
        if !gpass_info.minters.contains(authority.key) {
            return Err(GpassError::InvalidMintAuthority.into());
        }

        // Try to burn amount before mint
        let time_passed = utils::time_passed(clock.unix_timestamp, to.last_burned)?;
        if time_passed < gpass_info.burn_period {
            msg!("Burn period not yet passed, GPASS not burned");
        } else {
            msg!("Burn period passed, {} of GPASS burned", to.amount);
            gpass_info.total_amount = gpass_info
                .total_amount
                .checked_sub(to.amount)
                .ok_or(GpassError::Overflow)?;
            to.amount = 0;
            to.last_burned = clock.unix_timestamp;
        }

        msg!("Mint {} gpass to wallet {}", amount, to.key());
        to.amount = to.amount.checked_add(amount).ok_or(GpassError::Overflow)?;
        gpass_info.total_amount = gpass_info
            .total_amount
            .checked_add(amount)
            .ok_or(GpassError::Overflow)?;

        Ok(())
    }

    /// Burn the amount of GPASS from user wallet. Available only for burners.
    /// There is trying to burn overdues before burning.
    pub fn burn(ctx: Context<Burn>, amount: u64) -> Result<()> {
        let gpass_info = &mut ctx.accounts.gpass_info;
        let authority = &ctx.accounts.authority;
        let from = &mut ctx.accounts.from;
        let clock = Clock::get()?;

        require_neq!(amount, 0, GpassError::ZeroBurnAmount);
        if !gpass_info.burners.contains(authority.key) {
            return Err(GpassError::InvalidBurnAuthority.into());
        }

        // Try to burn amount before mint
        let time_passed = utils::time_passed(clock.unix_timestamp, from.last_burned)?;
        if time_passed < gpass_info.burn_period {
            msg!("Burn period not yet passed, GPASS not burned");
        } else {
            msg!("Burn period passed, {} of GPASS burned", from.amount);
            gpass_info.total_amount = gpass_info
                .total_amount
                .checked_sub(from.amount)
                .ok_or(GpassError::Overflow)?;
            from.amount = 0;
            from.last_burned = clock.unix_timestamp;
        }

        if from.amount == 0 {
            msg!("Wallet empty");
        } else {
            msg!("Burn {} gpass from wallet {}", amount, from.key());
            from.amount = from.amount.checked_sub(amount).unwrap_or(0);
            gpass_info.total_amount = gpass_info
                .total_amount
                .checked_sub(amount)
                .ok_or(GpassError::Overflow)?;
        }

        Ok(())
    }

    /// Everyone in any time can synchronize user GPASS balance and burn overdues.
    pub fn try_burn_in_period(ctx: Context<BurnInPeriod>) -> Result<()> {
        let gpass_info = &mut ctx.accounts.gpass_info;
        let wallet = &mut ctx.accounts.wallet;
        let clock = Clock::get()?;

        let time_passed = utils::time_passed(clock.unix_timestamp, wallet.last_burned)?;
        if time_passed < gpass_info.burn_period {
            msg!("Burn period not yet passed, GPASS not burned");
            return Err(GpassError::PeriodNotPassed.into());
        } else {
            msg!("Burn period passed, {} of GPASS burned", wallet.amount);
            gpass_info.total_amount = gpass_info
                .total_amount
                .checked_sub(wallet.amount)
                .ok_or(GpassError::Overflow)?;
            wallet.amount = 0;
            wallet.last_burned = clock.unix_timestamp;
        }

        Ok(())
    }
}
