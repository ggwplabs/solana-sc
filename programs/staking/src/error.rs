use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    // Signatures and access
    #[msg("Access denied")]
    AccessDenied, // 6000

    // Misc.
    #[msg("Operation overflow")]
    Overflow, // 6001

    // Constraints
    #[msg("Invalid epoch period days")]
    InvalidEpochPeriodDays, // 6002
    #[msg("Invalid min stake amount")]
    InvalidMinStakeAmount, // 6003
    #[msg("Invalid hold period days")]
    InvalidHoldPeriodDays, // 6004
    #[msg("Invalid hold royalty")]
    InvalidHoldRoyalty, // 6005
    #[msg("Invalid royalty")]
    InvalidRoyalty, // 6006
    #[msg("Invalid APR")]
    InvalidAPR, // 6007
    #[msg("Invalid accumulative fund mint PK")]
    InvalidAccumulativeFundMint, // 6008
    #[msg("Invalid accumulative fund pk")]
    InvalidAccumulativeFundPK, // 6009
    #[msg("Invalid treasury mint PK")]
    InvalidTreasuryMint, // 6010
    #[msg("Invalid treasury owner PK")]
    InvalidTreasuryOwner, // 6011
    #[msg("Invalid user GGWP wallet mint")]
    InvalidUserGGWPWalletMint, // 6012
    #[msg("Invalid staking fund mint PK")]
    InvalidStakingFundMint, // 6013
    #[msg("Invalid staking fund owner PK")]
    InvalidStakingFundOwner, // 6014

    // Functional errors
}
