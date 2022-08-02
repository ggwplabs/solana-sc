use anchor_lang::prelude::*;

const DESCRIMINATOR_LEN: usize = 8;

#[account]
#[derive(Default, Debug)]
pub struct FreezingParams {
    // Administrator can call the admin only instructions
    pub admin: Pubkey,
    pub mint_auth_bump: u8,

    pub ggwp_token: Pubkey,
    pub gpass_token: Pubkey,

    // Wallet for royalty
    pub accumulative_fund: Pubkey,
}

impl FreezingParams {
    pub const LEN: usize = DESCRIMINATOR_LEN +
        32 + // admin pk
        8 + // bump
        32 + 32 + // tokens
        32; // fund pk
}
