use anchor_lang::prelude::*;

const DESCRIMINATOR_LEN: usize = 8;

pub const USER_INFO_SEED: &str = "user_info";
pub const GPASS_BURN_AUTH_SEED: &str = "gpass_burn_auth";

#[account]
#[derive(Default, Debug)]
pub struct FightingSettings {
    pub admin: Pubkey,
    pub update_auth: Pubkey,

    pub afk_timeout: i64,
    pub gpass_burn_auth_bump: u8,
}

impl FightingSettings {
    pub const LEN: usize = DESCRIMINATOR_LEN +
        32 + // admin pk
        32 + // update auth pk
        8 + // AFK timeout
        1; // gpass burn auth bump
}

#[account]
#[derive(Default, Debug)]
pub struct UserInfo {
    pub in_game: bool,
    pub in_game_time: i64,
}

impl UserInfo {
    pub const LEN: usize = DESCRIMINATOR_LEN +
        1 + // in game status
        8 // in game time
        ;
}
