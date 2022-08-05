use anchor_lang::{prelude::*, solana_program::clock::UnixTimestamp};

use crate::error::FreezingError;

pub fn calc_earned_gpass(clock: &Clock, last_getting_gpass: UnixTimestamp) -> Result<u64> {
    // todo
    Ok(0)
}

pub fn is_withdraw_royalty(clock: &Clock, freezed_time: UnixTimestamp) -> Result<bool> {
    let current_time = clock.unix_timestamp;
    let spent_time = current_time.checked_sub(freezed_time).ok_or(FreezingError::Overflow)?;
    if spent_time >= 1296000 {
        return Ok(false);
    }
    Ok(true)
}
