use anchor_lang::prelude::*;

pub fn calc_reward_amount(
    play_to_earn_fund_amount: u64,
    freezed_users: u64,
    reward_coefficient: u32,
    gpass_daily_reward: u64,
    gpass_daily_reward_coefficient: u32,
) -> Result<u64> {

    Ok(0)
}

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
    #[test]
    pub fn test_calc_reward_amount() {

    }
}