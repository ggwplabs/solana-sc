use anchor_lang::prelude::*;

#[error_code]
pub enum FightingError {
    // Signatures and access
    #[msg("Access denied")]
    AccessDenied, // 6000

    // Misc.
    #[msg("Operation overflow")]
    Overflow, // 6001

    // Constraints
    #[msg("Invalid AFK timeout in sec")]
    InvalidAFKTimeout, // 6002

    // Functional errors

}
