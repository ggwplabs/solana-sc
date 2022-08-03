use crate::state::{Settings, Wallet, USER_WALLET_SEED};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = Settings::LEN)]
    pub settings: Account<'info, Settings>,
    // Misc.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateWallet<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = Wallet::LEN,
        seeds = [
            USER_WALLET_SEED.as_bytes(),
            program_id.as_ref(),
            user.key().as_ref(),
        ],
        bump,
    )]
    pub wallet: Account<'info, Wallet>,
    // Misc.
    pub system_program: Program<'info, System>,
}
