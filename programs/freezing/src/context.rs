use crate::error::*;
use crate::state::{FreezingParams, UserInfo, GPASS_MINT_AUTH_SEED, USER_INFO_SEED};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use gpass::state::{GpassSettings, Wallet, USER_WALLET_SEED};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = FreezingParams::LEN)]
    pub freezing_params: Box<Account<'info, FreezingParams>>,

    /// CHECK: GPASS Mint auth PDA
    #[account(
        seeds = [
            GPASS_MINT_AUTH_SEED.as_bytes(),
            freezing_params.key().as_ref(),
            gpass_settings.key().as_ref()
        ],
        bump,
    )]
    pub gpass_mint_auth: UncheckedAccount<'info>,

    pub ggwp_token: Box<Account<'info, Mint>>,
    pub gpass_settings: Box<Account<'info, GpassSettings>>,

    #[account(
        constraint = accumulative_fund.mint == ggwp_token.key()
        @FreezingError::InvalidAccumulativeFundMint,
    )]
    pub accumulative_fund: Box<Account<'info, TokenAccount>>,
    #[account(
        constraint = treasury.mint == ggwp_token.key()
        @FreezingError::InvalidTreasuryMint,
    )]
    pub treasury: Box<Account<'info, TokenAccount>>,

    // Misc.
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateParams<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub freezing_params: Account<'info, FreezingParams>,
}

#[derive(Accounts)]
pub struct Freeze<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init_if_needed, payer = user, space = UserInfo::LEN,
        seeds = [
            USER_INFO_SEED.as_bytes(),
            freezing_params.key().as_ref(),
            user.key().as_ref(),
        ],
        bump,
    )]
    pub user_info: Box<Account<'info, UserInfo>>,

    #[account(mut)]
    pub freezing_params: Box<Account<'info, FreezingParams>>,

    pub gpass_settings: Box<Account<'info, GpassSettings>>,

    #[account(mut,
        constraint = user_ggwp_wallet.mint == freezing_params.ggwp_token
        @FreezingError::InvalidUserGGWPWalletMint,
        constraint = user_ggwp_wallet.owner == user.key()
        @FreezingError::InvalidUserGGWPWalletOwner,
    )]
    pub user_ggwp_wallet: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        seeds = [
            USER_WALLET_SEED.as_bytes(),
            gpass_settings.key().as_ref(),
            user.key().as_ref(),
        ],
        bump
    )]
    pub user_gpass_wallet: Box<Account<'info, Wallet>>,

    #[account(mut,
        constraint = treasury.key() == freezing_params.treasury
        @FreezingError::InvalidTreasuryPK,
    )]
    pub treasury: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        constraint = accumulative_fund.key() == freezing_params.accumulative_fund
        @FreezingError::InvalidAccumulativeFundPK,
    )]
    pub accumulative_fund: Box<Account<'info, TokenAccount>>,

    /// CHECK: Mint auth PDA
    #[account(
        seeds = [
            GPASS_MINT_AUTH_SEED.as_bytes(),
            freezing_params.key().as_ref(),
            gpass_settings.key().as_ref()
        ],
        bump = freezing_params.gpass_mint_auth_bump,
    )]
    pub gpass_mint_auth: UncheckedAccount<'info>,

    // Misc.
    /// CHECK: GPASS program
    #[account( constraint = gpass_program.key() == gpass::id() )]
    pub gpass_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub user: Signer<'info>,
    // #[account(mut,
    //     seeds = [
    //         program_id.as_ref(),
    //         freezing_params.key().as_ref(),
    //         user.key().as_ref(),
    //         USER_INFO_SEED.as_bytes(),
    //     ],
    //     bump,
    // )]
    // pub user_info: Box<Account<'info, UserInfo>>,

    // pub freezing_params: Box<Account<'info, FreezingParams>>,

    // #[account(mut,
    //     constraint = gpass_token.key() == freezing_params.gpass_token,
    // )]
    // pub gpass_token: Box<Account<'info, Mint>>,

    // #[account(mut,
    //     constraint = user_gpass_wallet.mint == freezing_params.gpass_token
    //     @FreezingError::InvalidUserGPASSWalletMint,
    //     constraint = user_gpass_wallet.owner == user.key()
    //     @FreezingError::InvalidUserGPASSWalletOwner,
    // )]
    // pub user_gpass_wallet: Box<Account<'info, TokenAccount>>,

    // /// CHECK: Mint auth PDA
    // #[account(
    //     seeds = [
    //         program_id.as_ref(),
    //         freezing_params.to_account_info().key.as_ref(),
    //         gpass_mint_auth.key().as_ref()
    //     ],
    //     bump = freezing_params.gpass_mint_auth_bump,
    // )]
    // pub gpass_mint_auth: UncheckedAccount<'info>,

    // // Misc.
    // pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Unfreeze<'info> {
    pub user: Signer<'info>,
    // #[account(mut,
    //     seeds = [
    //         program_id.as_ref(),
    //         freezing_params.key().as_ref(),
    //         user.key().as_ref(),
    //         USER_INFO_SEED.as_bytes(),
    //     ],
    //     bump,
    // )]
    // pub user_info: Box<Account<'info, UserInfo>>,

    // pub freezing_params: Box<Account<'info, FreezingParams>>,

    // #[account(mut,
    //     constraint = gpass_token.key() == freezing_params.gpass_token,
    // )]
    // pub gpass_token: Box<Account<'info, Mint>>,

    // #[account(mut,
    //     constraint = user_gpass_wallet.mint == freezing_params.gpass_token
    //     @FreezingError::InvalidUserGPASSWalletMint,
    //     constraint = user_gpass_wallet.owner == user.key()
    //     @FreezingError::InvalidUserGPASSWalletOwner,
    // )]
    // pub user_gpass_wallet: Box<Account<'info, TokenAccount>>,

    // /// CHECK: Mint auth PDA
    // #[account(
    //     seeds = [
    //         program_id.as_ref(),
    //         freezing_params.to_account_info().key.as_ref(),
    //         gpass_mint_auth.key().as_ref()
    //     ],
    //     bump = freezing_params.gpass_mint_auth_bump,
    // )]
    // pub gpass_mint_auth: UncheckedAccount<'info>,

    // // Misc.
    // pub token_program: Program<'info, Token>,
}
