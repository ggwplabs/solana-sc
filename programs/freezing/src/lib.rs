use crate::error::FreezingError;
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

    /// Initialize new freezing params with tokens PKs.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let freezing_params = &mut ctx.accounts.freezing_params;

        // Change GPASS mint authority to PDA
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            SetAuthority {
                account_or_mint: ctx.accounts.gpass_token.to_account_info(),
                current_authority: ctx.accounts.admin.to_account_info(),
            },
        );
        anchor_spl::token::set_authority(
            cpi_ctx,
            spl_token::instruction::AuthorityType::MintTokens,
            Some(ctx.accounts.gpass_mint_auth.key()),
        )?;

        freezing_params.admin = ctx.accounts.admin.key();
        freezing_params.ggwp_token = ctx.accounts.ggwp_token.key();
        freezing_params.gpass_token = ctx.accounts.gpass_token.key();
        freezing_params.accumulative_fund = ctx.accounts.accumulative_fund.key();
        freezing_params.treasury = ctx.accounts.treasury.key();
        freezing_params.gpass_mint_auth_bump = ctx.bumps["gpass_mint_auth"];

        Ok(())
    }

    /// Current admin can set another admin.
    pub fn change_admin(ctx: Context<ChangeAdmin>) -> Result<()> {
        let freezing_params = &mut ctx.accounts.freezing_params;
        freezing_params.admin = ctx.accounts.new_admin.key();
        Ok(())
    }

    pub fn change_params(ctx: Context<ChangeParams>) -> Result<()> {
        // TODO
        Ok(())
    }

    /// User freezes his amount of GGWP token to get the GPASS tokens.
    pub fn freeze(ctx: Context<Freeze>, amount: u64) -> Result<()> {
        let user = &ctx.accounts.user;
        let freezing_params = &ctx.accounts.freezing_params;
        let treasury = &mut ctx.accounts.treasury;
        let user_info = &mut ctx.accounts.user_info;
        let user_ggwp_wallet = &mut ctx.accounts.user_ggwp_wallet;
        let user_gpass_wallet = &mut ctx.accounts.user_gpass_wallet;
        let gpass_token = &mut ctx.accounts.gpass_token;
        let gpass_mint_auth = &ctx.accounts.gpass_mint_auth;
        let token_program = &ctx.accounts.token_program;
        let clock = Clock::get()?;

        // Init user info in needed
        if !user_info.is_initialized {
            user_info.is_initialized = true;
            user_info.freezed_amount = 0;
            user_info.last_getting_gpass = clock.unix_timestamp;
        }

        // Pay current GPASS earned by user
        let gpass_earned = utils::calc_earned_gpass(&clock, user_info.last_getting_gpass)?;
        msg!("Earned GPASS: {}", gpass_earned);
        if gpass_earned > 0 {
            user_info.last_getting_gpass = clock.unix_timestamp;
            // Mint GPASS tokens to user
            let seeds = &[
                ctx.program_id.as_ref(),
                freezing_params.to_account_info().key.as_ref(),
                &[freezing_params.gpass_mint_auth_bump],
            ];
            let signer = &[&seeds[..]];
            anchor_spl::token::mint_to(
                CpiContext::new_with_signer(
                    token_program.to_account_info(),
                    MintTo {
                        mint: gpass_token.to_account_info(),
                        authority: gpass_mint_auth.to_account_info(),
                        to: user_gpass_wallet.to_account_info(),
                    },
                    signer,
                ),
                gpass_earned,
            )?;
        }

        // Freeze additional GGWP
        anchor_spl::token::transfer(
            CpiContext::new(
                token_program.to_account_info(),
                Transfer {
                    from: user_ggwp_wallet.to_account_info(),
                    to: treasury.to_account_info(),
                    authority: user.to_account_info(),
                },
            ),
            amount,
        )?;

        user_info
            .freezed_amount
            .checked_add(amount)
            .ok_or(FreezingError::Overflow)?;

        Ok(())
    }

    /// In every time user can withdraw GPASS earned.
    pub fn withdraw_gpass(ctx: Context<Withdraw>) -> Result<()> {
        let freezing_params = &ctx.accounts.freezing_params;
        let user_info = &mut ctx.accounts.user_info;
        let user_gpass_wallet = &mut ctx.accounts.user_gpass_wallet;
        let gpass_token = &mut ctx.accounts.gpass_token;
        let gpass_mint_auth = &ctx.accounts.gpass_mint_auth;
        let token_program = &ctx.accounts.token_program;
        let clock = Clock::get()?;

        // Pay current GPASS earned by user
        let gpass_earned = utils::calc_earned_gpass(&clock, user_info.last_getting_gpass)?;
        // TODO: if gpass_earned == 0 -> error?
        msg!("Earned GPASS: {}", gpass_earned);
        if gpass_earned > 0 {
            user_info.last_getting_gpass = clock.unix_timestamp;
            // Mint GPASS tokens to user
            let seeds = &[
                ctx.program_id.as_ref(),
                freezing_params.to_account_info().key.as_ref(),
                &[freezing_params.gpass_mint_auth_bump],
            ];
            let signer = &[&seeds[..]];
            anchor_spl::token::mint_to(
                CpiContext::new_with_signer(
                    token_program.to_account_info(),
                    MintTo {
                        mint: gpass_token.to_account_info(),
                        authority: gpass_mint_auth.to_account_info(),
                        to: user_gpass_wallet.to_account_info(),
                    },
                    signer,
                ),
                gpass_earned,
            )?;
        }

        Ok(())
    }

    // User unfreeze his amount of GGWP token.
    pub fn unfreeze(ctx: Context<Unfreeze>, amount: u64) -> Result<()> {
        // TODO
        // pay current gpass earned
        // unfreeze with conditions
        Ok(())
    }

    // TODO: viewers
}
