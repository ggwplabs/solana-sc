use super::utils::get_or_create_token_account;
use crate::commands;
use anchor_client::anchor_lang::system_program;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::ClientError;
use anchor_client::{solana_sdk::pubkey::Pubkey, Client, Program};
use clap::value_t_or_exit;
use clap::{ArgMatches, Error};
use spl_token::ui_amount_to_amount;
use staking::state::{StakingInfo, UserInfo, STAKING_FUND_AUTH_SEED};

pub fn handle(
    cmd_matches: &ArgMatches,
    client: &Client,
    staking_program_id: Pubkey,
) -> Result<(), Error> {
    let staking_program = client.program(staking_program_id);
    let (sub_command, arg_matches) = cmd_matches.subcommand();
    match (sub_command, arg_matches) {
        (commands::staking::CMD_INITIALIZE, Some(arg_matches)) => {
            let update_auth = value_t_or_exit!(arg_matches, "update_auth", Pubkey);
            let ggwp_token = value_t_or_exit!(arg_matches, "ggwp_token", Pubkey);
            let staking_fund = value_t_or_exit!(arg_matches, "staking_fund", Pubkey);
            let accumulative_fund = value_t_or_exit!(arg_matches, "accumulative_fund", Pubkey);
            let epoch_period_days = value_t_or_exit!(arg_matches, "epoch_period_days", u16);
            let min_stake_amount = value_t_or_exit!(arg_matches, "min_stake_amount", f64);
            let min_stake_amount = ui_amount_to_amount(min_stake_amount, 9);
            let hold_period_days = value_t_or_exit!(arg_matches, "hold_period_days", u16);
            let hold_royalty = value_t_or_exit!(arg_matches, "hold_royalty", u8);
            let royalty = value_t_or_exit!(arg_matches, "royalty", u8);
            let apr_start = value_t_or_exit!(arg_matches, "apr_start", u8);
            let apr_step = value_t_or_exit!(arg_matches, "apr_step", u8);
            let apr_end = value_t_or_exit!(arg_matches, "apr_end", u8);

            cmd_initialize(
                staking_program,
                update_auth,
                ggwp_token,
                staking_fund,
                accumulative_fund,
                epoch_period_days,
                min_stake_amount,
                hold_period_days,
                hold_royalty,
                royalty,
                apr_start,
                apr_step,
                apr_end,
            )
            .expect("Initialize error");

            println!("Successful");
            Ok(())
        }

        (commands::staking::CMD_UPDATE_ADMIN, Some(arg_matches)) => {
            let staking_info = value_t_or_exit!(arg_matches, "staking_info", Pubkey);
            let admin = value_t_or_exit!(arg_matches, "admin", Pubkey);
            cmd_update_admin(staking_program, staking_info, admin).expect("Update admin error");

            println!("Successful");
            Ok(())
        }

        (commands::staking::CMD_SET_UPDATE_AUTHORITY, Some(arg_matches)) => {
            let staking_info = value_t_or_exit!(arg_matches, "staking_info", Pubkey);
            let update_auth = value_t_or_exit!(arg_matches, "update_auth", Pubkey);
            cmd_set_update_authority(staking_program, staking_info, update_auth)
                .expect("Set update auth error");

            println!("Successful");
            Ok(())
        }

        (commands::staking::CMD_UPDATE_EPOCH_PERIOD_DAYS, Some(arg_matches)) => {
            let staking_info = value_t_or_exit!(arg_matches, "staking_info", Pubkey);
            let epoch_period_days = value_t_or_exit!(arg_matches, "epoch_period_days", u16);
            cmd_update_epoch_period_days(staking_program, staking_info, epoch_period_days)
                .expect("Update epoch period days error");

            println!("Successful");
            Ok(())
        }

        (commands::staking::CMD_UPDATE_MIN_STAKE_AMOUNT, Some(arg_matches)) => {
            let staking_info = value_t_or_exit!(arg_matches, "staking_info", Pubkey);
            let amount = value_t_or_exit!(arg_matches, "amount", f64);
            let amount = ui_amount_to_amount(amount, 9);
            cmd_update_min_stake_amount(staking_program, staking_info, amount)
                .expect("Update min stake amount error");

            println!("Successful");
            Ok(())
        }

        (commands::staking::CMD_UPDATE_HOLD_PERIOD_DAYS, Some(arg_matches)) => {
            let staking_info = value_t_or_exit!(arg_matches, "staking_info", Pubkey);
            let hold_period_days = value_t_or_exit!(arg_matches, "hold_period_days", u16);
            cmd_update_hold_period_days(staking_program, staking_info, hold_period_days)
                .expect("Update hold period days");

            println!("Successful");
            Ok(())
        }

        (commands::staking::CMD_UPDATE_HOLD_ROYALTY, Some(arg_matches)) => {
            let staking_info = value_t_or_exit!(arg_matches, "staking_info", Pubkey);
            let hold_royalty = value_t_or_exit!(arg_matches, "hold_royalty", u8);
            cmd_update_hold_royalty(staking_program, staking_info, hold_royalty)
                .expect("Update hold royalty error");

            println!("Successful");
            Ok(())
        }

        (commands::staking::CMD_UPDATE_ROYALTY, Some(arg_matches)) => {
            let staking_info = value_t_or_exit!(arg_matches, "staking_info", Pubkey);
            let royalty = value_t_or_exit!(arg_matches, "royalty", u8);
            cmd_update_royalty(staking_program, staking_info, royalty)
                .expect("Update royalty error");

            println!("Successful");
            Ok(())
        }

        (commands::staking::CMD_STAKE, Some(arg_matches)) => {
            let staking_info = value_t_or_exit!(arg_matches, "staking_info", Pubkey);
            let amount = value_t_or_exit!(arg_matches, "amount", f64);
            let amount = ui_amount_to_amount(amount, 9);
            cmd_stake(staking_program, staking_info, amount).expect("Stake error");

            println!("Successful");
            Ok(())
        }

        (commands::staking::CMD_WITHDRAW, Some(arg_matches)) => {
            let staking_info = value_t_or_exit!(arg_matches, "staking_info", Pubkey);
            cmd_withdraw(staking_program, staking_info).expect("Withdraw error");

            println!("Successful");
            Ok(())
        }

        (commands::staking::CMD_SHOW_STAKING_INFO, Some(arg_matches)) => {
            let staking_info = value_t_or_exit!(arg_matches, "staking_info", Pubkey);
            let staking_info_data: StakingInfo = staking_program
                .account(staking_info)
                .expect("Error getting staking info data");
            println!("Staking info data: {:?}", staking_info_data);
            Ok(())
        }

        (commands::staking::CMD_SHOW_USER_INFO, Some(arg_matches)) => {
            let staking_info = value_t_or_exit!(arg_matches, "staking_info", Pubkey);
            let (user_info, _) = Pubkey::find_program_address(
                &[
                    staking::state::USER_INFO_SEED.as_bytes(),
                    staking_info.as_ref(),
                    staking_program.payer().as_ref(),
                ],
                &staking_program.id(),
            );
            let user_info_data: UserInfo = staking_program
                .account(user_info)
                .expect("Getting user info error");
            println!("User info data: {:?}", user_info_data);
            Ok(())
        }

        _ => {
            println!("{}", cmd_matches.usage());
            Ok(())
        }
    }
}

pub fn cmd_initialize(
    staking_program: Program,
    update_auth: Pubkey,
    ggwp_token: Pubkey,
    staking_fund: Pubkey,
    accumulative_fund: Pubkey,
    epoch_period_days: u16,
    min_stake_amount: u64,
    hold_period_days: u16,
    hold_royalty: u8,
    royalty: u8,
    apr_start: u8,
    apr_step: u8,
    apr_end: u8,
) -> Result<(), ClientError> {
    let staking_info = Keypair::new();
    println!("New staking info PK: {}", staking_info.pubkey());

    let (staking_fund_auth, _) = Pubkey::find_program_address(
        &[
            STAKING_FUND_AUTH_SEED.as_bytes(),
            staking_info.pubkey().as_ref(),
        ],
        &staking_program.id(),
    );

    let (staking_treasury_auth, _) = Pubkey::find_program_address(
        &[
            staking::state::TREASURY_AUTH_SEED.as_bytes(),
            staking_info.pubkey().as_ref(),
        ],
        &staking_program.id(),
    );

    let staking_treasury =
        get_or_create_token_account(&staking_program, ggwp_token, staking_treasury_auth)?;

    staking_program
        .request()
        .accounts(staking::accounts::Initialize {
            admin: staking_program.payer(),
            staking_info: staking_info.pubkey(),
            ggwp_token: ggwp_token,
            accumulative_fund: accumulative_fund,
            treasury: staking_treasury,
            treasury_auth: staking_treasury_auth,
            staking_fund: staking_fund,
            staking_fund_auth: staking_fund_auth,
            system_program: system_program::ID,
        })
        .args(staking::instruction::Initialize {
            update_auth: update_auth,
            epoch_period_days: epoch_period_days,
            min_stake_amount: min_stake_amount,
            hold_period_days: hold_period_days,
            hold_royalty: hold_royalty,
            royalty: royalty,
            apr_start: apr_start,
            apr_step: apr_step,
            apr_end: apr_end,
        })
        .signer(&staking_info)
        .send()?;

    Ok(())
}

pub fn cmd_update_admin(
    staking_program: Program,
    staking_info: Pubkey,
    admin: Pubkey,
) -> Result<(), ClientError> {
    staking_program
        .request()
        .accounts(staking::accounts::UpdateParam {
            authority: staking_program.payer(),
            staking_info: staking_info,
        })
        .args(staking::instruction::UpdateAdmin { admin: admin })
        .send()?;

    Ok(())
}

pub fn cmd_set_update_authority(
    staking_program: Program,
    staking_info: Pubkey,
    update_auth: Pubkey,
) -> Result<(), ClientError> {
    staking_program
        .request()
        .accounts(staking::accounts::UpdateParam {
            authority: staking_program.payer(),
            staking_info: staking_info,
        })
        .args(staking::instruction::SetUpdateAuthority {
            update_auth: update_auth,
        })
        .send()?;

    Ok(())
}

pub fn cmd_update_epoch_period_days(
    staking_program: Program,
    staking_info: Pubkey,
    epoch_period_days: u16,
) -> Result<(), ClientError> {
    staking_program
        .request()
        .accounts(staking::accounts::UpdateParam {
            authority: staking_program.payer(),
            staking_info: staking_info,
        })
        .args(staking::instruction::UpdateEpochPeriodDays {
            epoch_period_days: epoch_period_days,
        })
        .send()?;

    Ok(())
}

pub fn cmd_update_min_stake_amount(
    staking_program: Program,
    staking_info: Pubkey,
    min_stake_amount: u64,
) -> Result<(), ClientError> {
    staking_program
        .request()
        .accounts(staking::accounts::UpdateParam {
            authority: staking_program.payer(),
            staking_info: staking_info,
        })
        .args(staking::instruction::UpdateMinStakeAmount {
            min_stake_amount: min_stake_amount,
        })
        .send()?;

    Ok(())
}

pub fn cmd_update_hold_period_days(
    staking_program: Program,
    staking_info: Pubkey,
    hold_period_days: u16,
) -> Result<(), ClientError> {
    staking_program
        .request()
        .accounts(staking::accounts::UpdateParam {
            authority: staking_program.payer(),
            staking_info: staking_info,
        })
        .args(staking::instruction::UpdateHoldPeriodDays {
            hold_period_days: hold_period_days,
        })
        .send()?;

    Ok(())
}

pub fn cmd_update_hold_royalty(
    staking_program: Program,
    staking_info: Pubkey,
    hold_royalty: u8,
) -> Result<(), ClientError> {
    staking_program
        .request()
        .accounts(staking::accounts::UpdateParam {
            authority: staking_program.payer(),
            staking_info: staking_info,
        })
        .args(staking::instruction::UpdateHoldRoyalty {
            hold_royalty: hold_royalty,
        })
        .send()?;

    Ok(())
}

pub fn cmd_update_royalty(
    staking_program: Program,
    staking_info: Pubkey,
    royalty: u8,
) -> Result<(), ClientError> {
    staking_program
        .request()
        .accounts(staking::accounts::UpdateParam {
            authority: staking_program.payer(),
            staking_info: staking_info,
        })
        .args(staking::instruction::UpdateRoyalty { royalty: royalty })
        .send()?;

    Ok(())
}

pub fn cmd_stake(
    staking_program: Program,
    staking_info: Pubkey,
    amount: u64,
) -> Result<(), ClientError> {
    let staking_info_data: StakingInfo = staking_program.account(staking_info)?;

    let (user_info, _) = Pubkey::find_program_address(
        &[
            staking::state::USER_INFO_SEED.as_bytes(),
            staking_info.as_ref(),
            staking_program.payer().as_ref(),
        ],
        &staking_program.id(),
    );

    let user_ggwp_wallet = get_or_create_token_account(
        &staking_program,
        staking_info_data.ggwp_token,
        staking_program.payer(),
    )?;

    staking_program
        .request()
        .accounts(staking::accounts::Stake {
            user: staking_program.payer(),
            user_info: user_info,
            user_ggwp_wallet: user_ggwp_wallet,
            staking_info: staking_info,
            treasury: staking_info_data.treasury,
            accumulative_fund: staking_info_data.accumulative_fund,
            system_program: system_program::ID,
            token_program: spl_token::id(),
        })
        .args(staking::instruction::Stake { amount: amount })
        .send()?;

    Ok(())
}

pub fn cmd_withdraw(staking_program: Program, staking_info: Pubkey) -> Result<(), ClientError> {
    let staking_info_data: StakingInfo = staking_program.account(staking_info)?;

    let (user_info, _) = Pubkey::find_program_address(
        &[
            staking::state::USER_INFO_SEED.as_bytes(),
            staking_info.as_ref(),
            staking_program.payer().as_ref(),
        ],
        &staking_program.id(),
    );

    let user_ggwp_wallet = get_or_create_token_account(
        &staking_program,
        staking_info_data.ggwp_token,
        staking_program.payer(),
    )?;

    let (treasury_auth, _) = Pubkey::find_program_address(
        &[
            staking::state::TREASURY_AUTH_SEED.as_bytes(),
            staking_info.as_ref(),
        ],
        &staking_program.id(),
    );

    let (staking_fund_auth, _) = Pubkey::find_program_address(
        &[
            staking::state::STAKING_FUND_AUTH_SEED.as_bytes(),
            staking_info.as_ref(),
        ],
        &staking_program.id(),
    );

    staking_program
        .request()
        .accounts(staking::accounts::Withdraw {
            user: staking_program.payer(),
            user_info: user_info,
            user_ggwp_wallet: user_ggwp_wallet,
            staking_info: staking_info,
            treasury: staking_info_data.treasury,
            treasury_auth: treasury_auth,
            accumulative_fund: staking_info_data.accumulative_fund,
            staking_fund: staking_info_data.staking_fund,
            staking_fund_auth: staking_fund_auth,
            system_program: system_program::ID,
            token_program: spl_token::id(),
        })
        .args(staking::instruction::Withdraw {})
        .send()?;

    Ok(())
}
