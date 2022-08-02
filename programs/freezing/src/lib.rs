use anchor_lang::prelude::*;
use anchor_spl::token::{set_authority, SetAuthority};
use context::*;

mod context;
mod error;
mod state;

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
        set_authority(
            cpi_ctx,
            spl_token::instruction::AuthorityType::MintTokens,
            Some(ctx.accounts.gpass_mint_auth.key()),
        )?;

        freezing_params.admin = ctx.accounts.admin.key();
        freezing_params.ggwp_token = ctx.accounts.ggwp_token.key();
        freezing_params.gpass_token = ctx.accounts.gpass_token.key();
        freezing_params.accumulative_fund = ctx.accounts.accumulative_fund.key();
        freezing_params.mint_auth_bump = ctx.bumps["gpass_mint_auth"];

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
        // TODO
        // pay current gpass earned
        // freeze additional ggwp
        Ok(())
    }

    /// In every time user can withdraw GPASS earned.
    pub fn withdraw_gpass(ctx: Context<Withdraw>) -> Result<()> {
        // TODO
        // pay current gpass earned
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
