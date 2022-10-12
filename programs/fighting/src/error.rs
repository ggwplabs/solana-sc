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
    #[msg("Invalid royalty value")]
    InvalidRoyaltyValue, // 6003

    // Functional errors
    #[msg("Not enough gpass for game")]
    NotEnoughGpass, // 6004
    #[msg("Still in game")]
    StillInGame, // 6005
    #[msg("User not in game")]
    UserNotInGame, // 6006
    #[msg("Invalid actions log size")]
    InvalidActionsLogSize, // 6007
    #[msg("Invalid play to earn fund address")]
    InvalidPlayToEarnFundAddress, // 6008
}
