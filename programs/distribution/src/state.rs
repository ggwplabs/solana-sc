use anchor_lang::prelude::*;

pub const DESCRIMINATOR_LEN: usize = 8;

pub const ACCUMULATIVE_FUND_AUTH_SEED: &str = "accumulative_fund_auth";

#[account]
#[derive(Default, Debug)]
pub struct DistributionInfo {
    pub admin: Pubkey,
    pub update_auth: Pubkey,

    pub ggwp_token: Pubkey,
    pub accumulative_fund: Pubkey,
    pub accumulative_fund_auth_bump: u8,

    pub play_to_earn_fund: Pubkey,
    pub play_to_earn_fund_share: u8,
    pub staking_fund: Pubkey,
    pub staking_fund_share: u8,
    pub company_fund: Pubkey,
    pub company_fund_share: u8,
    pub team_fund: Pubkey,
    pub team_fund_share: u8,
}

impl DistributionInfo {
    pub const LEN: usize = DESCRIMINATOR_LEN +
        32 + 32 + // admin, update auth pks
        32 + // ggwp token mint
        32 + 1 + // accumulative fund + auth bump
        32 + 1 + // play to earn fund + share
        32 + 1 + // staking fund + share
        32 + 1 + // company fund + share
        32 + 1; // team fund + share
}
