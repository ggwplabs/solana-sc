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
    #[msg("Not enough gpass for game")]
    NotEnoughGpass, // 6003
    #[msg("Still in game")]
    StillInGame, // 6004
    #[msg("User not in game")]
    UserNotInGame, // 6005
    #[msg("Invalid actions log size")]
    InvalidActionsLogSize, // 6006
}
