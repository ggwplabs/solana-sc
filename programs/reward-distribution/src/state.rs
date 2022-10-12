use anchor_lang::prelude::*;

pub const DESCRIMINATOR_LEN: usize = 8;
pub const MAX_TRANSFER_AUTH_LIST: usize = 6;
const MAX_TRANSFER_AUTH_LIST_LEN: usize = MAX_TRANSFER_AUTH_LIST * 32;

pub const PLAY_TO_EARN_FUND_AUTH_SEED: &str = "play_to_earn_fund_auth";

#[account]
#[derive(Default, Debug)]
pub struct RewardDistributionInfo {
    pub admin: Pubkey,
    pub update_auth: Pubkey,

    pub ggwp_token: Pubkey,
    pub play_to_earn_fund: Pubkey,
    pub play_to_earn_fund_auth_bump: u8,
    pub transfer_auth_list: Vec<Pubkey>,
}

impl RewardDistributionInfo {
    pub const LEN: usize = DESCRIMINATOR_LEN +
        32 + 32 + // admin, update auth pks
        32 + // ggwp token mint
        32 + 1 + // accumulative fund + auth bump
        MAX_TRANSFER_AUTH_LIST_LEN;
}
