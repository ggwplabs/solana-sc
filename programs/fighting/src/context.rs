use crate::state::FightingSettings;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = FightingSettings::LEN)]
    pub fighting_settings: Account<'info, FightingSettings>,

    // Misc.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateSetting<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub fighting_settings: Account<'info, FightingSettings>,
}

#[derive(Accounts)]
pub struct StartGame {}

#[derive(Accounts)]
pub struct FinalizeGame {}
