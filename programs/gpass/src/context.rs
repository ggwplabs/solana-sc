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
pub struct UpdateParam<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub settings: Account<'info, Settings>,
}

#[derive(Accounts)]
pub struct CreateWallet<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub user: Signer<'info>,
    #[account(init, payer = payer, space = Wallet::LEN,
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

#[derive(Accounts)]
pub struct MintTo<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub to: Account<'info, Wallet>,
    pub settings: Account<'info, Settings>,
}

#[derive(Accounts)]
pub struct Burn<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub from: Account<'info, Wallet>,
    pub settings: Account<'info, Settings>,
}

#[derive(Accounts)]
pub struct BurnInPeriod<'info> {
    #[account(mut)]
    pub wallet: Account<'info, Wallet>,
    pub settings: Account<'info, Settings>,
}
