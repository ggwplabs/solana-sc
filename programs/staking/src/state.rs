use anchor_lang::prelude::*;

pub const DESCRIMINATOR_LEN: usize = 8;

pub const TREASURY_AUTH_SEED: &str = "treasury_auth";
pub const STAKING_FUND_AUTH_SEED: &str = "staking_fund_auth";

#[account]
#[derive(Default, Debug)]
pub struct StakingInfo {
    pub admin: Pubkey,
    pub update_auth: Pubkey,

    pub ggwp_token: Pubkey,

    pub accumulative_fund: Pubkey,
    pub staking_fund: Pubkey,
    pub staking_fund_auth_bump: u8,
    pub treasury: Pubkey,
    pub treasury_auth_bump: u8,

    pub total_staked: u64,
    pub epoch: u64,
    pub epoch_period_days: u16,
    pub min_stake_amount: u64,
    pub hold_period_days: u16,
    pub hold_royalty: u8,
    pub royalty: u8,
    pub apr_start: u8,
    pub apr_step: u8,
    pub apr_end: u8,
}

impl StakingInfo {
    pub const LEN: usize = DESCRIMINATOR_LEN +
        32 + 32 + // admin, update auth pks
        32 + // ggwp token mint
        32 + // accumulative fund
        32 + 1 + // staking fund + bump
        32 + 1 + // treasury auth + bump
        8 + // total staked
        8 + // current epoch
        2 + // epoch length in days
        8 + // minimum stake amount
        2 + // hold period in days
        1 + // hold royalty percent
        1 + // royalty
        1 + 1 + 1; // start, step, end apr
}

#[account]
#[derive(Default, Debug)]
pub struct UserInfo {}
