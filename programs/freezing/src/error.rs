use anchor_lang::prelude::*;

#[error_code]
pub enum FreezingError {
    // Signatures and access
    #[msg("Access denied")]
    AccessDenied, // 6000

    // Misc.
    #[msg("Operation overflow")]
    Overflow, // 6001

    // Constraints
    #[msg("Invalid GPASS mint authority")]
    InvalidGPASSMintAuth, // 6002
    #[msg("Invalid accumulative fund mint PK")]
    InvalidAccumulativeFundMint, // 6003
    #[msg("Invalid treasury mint PK")]
    InvalidTreasuryMint, // 6004
    #[msg("Invalid user GGWP wallet mint")]
    InvalidUserGGWPWalletMint, // 6005
    #[msg("Invalid user GGWP wallet owner")]
    InvalidUserGGWPWalletOwner, // 6006
    #[msg("Invalid treasury pk")]
    InvalidTreasuryPK, // 6007
    #[msg("Invalid royalty value")]
    InvalidRoyaltyValue, // 6008
    #[msg("Invalid unfreeze royalty value")]
    InvalidUnfreezeRoyaltyValue, // 6009
    #[msg("Invalid unfreeze lock time")]
    InvalidUnfreezeLockTime, // 6010

    // Freezing errors
    #[msg("Freezing amount cannot be zero")]
    ZeroFreezingAmount, // 6011
    #[msg("Unfreezing amount cannot be zero")]
    ZeroUnfreezingAmount, // 6012
}
