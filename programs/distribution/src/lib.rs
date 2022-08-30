use crate::context::*;
use crate::error::DistributionError;
use crate::state::ACCUMULATIVE_FUND_AUTH_SEED;
use anchor_lang::prelude::*;
use anchor_spl::token::Transfer;

declare_id!("79GShMQgEBcfpiiwkBxv3yBxHqCN8J2E8DhivatqpfYC");

mod context;
mod error;
pub mod state;

#[program]
pub mod distribution {
    use super::*;

    /// Initialize with information about funds and shares.
    pub fn initialize(
        ctx: Context<Initialize>,
        update_auth: Pubkey,
        play_to_earn_fund_share: u8,
        staking_fund_share: u8,
        company_fund_share: u8,
        team_fund_share: u8,
    ) -> Result<()> {
        require!(
            play_to_earn_fund_share <= 100,
            DistributionError::InvalidShare
        );
        require!(staking_fund_share <= 100, DistributionError::InvalidShare);
        require!(company_fund_share <= 100, DistributionError::InvalidShare);
        require!(team_fund_share <= 100, DistributionError::InvalidShare);

        let distribution_info = &mut ctx.accounts.distribution_info;
        distribution_info.admin = ctx.accounts.admin.key();
        distribution_info.update_auth = update_auth;

        distribution_info.ggwp_token = ctx.accounts.ggwp_token.key();
        distribution_info.accumulative_fund = ctx.accounts.accumulative_fund.key();
        distribution_info.accumulative_fund_auth_bump = ctx.bumps["accumulative_fund_auth"];

        distribution_info.last_distribution = 0;
        distribution_info.play_to_earn_fund = ctx.accounts.play_to_earn_fund.key();
        distribution_info.play_to_earn_fund_share = play_to_earn_fund_share;
        distribution_info.staking_fund = ctx.accounts.staking_fund.key();
        distribution_info.staking_fund_share = staking_fund_share;
        distribution_info.company_fund = ctx.accounts.company_fund.key();
        distribution_info.company_fund_share = company_fund_share;
        distribution_info.team_fund = ctx.accounts.team_fund.key();
        distribution_info.team_fund_share = team_fund_share;

        Ok(())
    }

    // TODO: update methods

    /// Anyone can run the distribution of GGWP tokens.
    pub fn distribute(ctx: Context<Distribute>) -> Result<()> {
        let distribution_info = &mut ctx.accounts.distribution_info;
        let accumulative_fund = &ctx.accounts.accumulative_fund;
        let accumulative_fund_auth = &ctx.accounts.accumulative_fund_auth;
        let play_to_earn_fund = &ctx.accounts.play_to_earn_fund;
        let staking_fund = &ctx.accounts.staking_fund;
        let company_fund = &ctx.accounts.company_fund;
        let team_fund = &ctx.accounts.team_fund;
        let token_program = &ctx.accounts.token_program;
        let clock = Clock::get()?;

        require_neq!(
            accumulative_fund.amount,
            0,
            DistributionError::EmptyAccumulativeFund
        );

        let seeds = &[
            ACCUMULATIVE_FUND_AUTH_SEED.as_bytes(),
            distribution_info.to_account_info().key.as_ref(),
            accumulative_fund.to_account_info().key.as_ref(),
            &[distribution_info.accumulative_fund_auth_bump],
        ];
        let accumulative_fund_auth_signer = &[&seeds[..]];

        let amount = accumulative_fund.amount;
        msg!("Accumulative fund amount: {}", amount);

        // Transfer GGWP to play to earn fund
        let play_to_earn_amount =
            calc_share_amount(distribution_info.play_to_earn_fund_share, amount)?;
        msg!("Play to earn fund share: {}", play_to_earn_amount);
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                Transfer {
                    from: accumulative_fund.to_account_info(),
                    to: play_to_earn_fund.to_account_info(),
                    authority: accumulative_fund_auth.to_account_info(),
                },
                accumulative_fund_auth_signer,
            ),
            play_to_earn_amount,
        )?;

        // Transfer GGWP to staking fund
        let staking_fund_amount = calc_share_amount(distribution_info.staking_fund_share, amount)?;
        msg!("Staking fund share: {}", staking_fund_amount);
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                Transfer {
                    from: accumulative_fund.to_account_info(),
                    to: staking_fund.to_account_info(),
                    authority: accumulative_fund_auth.to_account_info(),
                },
                accumulative_fund_auth_signer,
            ),
            staking_fund_amount,
        )?;

        // Transfer GGWP to company fund
        let company_fund_amount = calc_share_amount(distribution_info.company_fund_share, amount)?;
        msg!("Company fund share: {}", company_fund_amount);
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                Transfer {
                    from: accumulative_fund.to_account_info(),
                    to: company_fund.to_account_info(),
                    authority: accumulative_fund_auth.to_account_info(),
                },
                accumulative_fund_auth_signer,
            ),
            company_fund_amount,
        )?;

        // Transfer GGWP to team fund
        let team_amount = amount
            .checked_sub(play_to_earn_amount + staking_fund_amount + company_fund_amount)
            .ok_or(DistributionError::Overflow)?;
        msg!("Team fund share: {}", company_fund_amount);
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                Transfer {
                    from: accumulative_fund.to_account_info(),
                    to: team_fund.to_account_info(),
                    authority: accumulative_fund_auth.to_account_info(),
                },
                accumulative_fund_auth_signer,
            ),
            team_amount,
        )?;

        distribution_info.last_distribution = clock.unix_timestamp;

        Ok(())
    }
}

/// Get the percent value.
pub fn calc_share_amount(share: u8, amount: u64) -> Result<u64> {
    let ui_amount = anchor_spl::token::spl_token::amount_to_ui_amount(amount, 9);
    let share_amount = ui_amount / 100.0 * share as f64;
    Ok(anchor_spl::token::spl_token::ui_amount_to_amount(
        share_amount,
        9,
    ))
}
