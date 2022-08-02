use anchor_lang::prelude::*;

#[error_code]
pub enum FreezingError {
    // Signatures and access
    #[msg("Access denied")]
    AccessDenied, // 6000

    // Constraints
    #[msg("Invalid GPASS mint authority")]
    InvalidGPASSMintAuth, // 6001
    #[msg("Invalid accumulative fund mint PK")]
    InvalidAccumulativeFundMint, // 6002
}
