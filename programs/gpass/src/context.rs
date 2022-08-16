use crate::state::{GpassInfo, Wallet, USER_WALLET_SEED};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = GpassInfo::LEN)]
    pub gpass_info: Account<'info, GpassInfo>,
    // Misc.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateParam<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub gpass_info: Account<'info, GpassInfo>,
}

#[derive(Accounts)]
pub struct CreateWallet<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub user: SystemAccount<'info>,
    pub gpass_info: Account<'info, GpassInfo>,
    #[account(init, payer = payer, space = Wallet::LEN,
        seeds = [
            USER_WALLET_SEED.as_bytes(),
            gpass_info.key().as_ref(),
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
    #[account(mut)]
    pub gpass_info: Account<'info, GpassInfo>,
}

#[derive(Accounts)]
pub struct Burn<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub from: Account<'info, Wallet>,
    #[account(mut)]
    pub gpass_info: Account<'info, GpassInfo>,
}

#[derive(Accounts)]
pub struct BurnInPeriod<'info> {
    #[account(mut)]
    pub wallet: Account<'info, Wallet>,
    #[account(mut)]
    pub gpass_info: Account<'info, GpassInfo>,
}
