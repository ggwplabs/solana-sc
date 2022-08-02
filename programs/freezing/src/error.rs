use anchor_lang::prelude::*;

#[error_code]
pub enum ProxyStakingError {
    // Signatures and access
    #[msg("Access denied")]
    AccessDenied, // 6000
}
