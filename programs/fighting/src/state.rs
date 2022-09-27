use anchor_lang::prelude::*;

const DESCRIMINATOR_LEN: usize = 8;

#[account]
#[derive(Default, Debug)]
pub struct FightingSettings {
    pub admin: Pubkey,
    pub update_auth: Pubkey,

    pub afk_timeout_sec: i64,
}

impl FightingSettings {
    pub const LEN: usize = DESCRIMINATOR_LEN +
        32 + // admin pk
        32 + // update auth pk
        8; // AFK timeout sec
}
