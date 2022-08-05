use crate::error::GpassError;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::UnixTimestamp;

pub fn time_passed(current_timestamp: UnixTimestamp, last_burned: UnixTimestamp) -> Result<u64> {
    if last_burned <= 0 {
        return Err(GpassError::InvalidLastBurnedValue.into());
    }

    let current_timestamp = current_timestamp as u64;
    let last_burned = last_burned as u64;

    let time_passed = current_timestamp
        .checked_sub(last_burned)
        .ok_or(GpassError::Overflow)?;

    Ok(time_passed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_time_passed() {
        assert_eq!(
            time_passed(0, 0),
            Err(GpassError::InvalidLastBurnedValue.into())
        );
        assert_eq!(
            time_passed(10000, 0),
            Err(GpassError::InvalidLastBurnedValue.into())
        );
        assert_eq!(time_passed(1000, 1001), Err(GpassError::Overflow.into()));
        assert_eq!(time_passed(1000, 500), Ok(500));
    }
}
