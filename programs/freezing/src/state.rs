use anchor_lang::prelude::*;

const DESCRIMINATOR_LEN: usize = 8;
const MAX_REWARDS_TABLE_ROWS: usize = 5;
const REWARD_TABLE_ROW_SIZE: usize = 8 + 8;
const MAX_REWARD_TABLE_LEN: usize = 8 + REWARD_TABLE_ROW_SIZE * MAX_REWARDS_TABLE_ROWS;

pub const GPASS_MINT_AUTH_SEED: &str = "gpass_mint_auth";
pub const USER_INFO_SEED: &str = "user_info";

#[account]
#[derive(Default, Debug)]
pub struct FreezingParams {
    // Administrator can call the admin only instructions
    pub admin: Pubkey,
    pub update_auth: Pubkey,

    pub ggwp_token: Pubkey,
    pub gpass_settings: Pubkey,
    pub gpass_mint_auth_bump: u8,

    // Wallet for royalty
    pub accumulative_fund: Pubkey,
    // Wallet for freezed GGWP
    pub treasury: Pubkey,

    pub total_freezed: u64,
    pub reward_period: i64,
    pub royalty: u8,
    pub unfreeze_royalty: u8,
    pub unfreeze_lock_period: i64,

    pub reward_table: Vec<RewardTableRow>,
}

impl FreezingParams {
    pub const LEN: usize = DESCRIMINATOR_LEN +
        32 + // admin pk
        32 + // update auth pk
        32 + // ggwp token
        32 + 1 + // gpass settings, gpass mint auth bump
        32 + // fund pk
        32 + // treasury pk
        8 + // total freezed
        8 + // reward period
        1 + 1 + // royalty percents
        8 + // unfreeze lock time in secs
        MAX_REWARD_TABLE_LEN;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug)]
pub struct RewardTableRow {
    pub ggwp_amount: u64,
    pub gpass_amount: u64,
}

#[account]
#[derive(Default, Debug)]
pub struct UserInfo {
    pub is_initialized: bool,
    pub freezed_amount: u64,
    pub freezed_time: i64,       // UnixTimestamp
    pub last_getting_gpass: i64, // UnixTimestamp
}

impl UserInfo {
    pub const LEN: usize = DESCRIMINATOR_LEN +
        1 + // is initialized
        8 + // freezed amount
        8 + // freezed time
        8; // last getting gpass
}
