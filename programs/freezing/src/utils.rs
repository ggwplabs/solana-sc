use crate::{
    error::FreezingError,
    state::{RewardTableRow, MAX_REWARDS_TABLE_ROWS},
};
use anchor_lang::{prelude::*, solana_program::clock::UnixTimestamp};
use anchor_spl::token::spl_token::{amount_to_ui_amount, ui_amount_to_amount};

/// Checks reward table valid.
pub fn is_reward_table_valid(reward_table: &Vec<RewardTableRow>) -> Result<bool> {
    if reward_table.is_empty() {
        return Ok(false);
    }
    if reward_table.len() > MAX_REWARDS_TABLE_ROWS {
        return Ok(false);
    }

    for i in 0..reward_table.len() {
        if reward_table[i].ggwp_amount == 0 || reward_table[i].gpass_amount == 0 {
            return Ok(false);
        }
        if i > 0 {
            for j in 0..i {
                if reward_table[i].ggwp_amount <= reward_table[j].ggwp_amount {
                    return Ok(false);
                }
                if reward_table[i].gpass_amount <= reward_table[j].gpass_amount {
                    return Ok(false);
                }
            }
        }
    }

    Ok(true)
}

/// Checks reward table and get the GPASS reward amount.
pub fn earned_gpass_immediately(
    reward_table: &Vec<RewardTableRow>,
    user_ggwp_amount: u64,
) -> Result<u64> {
    let mut earned_gpass = 0;
    for row in reward_table {
        if user_ggwp_amount >= row.ggwp_amount {
            earned_gpass = row.gpass_amount;
        } else {
            break;
        }
    }

    Ok(earned_gpass)
}

pub fn calc_earned_gpass(
    reward_table: &Vec<RewardTableRow>,
    user_ggwp_amount: u64,
    current_time: UnixTimestamp,
    last_getting_gpass: UnixTimestamp,
    reward_period: UnixTimestamp,
) -> Result<u64> {
    let spent_time = current_time
        .checked_sub(last_getting_gpass)
        .ok_or(FreezingError::Overflow)?;
    msg!("Spent time: {}", spent_time);
    if spent_time < reward_period {
        msg!("Reward period is not passed yet.");
        return Ok(0);
    }

    let reward_periods_spent = spent_time
        .checked_div(reward_period)
        .ok_or(FreezingError::Overflow)? as u64;
    let earned_gpass = earned_gpass_immediately(reward_table, user_ggwp_amount)?;
    Ok(earned_gpass
        .checked_mul(reward_periods_spent)
        .ok_or(FreezingError::Overflow)?)
}

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
        .ok_or(FreezingError::Overflow)?;
    if spent_time >= unfreeze_lock_period {
        return Ok(false);
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_is_reward_table_valid() {
        assert_eq!(is_reward_table_valid(&vec![]), Ok(false));
        assert_eq!(
            is_reward_table_valid(&vec![RewardTableRow {
                ggwp_amount: 0,
                gpass_amount: 0,
            }]),
            Ok(false)
        );
        assert_eq!(
            is_reward_table_valid(&vec![RewardTableRow {
                ggwp_amount: 1000,
                gpass_amount: 0,
            }]),
            Ok(false)
        );
        assert_eq!(
            is_reward_table_valid(&vec![RewardTableRow {
                ggwp_amount: 0,
                gpass_amount: 5,
            }]),
            Ok(false)
        );
        assert_eq!(
            is_reward_table_valid(&vec![
                RewardTableRow {
                    ggwp_amount: 1000,
                    gpass_amount: 5,
                },
                RewardTableRow {
                    ggwp_amount: 1000,
                    gpass_amount: 10,
                },
                RewardTableRow {
                    ggwp_amount: 3000,
                    gpass_amount: 15,
                }
            ]),
            Ok(false)
        );
        assert_eq!(
            is_reward_table_valid(&vec![
                RewardTableRow {
                    ggwp_amount: 1000,
                    gpass_amount: 5,
                },
                RewardTableRow {
                    ggwp_amount: 2000,
                    gpass_amount: 10,
                },
                RewardTableRow {
                    ggwp_amount: 500,
                    gpass_amount: 15,
                }
            ]),
            Ok(false)
        );
        assert_eq!(
            is_reward_table_valid(&vec![
                RewardTableRow {
                    ggwp_amount: 1000,
                    gpass_amount: 5,
                },
                RewardTableRow {
                    ggwp_amount: 2000,
                    gpass_amount: 5,
                },
                RewardTableRow {
                    ggwp_amount: 3000,
                    gpass_amount: 15,
                }
            ]),
            Ok(false)
        );
        assert_eq!(
            is_reward_table_valid(&vec![
                RewardTableRow {
                    ggwp_amount: 1000,
                    gpass_amount: 5,
                },
                RewardTableRow {
                    ggwp_amount: 2000,
                    gpass_amount: 10,
                },
                RewardTableRow {
                    ggwp_amount: 3000,
                    gpass_amount: 2,
                }
            ]),
            Ok(false)
        );
        assert_eq!(
            is_reward_table_valid(&vec![
                RewardTableRow {
                    ggwp_amount: 1000,
                    gpass_amount: 5,
                },
                RewardTableRow {
                    ggwp_amount: 2000,
                    gpass_amount: 10,
                },
                RewardTableRow {
                    ggwp_amount: 2000,
                    gpass_amount: 10,
                }
            ]),
            Ok(false)
        );
        assert_eq!(
            is_reward_table_valid(&vec![RewardTableRow {
                ggwp_amount: 1000,
                gpass_amount: 5,
            }]),
            Ok(true)
        );
        assert_eq!(
            is_reward_table_valid(&vec![
                RewardTableRow {
                    ggwp_amount: 1000,
                    gpass_amount: 5,
                },
                RewardTableRow {
                    ggwp_amount: 2000,
                    gpass_amount: 10,
                },
                RewardTableRow {
                    ggwp_amount: 3000,
                    gpass_amount: 15,
                }
            ]),
            Ok(true)
        );
    }

    #[test]
    pub fn test_earned_gpass_immediately() {
        assert_eq!(earned_gpass_immediately(&vec![], 0), Ok(0));
        assert_eq!(
            earned_gpass_immediately(
                &vec![RewardTableRow {
                    ggwp_amount: 1000,
                    gpass_amount: 5,
                }],
                500
            ),
            Ok(0)
        );
        assert_eq!(
            earned_gpass_immediately(
                &vec![RewardTableRow {
                    ggwp_amount: 1000,
                    gpass_amount: 5,
                }],
                1000
            ),
            Ok(5)
        );
        assert_eq!(
            earned_gpass_immediately(
                &vec![
                    RewardTableRow {
                        ggwp_amount: 1000,
                        gpass_amount: 5,
                    },
                    RewardTableRow {
                        ggwp_amount: 2000,
                        gpass_amount: 10,
                    }
                ],
                1500
            ),
            Ok(5)
        );
        assert_eq!(
            earned_gpass_immediately(
                &vec![
                    RewardTableRow {
                        ggwp_amount: 1000,
                        gpass_amount: 5,
                    },
                    RewardTableRow {
                        ggwp_amount: 2000,
                        gpass_amount: 10,
                    }
                ],
                2000
            ),
            Ok(10)
        );
        assert_eq!(
            earned_gpass_immediately(
                &vec![
                    RewardTableRow {
                        ggwp_amount: 1000,
                        gpass_amount: 5,
                    },
                    RewardTableRow {
                        ggwp_amount: 2000,
                        gpass_amount: 10,
                    }
                ],
                3000
            ),
            Ok(10)
        );
    }

    #[test]
    pub fn test_calc_earned_gpass() {
        let reward_table = vec![
            RewardTableRow {
                ggwp_amount: 1000,
                gpass_amount: 5,
            },
            RewardTableRow {
                ggwp_amount: 2000,
                gpass_amount: 10,
            },
            RewardTableRow {
                ggwp_amount: 3000,
                gpass_amount: 15,
            },
        ];
        let current_time = 1660032700;
        let reward_period = 100;

        assert_eq!(
            calc_earned_gpass(&reward_table, 1000, current_time, 1660032700, reward_period),
            Ok(0)
        );
        assert_eq!(
            calc_earned_gpass(&reward_table, 1000, current_time, 1660032750, reward_period),
            Ok(0)
        );
        assert_eq!(
            calc_earned_gpass(&reward_table, 1000, current_time, 1660032800, reward_period,),
            Ok(0)
        );

        assert_eq!(
            calc_earned_gpass(&reward_table, 1000, current_time, 1660032600, reward_period,),
            Ok(5)
        );
        assert_eq!(
            calc_earned_gpass(&reward_table, 1000, current_time, 1660032500, reward_period,),
            Ok(10)
        );

        assert_eq!(
            calc_earned_gpass(&reward_table, 3000, current_time, 1660032450, reward_period,),
            Ok(30)
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
        assert_eq!(is_withdraw_royalty(1660032700, 1660032700, 100), Ok(true));
        assert_eq!(is_withdraw_royalty(1660032700, 1660032650, 100), Ok(true));
        assert_eq!(is_withdraw_royalty(1660032700, 1660032800, 100), Ok(true));
        assert_eq!(is_withdraw_royalty(1660032700, 1660032500, 100), Ok(false));
        assert_eq!(is_withdraw_royalty(1660032700, 1660032300, 100), Ok(false));
    }
}
