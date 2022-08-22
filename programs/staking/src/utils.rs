use crate::error::StakingError;
use anchor_lang::{prelude::*, solana_program::clock::UnixTimestamp};
use anchor_spl::token::spl_token::{amount_to_ui_amount, ui_amount_to_amount};

/// Get the percent value.
pub fn calc_royalty_amount(royalty: u8, amount: u64) -> Result<u64> {
    let ui_amount = amount_to_ui_amount(amount, 9);
    let royalty_amount = ui_amount / 100.0 * royalty as f64;
    Ok(ui_amount_to_amount(royalty_amount, 9))
}

/// Checks stake time for withdraw royalty.
pub fn is_withdraw_royalty(
    current_time: UnixTimestamp,
    stake_time: UnixTimestamp,
    hold_period_days: u16,
) -> Result<bool> {
    let spent_time = current_time
        .checked_sub(stake_time)
        .ok_or(StakingError::Overflow)?;
    let spent_days = spent_time
        .checked_div(24 * 60 * 60)
        .ok_or(StakingError::Overflow)?;
    println!("Spent days: {}", spent_days);
    if spent_days >= hold_period_days as i64 {
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
        assert_eq!(is_withdraw_royalty(1660032700, 1660032700, 2), Ok(true));
        assert_eq!(is_withdraw_royalty(1660032700, 1660032700 + 40000, 2), Ok(true));
        assert_eq!(
            is_withdraw_royalty(1660032700, 1660032700 - 1 * 24 * 60 * 60, 2),
            Ok(true)
        );
        assert_eq!(
            is_withdraw_royalty(1660032700, 1660032700 - 1 * 24 * 60 * 60 + 100, 2),
            Ok(true)
        );
        assert_eq!(
            is_withdraw_royalty(1660032700, 1660032700 - 2 * 24 * 60 * 60, 2),
            Ok(false)
        );
        assert_eq!(
            is_withdraw_royalty(1660032700, 1660032700 - 31 * 24 * 60 * 60, 30),
            Ok(false)
        );
    }
}
