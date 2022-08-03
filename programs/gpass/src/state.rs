use anchor_lang::{prelude::*, solana_program::clock::UnixTimestamp};

const DESCRIMINATOR_LEN: usize = 8;
pub const MAX_MINTERS: usize = 1;
const MINTERS_LEN: usize = 1 + MAX_MINTERS * 32;

pub const USER_WALLET_SEED: &str = "user_gpass_wallet";

#[account]
#[derive(Default, Debug)]
pub struct Settings {
    pub admin: Pubkey,
    pub burn_period: UnixTimestamp,
    pub minters: Vec<Pubkey>,
}

impl Settings {
    pub const LEN: usize = DESCRIMINATOR_LEN +
        8 + // admin pk
        8 + // burn period
        MINTERS_LEN; // minters
}

#[account]
#[derive(Default, Debug)]
pub struct Wallet {
    pub amount: u64,
    pub last_reset: UnixTimestamp,
}

impl Wallet {
    pub const LEN: usize = DESCRIMINATOR_LEN +
        8 + // amount
        8; // last reset
}
