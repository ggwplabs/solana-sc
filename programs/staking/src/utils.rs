use crate::error::StakingError;
use anchor_lang::{prelude::*, solana_program::clock::UnixTimestamp};
use anchor_spl::token::spl_token::{amount_to_ui_amount, ui_amount_to_amount};
use std::ops::Mul;

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
) -> Result<(u64, bool)> {
    let spent_time = time
        .checked_sub(staking_start_time)
        .ok_or(StakingError::Overflow)?;
    let spent_days = spent_time
        .checked_div(24 * 60 * 60)
        .ok_or(StakingError::Overflow)?;

    let epoch_period_days = epoch_period_days as i64;
    let epoch = spent_days
        .checked_div(epoch_period_days)
        .ok_or(StakingError::Overflow)?;

    let is_full_epoch = if (spent_days % epoch_period_days) == 0 {
        true
    } else {
        false
    };

    Ok((epoch as u64 + 1, is_full_epoch))
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

/// Get the vector of epoch past since start time
pub fn calc_user_past_epochs(
    staking_start_time: UnixTimestamp,
    user_stake_time: UnixTimestamp,
    current_time: UnixTimestamp,
    epoch_period_days: u16,
) -> Result<Vec<u64>> {
    let mut epochs = vec![];
    let (mut user_start_epoch, is_user_start_epoch_full) =
        get_epoch_by_time(staking_start_time, user_stake_time, epoch_period_days)?;
    let (user_end_epoch, _) =
        get_epoch_by_time(staking_start_time, current_time, epoch_period_days)?;

    if !is_user_start_epoch_full {
        user_start_epoch += 1;
    }

    msg!("User start epoch: {}", user_start_epoch);
    msg!("User end epoch: {}", user_end_epoch);
    for epoch in user_start_epoch..user_end_epoch {
        epochs.push(epoch);
    }

    Ok(epochs)
}

// TODO: test
/// Calc user reward amount by user stake amount, epoch staked.
pub fn calc_user_reward_amount(
    epoch_period_days: u16,
    staking_start_time: UnixTimestamp,
    staking_start_apr: u8,
    staking_step_apr: u8,
    staking_end_apr: u8,
    user_staked_amount: u64,
    user_stake_time: UnixTimestamp,
    current_time: UnixTimestamp,
) -> Result<u64> {
    let epochs = calc_user_past_epochs(
        staking_start_time,
        user_stake_time,
        current_time,
        epoch_period_days,
    )?;
    msg!("User epochs: {:?}", epochs);

    let mut user_new_amount = amount_to_ui_amount(user_staked_amount, 9);
    println!("User start amount ui: {}", user_new_amount);
    for epoch in epochs {
        let current_apr =
            get_apr_by_epoch(epoch, staking_start_apr, staking_step_apr, staking_end_apr)? as f64;
        println!("Current epoch: {}", epoch);
        println!("Current apr %: {}", current_apr);
        let current_apr = current_apr / 100.0;
        println!("Current apr: {}", current_apr);
        user_new_amount =
            user_new_amount.mul((1.0 + current_apr / 365.0).powi(epoch_period_days as i32));
        println!("Current new amount ui: {}", user_new_amount);
    }

    println!("User new amount ui: {}", user_new_amount);
    let user_new_amount = ui_amount_to_amount(user_new_amount, 9);
    println!("User new amount: {}", user_new_amount);
    let user_reward = user_new_amount
        .checked_sub(user_staked_amount)
        .ok_or(StakingError::Overflow)?;

    println!("User reward amount: {}", user_reward);
    println!(
        "User reward amount ui: {}",
        amount_to_ui_amount(user_reward, 9)
    );

    Ok(user_reward)
}

#[cfg(test)]
#[allow(non_upper_case_globals)]
mod tests {
    use super::*;

    const time: i64 = 1660032700;
    const day: i64 = 24 * 60 * 60;

    #[test]
    pub fn test_calc_user_reward_amount() {
        // TODO
        let amount = 10_000_000_000;
        // User rewards for first epoch
        assert_eq!(
            calc_user_reward_amount(10, time, 10, 1, 5, amount, time, time + 10 * day),
            Ok(27431062) // 0.027431062
        );

        // TODO: amounts less than 1 GGWP (0.1 etc)

        // User stake in half epoch

        // User stake in next epoch

        // User stake in half next epoch

        // Big amounts check overflow
        let amount: u64 = 100000_000_000_000;
    }

    #[test]
    pub fn test_calc_user_reward_amount_zero_epochs() {
        let amount = 100_000_000_000;
        assert_eq!(
            calc_user_reward_amount(10, time, 10, 1, 5, amount, time, time + 5 * day),
            Ok(0)
        );
        assert_eq!(
            calc_user_reward_amount(10, time, 10, 1, 5, amount, time + 5 * day, time + 5 * day),
            Ok(0)
        );
        assert_eq!(
            calc_user_reward_amount(10, time, 10, 1, 5, amount, time + 5 * day, time + 10 * day),
            Ok(0)
        );
        assert_eq!(
            calc_user_reward_amount(10, time, 10, 1, 5, amount, time + 5 * day, time + 15 * day),
            Ok(0)
        );
        assert_eq!(
            calc_user_reward_amount(10, time, 10, 1, 5, amount, time + 10 * day, time + 10 * day),
            Ok(0)
        );
        assert_eq!(
            calc_user_reward_amount(10, time, 10, 1, 5, amount, time + 15 * day, time + 15 * day),
            Ok(0)
        );
        assert_eq!(
            calc_user_reward_amount(10, time, 10, 1, 5, amount, time + 15 * day, time + 20 * day),
            Ok(0)
        );
    }

    #[test]
    pub fn test_calc_user_past_epoch() {
        // User starts with staking
        assert_eq!(calc_user_past_epochs(time, time, time, 10), Ok(vec![]));
        assert_eq!(
            calc_user_past_epochs(time, time, time + 5 * day, 10),
            Ok(vec![])
        );
        assert_eq!(
            calc_user_past_epochs(time, time, time + 9 * day, 10),
            Ok(vec![])
        );
        assert_eq!(
            calc_user_past_epochs(time, time, time + 10 * day, 10),
            Ok(vec![1])
        );
        assert_eq!(
            calc_user_past_epochs(time, time, time + 11 * day, 10),
            Ok(vec![1])
        );
        assert_eq!(
            calc_user_past_epochs(time, time, time + 19 * day, 10),
            Ok(vec![1])
        );
        assert_eq!(
            calc_user_past_epochs(time, time, time + 20 * day, 10),
            Ok(vec![1, 2])
        );
        assert_eq!(
            calc_user_past_epochs(time, time, time + 21 * day, 10),
            Ok(vec![1, 2])
        );
        assert_eq!(
            calc_user_past_epochs(time, time, time + 30 * day, 10),
            Ok(vec![1, 2, 3])
        );

        // User starts later in first epoch
        assert_eq!(
            calc_user_past_epochs(time, time + 5 * day, time + 5 * day, 10),
            Ok(vec![])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 5 * day, time + 10 * day, 10),
            Ok(vec![])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 5 * day, time + 15 * day, 10),
            Ok(vec![])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 5 * day, time + 20 * day, 10),
            Ok(vec![2])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 5 * day, time + 25 * day, 10),
            Ok(vec![2])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 5 * day, time + 30 * day, 10),
            Ok(vec![2, 3])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 5 * day, time + 35 * day, 10),
            Ok(vec![2, 3])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 5 * day, time + 40 * day, 10),
            Ok(vec![2, 3, 4])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 9 * day, time + 10 * day, 10),
            Ok(vec![])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 9 * day, time + 20 * day, 10),
            Ok(vec![2])
        );

        // User starts in next (4) epoch
        assert_eq!(
            calc_user_past_epochs(time, time + 30 * day, time + 30 * day, 10),
            Ok(vec![])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 30 * day, time + 35 * day, 10),
            Ok(vec![])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 30 * day, time + 40 * day, 10),
            Ok(vec![4])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 30 * day, time + 45 * day, 10),
            Ok(vec![4])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 30 * day, time + 50 * day, 10),
            Ok(vec![4, 5])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 30 * day, time + 55 * day, 10),
            Ok(vec![4, 5])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 30 * day, time + 60 * day, 10),
            Ok(vec![4, 5, 6])
        );

        // User starts in half of epoch (4)
        assert_eq!(
            calc_user_past_epochs(time, time + 35 * day, time + 35 * day, 10),
            Ok(vec![])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 35 * day, time + 40 * day, 10),
            Ok(vec![])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 35 * day, time + 45 * day, 10),
            Ok(vec![])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 35 * day, time + 50 * day, 10),
            Ok(vec![5])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 35 * day, time + 55 * day, 10),
            Ok(vec![5])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 35 * day, time + 60 * day, 10),
            Ok(vec![5, 6])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 35 * day, time + 65 * day, 10),
            Ok(vec![5, 6])
        );
        assert_eq!(
            calc_user_past_epochs(time, time + 35 * day, time + 70 * day, 10),
            Ok(vec![5, 6, 7])
        );
    }

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
        assert_eq!(is_withdraw_royalty(time, time, 2), Ok(true));
        assert_eq!(is_withdraw_royalty(time, time + 40000, 2), Ok(true));
        assert_eq!(is_withdraw_royalty(time, time - 1 * day, 2), Ok(true));
        assert_eq!(is_withdraw_royalty(time, time - 1 * day + 100, 2), Ok(true));
        assert_eq!(is_withdraw_royalty(time, time - 2 * day, 2), Ok(false));
        assert_eq!(is_withdraw_royalty(time, time - 31 * day, 30), Ok(false));
    }

    #[test]
    pub fn test_get_epoch_by_time() {
        assert_eq!(get_epoch_by_time(time, time, 10), Ok((1, true)));
        assert_eq!(get_epoch_by_time(time, time + 5 * day, 10), Ok((1, false)));
        assert_eq!(get_epoch_by_time(time, time + 10 * day, 10), Ok((2, true)));
        assert_eq!(get_epoch_by_time(time, time + 15 * day, 10), Ok((2, false)));
        assert_eq!(get_epoch_by_time(time, time + 20 * day, 10), Ok((3, true)));
        assert_eq!(
            get_epoch_by_time(time, time + 299 * day, 10),
            Ok((30, false))
        );
        assert_eq!(
            get_epoch_by_time(time, time + 300 * day, 10),
            Ok((31, true))
        );
        assert_eq!(
            get_epoch_by_time(time, time + 301 * day, 10),
            Ok((31, false))
        );
        assert_eq!(
            get_epoch_by_time(time, time + 3000 * day, 10),
            Ok((301, true))
        );
        assert_eq!(
            get_epoch_by_time(time, time + 3001 * day, 10),
            Ok((301, false))
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
