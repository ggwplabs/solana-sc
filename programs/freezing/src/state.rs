use anchor_lang::prelude::*;

const DESCRIMINATOR_LEN: usize = 8;

pub const USER_INFO_SEED: &str = "user_info";

#[account]
#[derive(Default, Debug)]
pub struct FreezingParams {
    // Administrator can call the admin only instructions
    pub admin: Pubkey,
    pub update_auth: Pubkey,

    pub ggwp_token: Pubkey,
    pub gpass_token: Pubkey,

    // Wallet for royalty
    pub accumulative_fund: Pubkey,
    // Wallet for freezed GGWP
    pub treasury: Pubkey,

    // TODO: additional params
    pub gpass_mint_auth_bump: u8,
}

impl FreezingParams {
    pub const LEN: usize = DESCRIMINATOR_LEN +
        32 + // admin pk
        32 + // update auth pk
        32 + 32 + // tokens
        32 + // fund pk
        32 + // treasury pk
        8; // bump
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
