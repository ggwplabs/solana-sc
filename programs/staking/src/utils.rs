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
    if spent_days >= hold_period_days as i64 {
        return Ok(false);
    }
    Ok(true)
}

/// Get number of epoch.
pub fn get_epoch_by_time(
    staking_start_time: UnixTimestamp,
    time: UnixTimestamp,
    epoch_period_days: u16,
) -> Result<u64> {
    let spent_time = time
        .checked_sub(staking_start_time)
        .ok_or(StakingError::Overflow)?;
    let spent_days = spent_time
        .checked_div(24 * 60 * 60)
        .ok_or(StakingError::Overflow)?;
    let epoch = spent_days
        .checked_div(epoch_period_days as i64)
        .ok_or(StakingError::Overflow)?;
    Ok(epoch as u64)
}

/// Get the current APR by epoch.
pub fn get_apr_by_epoch(epoch: u64, start_apr: u8, step_apr: u8, end_apr: u8) -> Result<u8> {
    let current_apr = start_apr as u64 - step_apr as u64 * (epoch - 1);
    let current_apr = current_apr as u8;
    if current_apr < end_apr {
        return Ok(end_apr);
    } else {
        return Ok(current_apr);
    }
}

/// Calc user reward amount by user stake amount, epoch staked.
pub fn calc_user_reward_amount(
    user_staked_amount: u64,
    user_stake_time: UnixTimestamp,
) -> Result<u64> {
    // 1/365 const
    const DEGREE: f64 = 0.002739726027397;
    // TODO: за каждую эпоху своя награда и складывается
    // r_дн = apr_current ^ DEGREE - дневной процент внутри эпохи с apr_current
    // (amount * (1 + r_дн) ^ epoch_period_days) - сумма заработка юзера за эпоху
    // далее переход на новую эпоху, где новый r_дн и эмаунт это сумма с прошлой эпохи
    Ok(0)
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
        assert_eq!(
            is_withdraw_royalty(1660032700, 1660032700 + 40000, 2),
            Ok(true)
        );
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

    #[test]
    pub fn test_get_epoch_by_time() {
        assert_eq!(get_epoch_by_time(1660032700, 1660032700, 10), Ok(0));
        assert_eq!(
            get_epoch_by_time(1660032700, 1660032700 + 5 * 24 * 60 * 60, 10),
            Ok(0)
        );
        assert_eq!(
            get_epoch_by_time(1660032700, 1660032700 + 10 * 24 * 60 * 60, 10),
            Ok(1)
        );
        assert_eq!(
            get_epoch_by_time(1660032700, 1660032700 + 15 * 24 * 60 * 60, 10),
            Ok(1)
        );
        assert_eq!(
            get_epoch_by_time(1660032700, 1660032700 + 20 * 24 * 60 * 60, 10),
            Ok(2)
        );
        assert_eq!(
            get_epoch_by_time(1660032700, 1660032700 + 300 * 24 * 60 * 60, 10),
            Ok(30)
        );
        assert_eq!(
            get_epoch_by_time(1660032700, 1660032700 + 3000 * 24 * 60 * 60, 10),
            Ok(300)
        );
    }

    #[test]
    pub fn test_get_apr_by_epoch() {
        assert_eq!(get_apr_by_epoch(1, 45, 1, 5), Ok(45));
        assert_eq!(get_apr_by_epoch(1, 45, 2, 5), Ok(45));
        assert_eq!(get_apr_by_epoch(1, 45, 10, 5), Ok(45));
        assert_eq!(get_apr_by_epoch(1, 45, 10, 40), Ok(45));
        assert_eq!(get_apr_by_epoch(2, 45, 1, 5), Ok(44));
        assert_eq!(get_apr_by_epoch(3, 45, 1, 5), Ok(43));
        assert_eq!(get_apr_by_epoch(10, 45, 1, 5), Ok(36));
        assert_eq!(get_apr_by_epoch(2, 45, 2, 5), Ok(43));
        assert_eq!(get_apr_by_epoch(3, 45, 2, 5), Ok(41));
        assert_eq!(get_apr_by_epoch(40, 45, 1, 5), Ok(6));
        assert_eq!(get_apr_by_epoch(41, 45, 1, 5), Ok(5));
        assert_eq!(get_apr_by_epoch(42, 45, 1, 5), Ok(5));
        assert_eq!(get_apr_by_epoch(43, 45, 1, 5), Ok(5));
        assert_eq!(get_apr_by_epoch(44, 45, 1, 5), Ok(5));
    }
}
