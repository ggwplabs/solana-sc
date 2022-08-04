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
    #[msg("Max burners size exceeded")]
    MaxBurnersSizeExceeded, // 6003
    #[msg("Invalid burn period value")]
    InvalidBurnPeriodValue, // 6004
    #[msg("Invalid mint authority")]
    InvalidMintAuthority, // 6005
    #[msg("Invalid burn authority")]
    InvalidBurnAuthority, // 6006
    #[msg("Mint amount cannot be zero")]
    ZeroMintAmount, // 6007
    #[msg("Burn amount cannot be zero")]
    ZeroBurnAmount, // 6008

    // Functional errors
    #[msg("Invalid last burned value")]
    InvalidLastBurnedValue, // 6009
}
