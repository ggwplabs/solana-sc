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
    #[msg("Invalid accumulative fund pk")]
    InvalidAccumulativeFundPK, // 6004
    #[msg("Invalid treasury mint PK")]
    InvalidTreasuryMint, // 6005
    #[msg("Invalid user GGWP wallet mint")]
    InvalidUserGGWPWalletMint, // 6006
    #[msg("Invalid user GGWP wallet owner")]
    InvalidUserGGWPWalletOwner, // 6007
    #[msg("Invalid treasury pk")]
    InvalidTreasuryPK, // 6008
    #[msg("Invalid royalty value")]
    InvalidRoyaltyValue, // 6009
    #[msg("Invalid unfreeze royalty value")]
    InvalidUnfreezeRoyaltyValue, // 6010
    #[msg("Invalid unfreeze lock period")]
    InvalidUnfreezeLockPeriod, // 6011
    #[msg("Invalid reward table")]
    InvalidRewardTable, // 6012
    #[msg("Invalid reward period value")]
    InvalidRewardPeriod, // 6013

    // Freezing errors
    #[msg("Freezing amount cannot be zero")]
    ZeroFreezingAmount, // 6014
    #[msg("Unfreezing amount cannot be zero")]
    ZeroUnfreezingAmount, // 6015
    #[msg("Additional freezing is not available")]
    AdditionalFreezingNotAvailable, // 6016
    #[msg("Zero GPASS earned")]
    ZeroGpassEarned, // 6017
}
