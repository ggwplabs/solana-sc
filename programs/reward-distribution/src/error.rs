use anchor_lang::prelude::*;

#[error_code]
pub enum RewardDistributionError {
    // Signatures and access
    #[msg("Access denied")]
    AccessDenied, // 6000

    // Misc.
    #[msg("Operation overflow")]
    Overflow, // 6001

    // Constraints
    #[msg("Invalid play to earn fund mint")]
    InvalidPlayToEarnFundMint, // 6002
    #[msg("Invalid play to earn fund owner")]
    InvalidPlayToEarnFundOwner, // 6003
    #[msg("Invalid transfer auth list")]
    InvalidTransferAuthList, // 6004

    // Reward distribution errors
}
