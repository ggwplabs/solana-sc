use crate::{
    error::StakingError,
};
use anchor_lang::{prelude::*, solana_program::clock::UnixTimestamp};
use anchor_spl::token::spl_token::{amount_to_ui_amount, ui_amount_to_amount};

/// Get the percent value.
pub fn calc_royalty_amount(royalty: u8, amount: u64) -> Result<u64> {
    let ui_amount = amount_to_ui_amount(amount, 9);
    let royalty_amount = ui_amount / 100.0 * royalty as f64;
    Ok(ui_amount_to_amount(royalty_amount, 9))
}

/// Checks freezed time for withdraw royalty.
pub fn is_withdraw_royalty(
    current_time: UnixTimestamp,
    freezed_time: UnixTimestamp,
    unfreeze_lock_period: UnixTimestamp,
) -> Result<bool> {
    let spent_time = current_time
        .checked_sub(freezed_time)
        .ok_or(StakingError::Overflow)?;
    if spent_time >= unfreeze_lock_period {
        return Ok(false);
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_calc_royalty_amount() {
        assert_eq!(calc_royalty_amount(0, 0), Ok(0));
        assert_eq!(calc_royalty_amount(0, 1000), Ok(0));
        assert_eq!(calc_royalty_amount(8, 0), Ok(0));
        assert_eq!(calc_royalty_amount(8, 1000), Ok(80));
        assert_eq!(calc_royalty_amount(50, 5000), Ok(2500));
        assert_eq!(calc_royalty_amount(50, 5_000_000_000), Ok(2_500_000_000));
    }

    #[test]
    pub fn test_is_withdraw_royalty() {
        assert_eq!(is_withdraw_royalty(1660032700, 1660032700, 100), Ok(true));
        assert_eq!(is_withdraw_royalty(1660032700, 1660032650, 100), Ok(true));
        assert_eq!(is_withdraw_royalty(1660032700, 1660032800, 100), Ok(true));
        assert_eq!(is_withdraw_royalty(1660032700, 1660032500, 100), Ok(false));
        assert_eq!(is_withdraw_royalty(1660032700, 1660032300, 100), Ok(false));
    }
}
