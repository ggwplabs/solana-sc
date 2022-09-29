use crate::state::{
    Action, FightingSettings, GameInfo, GameResult, Identity, UserFightingInfo, GAME_INFO_SEED,
    GPASS_BURN_AUTH_SEED, USER_INFO_SEED,
};
use anchor_lang::prelude::*;
use gpass::state::{GpassInfo, Wallet};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = FightingSettings::LEN)]
    pub fighting_settings: Account<'info, FightingSettings>,

    pub gpass_info: Box<Account<'info, GpassInfo>>,

    /// CHECK: GPASS Burn auth PDA
    #[account(
        seeds = [
            GPASS_BURN_AUTH_SEED.as_bytes(),
            fighting_settings.key().as_ref(),
            gpass_info.key().as_ref(),
        ],
        bump,
    )]
    pub gpass_burn_auth: UncheckedAccount<'info>,

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
pub struct StartGame<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init_if_needed, payer = user, space = UserFightingInfo::LEN,
        seeds = [
            USER_INFO_SEED.as_bytes(),
            fighting_settings.key().as_ref(),
            user.key().as_ref(),
        ],
        bump
    )]
    pub user_info: Box<Account<'info, UserFightingInfo>>,

    pub fighting_settings: Box<Account<'info, FightingSettings>>,

    #[account(mut)]
    pub gpass_info: Box<Account<'info, GpassInfo>>,

    #[account(mut)]
    pub user_gpass_wallet: Box<Account<'info, Wallet>>,

    /// CHECK: GPASS Burn auth PDA
    #[account(
        seeds = [
            GPASS_BURN_AUTH_SEED.as_bytes(),
            fighting_settings.key().as_ref(),
            gpass_info.key().as_ref(),
        ],
        bump = fighting_settings.gpass_burn_auth_bump,
    )]
    pub gpass_burn_auth: UncheckedAccount<'info>,

    // Misc.
    /// CHECK: GPASS program
    #[account( constraint = gpass_program.key() == gpass::id() )]
    pub gpass_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
    game_id: u64,
    game_result: GameResult,
    actions_log: Vec<(Identity, Action)>,
)]
pub struct FinalizeGame<'info> {
    pub user: Signer<'info>,
    #[account(mut)]
    pub validator: Signer<'info>,

    #[account(init, payer = validator, space = GameInfo::LEN,
        seeds = [
            GAME_INFO_SEED.as_bytes(),
            fighting_settings.key().as_ref(),
            user.key().as_ref(),
            game_id.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub game_info: Box<Account<'info, GameInfo>>,

    #[account(mut,
        seeds = [
            USER_INFO_SEED.as_bytes(),
            fighting_settings.key().as_ref(),
            user.key().as_ref(),
        ],
        bump
    )]
    pub user_info: Box<Account<'info, UserFightingInfo>>,

    pub fighting_settings: Box<Account<'info, FightingSettings>>,

    // Misc.
    pub system_program: Program<'info, System>,
}
