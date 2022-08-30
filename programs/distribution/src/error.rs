use anchor_lang::prelude::*;

#[error_code]
pub enum DistributionError {
    // Signatures and access
    #[msg("Access denied")]
    AccessDenied, // 6000

    // Misc.
    #[msg("Operation overflow")]
    Overflow, // 6001

    // Constraints
    #[msg("Invalid accumulative fund mint")]
    InvalidAccumulativeFundMint, // 6002
    #[msg("Invalid accumulative fund owner")]
    InvalidAccumulativeFundOwner, // 6003
    #[msg("Invalid play to earn fund mint")]
    InvalidPlayToEarnFundMint, // 6004
    #[msg("Invalid staking fund mint")]
    InvalidStakingFundMint, // 6005
    #[msg("Invalid company fund mint")]
    InvalidCompanyFundMint, // 6006
    #[msg("Invalid team fund mint")]
    InvalidTeamFundMint, // 6007

    #[msg("Invalid share percent value")]
    InvalidShare, // 6008
}
