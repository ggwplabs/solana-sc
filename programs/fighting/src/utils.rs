use anchor_lang::prelude::*;
use anchor_spl::token::spl_token::{amount_to_ui_amount, ui_amount_to_amount};

/// Calculate reward amount for user
pub fn calc_reward_amount(
    play_to_earn_fund_amount: u64,
    freezed_users: u64,
    reward_coefficient: u32,
    gpass_daily_reward: u64,
    gpass_daily_reward_coefficient: u32,
) -> Result<u64> {
    let play_to_earn_fund_amount = amount_to_ui_amount(play_to_earn_fund_amount, 9);
    let mut reward_amount =
        play_to_earn_fund_amount / (freezed_users * reward_coefficient as u64) as f64;

    if reward_amount > gpass_daily_reward as f64 {
        reward_amount = gpass_daily_reward as f64 / gpass_daily_reward_coefficient as f64;
    }

    Ok(ui_amount_to_amount(reward_amount, 9))
}

/// Calcutale share
pub fn calc_share_amount(share: u8, amount: u64) -> Result<u64> {
    let ui_amount = anchor_spl::token::spl_token::amount_to_ui_amount(amount, 9);
    let share_amount = ui_amount / 100.0 * share as f64;
    Ok(anchor_spl::token::spl_token::ui_amount_to_amount(
        share_amount,
        9,
    ))
}

#[cfg(test)]
mod test {
    use super::{calc_reward_amount, calc_share_amount};

    #[test]
    pub fn test_calc_share_amount() {
        assert_eq!(calc_share_amount(8, 100_000_000_000), Ok(8_000_000_000));
        assert_eq!(calc_share_amount(10, 100_000_000_000), Ok(10_000_000_000));
        assert_eq!(calc_share_amount(50, 123_000_000_000), Ok(61_500_000_000));
        assert_eq!(calc_share_amount(10, 0), Ok(0));
    }

    #[test]
    pub fn test_calc_reward_amount() {
        // If daily gpass reward bigger than reward
        assert_eq!(calc_reward_amount(0, 10, 2, 100, 10), Ok(0));
        assert_eq!(calc_reward_amount(0, 0, 2, 100, 10), Ok(0));
        assert_eq!(
            calc_reward_amount(10_000_000_000, 2, 20000, 100, 10),
            Ok(250_000) // 0.00025 GGWP
        );
        assert_eq!(
            calc_reward_amount(123_000_000_000, 10, 2, 100, 10),
            Ok(6_150_000_000) // 6.150 GGWP
        );

        // If daily gpass reward less than reward
        assert_eq!(
            calc_reward_amount(10_000_000_000, 0, 20000, 100, 10),
            Ok(10_000_000_000) // 10.0 GGWP: 100/10
        );
        assert_eq!(
            calc_reward_amount(123_000_000_000, 10, 2, 5, 10),
            Ok(500_000_000) // 0.5 GGWP: 5/10
        );
    }
}
