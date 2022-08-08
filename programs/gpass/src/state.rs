use anchor_lang::prelude::*;

const DESCRIMINATOR_LEN: usize = 8;
pub const MAX_MINTERS: usize = 1;
const MINTERS_LEN: usize = 4 + MAX_MINTERS * 32;
pub const MAX_BURNERS: usize = 3;
const BURNERS_LEN: usize = 4 + MAX_BURNERS * 32;

pub const USER_WALLET_SEED: &str = "user_gpass_wallet";

#[account]
#[derive(Default, Debug)]
pub struct GpassSettings {
    pub admin: Pubkey,
    pub update_auth: Pubkey,
    pub burn_period: u64,
    pub minters: Vec<Pubkey>,
    pub burners: Vec<Pubkey>,
}

impl GpassSettings {
    pub const LEN: usize = DESCRIMINATOR_LEN +
        32 + // admin pk
        32 + // update auth
        8 + // burn period
        MINTERS_LEN + // minters list
        BURNERS_LEN; // burners list
}

#[account]
#[derive(Default, Debug)]
pub struct Wallet {
    pub amount: u64,
    pub last_burned: i64, // UnixTimestamp
}

impl Wallet {
    pub const LEN: usize = DESCRIMINATOR_LEN +
        8 + // amount
        8; // last reset
}
