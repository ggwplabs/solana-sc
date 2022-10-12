use crate::error::FightingError;
use crate::state::{
    Action, FightingSettings, GameInfo, GameResult, Identity, UserFightingInfo, GAME_INFO_SEED,
    GPASS_BURN_AUTH_SEED, REWARD_TRANSFER_AUTH_SEED, USER_INFO_SEED,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use freezing::state::FreezingInfo;
use gpass::state::{GpassInfo, Wallet};
use reward_distribution::state::RewardDistributionInfo;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = FightingSettings::LEN)]
    pub fighting_settings: Account<'info, FightingSettings>,

    pub gpass_info: Box<Account<'info, GpassInfo>>,
    pub reward_distribution_info: Box<Account<'info, RewardDistributionInfo>>,

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

    /// CHECK: Reward transfer auth PDA
    #[account(
        seeds = [
            REWARD_TRANSFER_AUTH_SEED.as_bytes(),
            fighting_settings.key().as_ref(),
            reward_distribution_info.key().as_ref(),
        ],
        bump,
    )]
    pub reward_transfer_auth: UncheckedAccount<'info>,

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

    #[account(mut)]
    pub user_ggwp_wallet: Box<Account<'info, TokenAccount>>,

    pub fighting_settings: Box<Account<'info, FightingSettings>>,
    pub freezing_info: Box<Account<'info, FreezingInfo>>,
    pub reward_distribution_info: Box<Account<'info, RewardDistributionInfo>>,

    /// CHECK: Reward transfer auth PDA
    #[account(
        seeds = [
            REWARD_TRANSFER_AUTH_SEED.as_bytes(),
            fighting_settings.key().as_ref(),
            reward_distribution_info.key().as_ref(),
        ],
        bump = fighting_settings.reward_transfer_auth_bump,
    )]
    pub reward_transfer_auth: UncheckedAccount<'info>,

    #[account(mut,
        constraint = play_to_earn_fund.key() == reward_distribution_info.play_to_earn_fund
        @FightingError::InvalidPlayToEarnFundAddress,
    )]
    pub play_to_earn_fund: Box<Account<'info, TokenAccount>>,
    /// CHECK: Play to earn fund auth for reward distribution
    pub play_to_earn_fund_auth: UncheckedAccount<'info>,

    #[account(mut)]
    pub accumulative_fund: Box<Account<'info, TokenAccount>>,

    // Misc.
    /// CHECK: reward_distribution_program
    #[account( constraint = reward_distribution_program.key() == reward_distribution::id() )]
    pub reward_distribution_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
