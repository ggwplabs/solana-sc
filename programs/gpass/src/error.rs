use anchor_lang::prelude::*;

#[error_code]
pub enum GpassError {
    // Signatures and access
    #[msg("Access denied")]
    AccessDenied, // 6000

    // Misc.
    #[msg("Operation overflow")]
    Overflow, // 6001

    // Constraints
    #[msg("Max minters size exceeded")]
    MaxMintersSizeExceeded, // 6002
    #[msg("Invalid burn period value")]
    InvalidBurnPeriodValue, // 6003
    #[msg("Invalid mint authority")]
    InvalidMintAuthority, // 6004
    #[msg("Mint amount cannot be zero")]
    ZeroMintAmount, // 6005

    // Functional errors
}
