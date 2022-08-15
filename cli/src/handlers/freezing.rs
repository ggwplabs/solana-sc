use super::utils::get_or_create_token_account;
use crate::commands;
use anchor_client::anchor_lang::system_program;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::ClientError;
use anchor_client::{solana_sdk::pubkey::Pubkey, Client, Program};
use clap::{value_t_or_exit, values_t_or_exit};
use clap::{ArgMatches, Error};
use freezing::state::{
    FreezingParams, RewardTableRow, UserInfo, GPASS_MINT_AUTH_SEED, TREASURY_AUTH_SEED,
};
use spl_token::ui_amount_to_amount;

pub fn handle(
    cmd_matches: &ArgMatches,
    client: &Client,
    freezing_program_id: Pubkey,
    gpass_program_id: Pubkey,
) -> Result<(), Error> {
    let program = client.program(freezing_program_id);
    let (sub_command, arg_matches) = cmd_matches.subcommand();
    match (sub_command, arg_matches) {
        (commands::freezing::CMD_INITIALIZE, Some(arg_matches)) => {
            println!("Commad initialize");
            let update_auth = value_t_or_exit!(arg_matches, "update_auth", Pubkey);
            let ggwp_token = value_t_or_exit!(arg_matches, "ggwp_token", Pubkey);
            let gpass_settings = value_t_or_exit!(arg_matches, "gpass_settings", Pubkey);
            let accumulative_fund = value_t_or_exit!(arg_matches, "accumulative_fund", Pubkey);
            let reward_period = value_t_or_exit!(arg_matches, "reward_period", i64);
            let royalty = value_t_or_exit!(arg_matches, "royalty", u8);
            let unfreeze_royalty = value_t_or_exit!(arg_matches, "unfreeze_royalty", u8);
            let unfreeze_lock_period = value_t_or_exit!(arg_matches, "unfreeze_lock_period", i64);

            let reward_table_ggwp = values_t_or_exit!(arg_matches, "reward_table_ggwp", u64);
            let reward_table_gpass = values_t_or_exit!(arg_matches, "reward_table_gpass", u64);
            assert_eq!(reward_table_ggwp.len(), reward_table_gpass.len());
            let reward_table = reward_table_ggwp
                .iter()
                .zip(reward_table_gpass.iter())
                .map(|v| RewardTableRow {
                    ggwp_amount: *v.0,
                    gpass_amount: *v.1,
                })
                .collect();

            cmd_initialize(
                &program,
                update_auth,
                ggwp_token,
                gpass_settings,
                accumulative_fund,
                reward_period,
                royalty,
                unfreeze_royalty,
                unfreeze_lock_period,
                reward_table,
            )
            .expect("Initialize error");

            println!("Successful");
            Ok(())
        }

        (commands::freezing::CMD_UPDATE_ADMIN, Some(arg_matches)) => {
            let params = value_t_or_exit!(arg_matches, "params", Pubkey);
            let admin = value_t_or_exit!(arg_matches, "admin", Pubkey);
            cmd_update_admin(&program, params, admin).expect("Update admin error");

            println!("Successful");
            Ok(())
        }

        (commands::freezing::CMD_SET_UPDATE_AUTHORITY, Some(arg_matches)) => {
            let params = value_t_or_exit!(arg_matches, "params", Pubkey);
            let update_authority = value_t_or_exit!(arg_matches, "update_authority", Pubkey);
            cmd_set_update_authority(&program, params, update_authority)
                .expect("Set update authority error");

            println!("Successful");
            Ok(())
        }

        (commands::freezing::CMD_UPDATE_ROYALTY, Some(arg_matches)) => {
            let params = value_t_or_exit!(arg_matches, "params", Pubkey);
            let royalty = value_t_or_exit!(arg_matches, "royalty", u8);
            cmd_update_royalty(&program, params, royalty).expect("Update royalty error");

            println!("Successful");
            Ok(())
        }

        (commands::freezing::CMD_UPDATE_UNFREEZE_ROYALTY, Some(arg_matches)) => {
            let params = value_t_or_exit!(arg_matches, "params", Pubkey);
            let unfreeze_royalty = value_t_or_exit!(arg_matches, "unfreeze_royalty", u8);
            cmd_update_unfreeze_royalty(&program, params, unfreeze_royalty)
                .expect("Update unfreeze royalty error");

            println!("Successful");
            Ok(())
        }

        (commands::freezing::CMD_UPDATE_UNFREEZE_LOCK_PERIOD, Some(arg_matches)) => {
            let params = value_t_or_exit!(arg_matches, "params", Pubkey);
            let unfreeze_lock_period = value_t_or_exit!(arg_matches, "unfreeze_lock_period", i64);
            cmd_update_unfreeze_lock_period(&program, params, unfreeze_lock_period)
                .expect("Update unfreeze lock period error");

            println!("Successful");
            Ok(())
        }

        (commands::freezing::CMD_UPDATE_REWARD_PERIOD, Some(arg_matches)) => {
            let params = value_t_or_exit!(arg_matches, "params", Pubkey);
            let reward_period = value_t_or_exit!(arg_matches, "reward_period", i64);
            cmd_update_reward_period(&program, params, reward_period)
                .expect("Update reward period error");

            println!("Successful");
            Ok(())
        }

        (commands::freezing::CMD_UPDATE_REWARD_TABLE, Some(arg_matches)) => {
            let params = value_t_or_exit!(arg_matches, "params", Pubkey);

            let reward_table_ggwp = values_t_or_exit!(arg_matches, "reward_table_ggwp", u64);
            let reward_table_gpass = values_t_or_exit!(arg_matches, "reward_table_gpass", u64);
            assert_eq!(reward_table_ggwp.len(), reward_table_gpass.len());
            let reward_table = reward_table_ggwp
                .iter()
                .zip(reward_table_gpass.iter())
                .map(|v| RewardTableRow {
                    ggwp_amount: *v.0,
                    gpass_amount: *v.1,
                })
                .collect();

            cmd_update_reward_table(&program, params, reward_table)
                .expect("Update reward table error");

            println!("Successful");
            Ok(())
        }

        (commands::freezing::CMD_FREEZE, Some(arg_matches)) => {
            let params = value_t_or_exit!(arg_matches, "params", Pubkey);
            let amount = value_t_or_exit!(arg_matches, "amount", f64);
            let amount = ui_amount_to_amount(amount, 9);
            cmd_freeze(&program, gpass_program_id, params, amount).expect("Freeze error");

            println!("Successful");
            Ok(())
        }

        (commands::freezing::CMD_WITHDRAW_GPASS, Some(arg_matches)) => {
            let params = value_t_or_exit!(arg_matches, "params", Pubkey);
            cmd_withdraw_gpass(&program, gpass_program_id, params).expect("Withdraw gpass error");

            println!("Successful");
            Ok(())
        }

        (commands::freezing::CMD_UNFREEZE, Some(arg_matches)) => {
            let params = value_t_or_exit!(arg_matches, "params", Pubkey);
            cmd_unfreeze(&program, gpass_program_id, params).expect("Unfreeze error");

            println!("Successful");
            Ok(())
        }

        (commands::freezing::CMD_SHOW_PARAMS, Some(arg_matches)) => {
            let params = value_t_or_exit!(arg_matches, "params", Pubkey);
            let params_data: FreezingParams = program.account(params).expect("Get params error");
            println!("Freezing params data: {:?}", params_data);
            Ok(())
        }

        (commands::freezing::CMD_SHOW_USER_INFO, Some(arg_matches)) => {
            let params = value_t_or_exit!(arg_matches, "params", Pubkey);
            let user = value_t_or_exit!(arg_matches, "user", Pubkey);
            let (user_info, _bump) = Pubkey::find_program_address(
                &[
                    freezing::state::USER_INFO_SEED.as_bytes(),
                    params.as_ref(),
                    user.as_ref(),
                ],
                &program.id(),
            );
            println!("User info address: {:?}", user_info);

            match program.account::<UserInfo>(user_info) {
                Ok(d) => {
                    println!("User info data: {:?}", d);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }

            Ok(())
        }

        _ => {
            println!("{}", cmd_matches.usage());
            Ok(())
        }
    }
}

fn cmd_initialize(
    program: &Program,
    update_auth: Pubkey,
    ggwp_token: Pubkey,
    gpass_settings: Pubkey,
    accumulative_fund: Pubkey,
    reward_period: i64,
    royalty: u8,
    unfreeze_royalty: u8,
    unfreeze_lock_period: i64,
    reward_table: Vec<RewardTableRow>,
) -> Result<(), ClientError> {
    let params = Keypair::new();
    println!("New Freezing params Pubkey: {}", params.pubkey());

    let (gpass_mint_auth, _) = Pubkey::find_program_address(
        &[
            GPASS_MINT_AUTH_SEED.as_bytes(),
            params.pubkey().as_ref(),
            gpass_settings.as_ref(),
        ],
        &program.id(),
    );
    println!("GPASS mint auth: {}", gpass_mint_auth);

    let (treasury_auth, _) = Pubkey::find_program_address(
        &[TREASURY_AUTH_SEED.as_bytes(), params.pubkey().as_ref()],
        &program.id(),
    );

    let treasury = get_or_create_token_account(program, ggwp_token, treasury_auth)?;

    program
        .request()
        .accounts(freezing::accounts::Initialize {
            admin: program.payer(),
            freezing_params: params.pubkey(),
            gpass_mint_auth: gpass_mint_auth,
            treasury_auth: treasury_auth,
            ggwp_token: ggwp_token,
            gpass_settings: gpass_settings,
            accumulative_fund: accumulative_fund,
            treasury: treasury,
            system_program: system_program::ID,
            token_program: spl_token::id(),
        })
        .args(freezing::instruction::Initialize {
            update_auth: update_auth,
            reward_period: reward_period,
            royalty: royalty,
            unfreeze_royalty: unfreeze_royalty,
            unfreeze_lock_period: unfreeze_lock_period,
            reward_table: reward_table,
        })
        .signer(&params)
        .send()?;

    Ok(())
}

fn cmd_update_admin(program: &Program, params: Pubkey, admin: Pubkey) -> Result<(), ClientError> {
    program
        .request()
        .accounts(freezing::accounts::UpdateParam {
            authority: program.payer(),
            freezing_params: params,
        })
        .args(freezing::instruction::UpdateAdmin { admin: admin })
        .send()?;

    Ok(())
}

fn cmd_set_update_authority(
    program: &Program,
    params: Pubkey,
    update_auth: Pubkey,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(freezing::accounts::UpdateParam {
            authority: program.payer(),
            freezing_params: params,
        })
        .args(freezing::instruction::SetUpdateAuthority {
            update_auth: update_auth,
        })
        .send()?;

    Ok(())
}

fn cmd_update_royalty(program: &Program, params: Pubkey, royalty: u8) -> Result<(), ClientError> {
    program
        .request()
        .accounts(freezing::accounts::UpdateParam {
            authority: program.payer(),
            freezing_params: params,
        })
        .args(freezing::instruction::UpdateRoyalty { royalty: royalty })
        .send()?;

    Ok(())
}

fn cmd_update_unfreeze_royalty(
    program: &Program,
    params: Pubkey,
    unfreeze_royalty: u8,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(freezing::accounts::UpdateParam {
            authority: program.payer(),
            freezing_params: params,
        })
        .args(freezing::instruction::UpdateUnfreezeRoyalty {
            unfreeze_royalty: unfreeze_royalty,
        })
        .send()?;

    Ok(())
}

fn cmd_update_unfreeze_lock_period(
    program: &Program,
    params: Pubkey,
    unfreeze_lock_period: i64,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(freezing::accounts::UpdateParam {
            authority: program.payer(),
            freezing_params: params,
        })
        .args(freezing::instruction::UpdateUnfreezeLockPeriod {
            unfreeze_lock_period: unfreeze_lock_period,
        })
        .send()?;

    Ok(())
}

fn cmd_update_reward_period(
    program: &Program,
    params: Pubkey,
    reward_period: i64,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(freezing::accounts::UpdateParam {
            authority: program.payer(),
            freezing_params: params,
        })
        .args(freezing::instruction::UpdateRewardPeriod {
            reward_period: reward_period,
        })
        .send()?;

    Ok(())
}

fn cmd_update_reward_table(
    program: &Program,
    params: Pubkey,
    reward_table: Vec<RewardTableRow>,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(freezing::accounts::UpdateParam {
            authority: program.payer(),
            freezing_params: params,
        })
        .args(freezing::instruction::UpdateRewardTable {
            reward_table: reward_table,
        })
        .send()?;

    Ok(())
}

fn cmd_freeze(
    freezing_program: &Program,
    gpass_program_id: Pubkey,
    params: Pubkey,
    amount: u64,
) -> Result<(), ClientError> {
    let freezing_params_data: FreezingParams = freezing_program.account(params)?;
    let (user_info, _) = Pubkey::find_program_address(
        &[
            freezing::state::USER_INFO_SEED.as_bytes(),
            params.as_ref(),
            freezing_program.payer().as_ref(),
        ],
        &freezing_program.id(),
    );

    let (gpass_mint_auth, _) = Pubkey::find_program_address(
        &[
            GPASS_MINT_AUTH_SEED.as_bytes(),
            params.as_ref(),
            freezing_params_data.gpass_settings.as_ref(),
        ],
        &freezing_program.id(),
    );

    let (user_gpass_wallet, _) = Pubkey::find_program_address(
        &[
            gpass::state::USER_WALLET_SEED.as_bytes(),
            freezing_params_data.gpass_settings.as_ref(),
            freezing_program.payer().as_ref(),
        ],
        &gpass_program_id,
    );

    let user_ggwp_wallet = get_or_create_token_account(
        &freezing_program,
        freezing_params_data.ggwp_token,
        freezing_program.payer(),
    )?;

    freezing_program
        .request()
        .accounts(freezing::accounts::Freeze {
            user: freezing_program.payer(),
            user_info: user_info,
            freezing_params: params,
            user_ggwp_wallet: user_ggwp_wallet,
            gpass_settings: freezing_params_data.gpass_settings,
            gpass_mint_auth: gpass_mint_auth,
            user_gpass_wallet: user_gpass_wallet,
            accumulative_fund: freezing_params_data.accumulative_fund,
            treasury: freezing_params_data.treasury,
            gpass_program: gpass_program_id,
            system_program: system_program::ID,
            token_program: spl_token::id(),
        })
        .args(freezing::instruction::Freeze { amount: amount })
        .send()?;

    Ok(())
}

fn cmd_withdraw_gpass(
    freezing_program: &Program,
    gpass_program_id: Pubkey,
    params: Pubkey,
) -> Result<(), ClientError> {
    let freezing_params_data: FreezingParams = freezing_program.account(params)?;
    let (user_info, _) = Pubkey::find_program_address(
        &[
            freezing::state::USER_INFO_SEED.as_bytes(),
            params.as_ref(),
            freezing_program.payer().as_ref(),
        ],
        &freezing_program.id(),
    );

    let (gpass_mint_auth, _) = Pubkey::find_program_address(
        &[
            GPASS_MINT_AUTH_SEED.as_bytes(),
            params.as_ref(),
            freezing_params_data.gpass_settings.as_ref(),
        ],
        &freezing_program.id(),
    );

    let (user_gpass_wallet, _) = Pubkey::find_program_address(
        &[
            gpass::state::USER_WALLET_SEED.as_bytes(),
            freezing_params_data.gpass_settings.as_ref(),
            freezing_program.payer().as_ref(),
        ],
        &gpass_program_id,
    );

    freezing_program
        .request()
        .accounts(freezing::accounts::Withdraw {
            user: freezing_program.payer(),
            user_info: user_info,
            freezing_params: params,
            gpass_settings: freezing_params_data.gpass_settings,
            gpass_mint_auth: gpass_mint_auth,
            user_gpass_wallet: user_gpass_wallet,
            gpass_program: gpass_program_id,
        })
        .args(freezing::instruction::WithdrawGpass {})
        .send()?;

    Ok(())
}

fn cmd_unfreeze(
    freezing_program: &Program,
    gpass_program_id: Pubkey,
    params: Pubkey,
) -> Result<(), ClientError> {
    let freezing_params_data: FreezingParams = freezing_program.account(params)?;
    let (user_info, _) = Pubkey::find_program_address(
        &[
            freezing::state::USER_INFO_SEED.as_bytes(),
            params.as_ref(),
            freezing_program.payer().as_ref(),
        ],
        &freezing_program.id(),
    );

    let (gpass_mint_auth, _) = Pubkey::find_program_address(
        &[
            GPASS_MINT_AUTH_SEED.as_bytes(),
            params.as_ref(),
            freezing_params_data.gpass_settings.as_ref(),
        ],
        &freezing_program.id(),
    );

    let (user_gpass_wallet, _) = Pubkey::find_program_address(
        &[
            gpass::state::USER_WALLET_SEED.as_bytes(),
            freezing_params_data.gpass_settings.as_ref(),
            freezing_program.payer().as_ref(),
        ],
        &gpass_program_id,
    );

    let user_ggwp_wallet = get_or_create_token_account(
        &freezing_program,
        freezing_params_data.ggwp_token,
        freezing_program.payer(),
    )?;

    let (treasury_auth, _) = Pubkey::find_program_address(
        &[TREASURY_AUTH_SEED.as_bytes(), params.as_ref()],
        &freezing_program.id(),
    );

    freezing_program
        .request()
        .accounts(freezing::accounts::Unfreeze {
            user: freezing_program.payer(),
            user_info: user_info,
            freezing_params: params,
            user_ggwp_wallet: user_ggwp_wallet,
            gpass_settings: freezing_params_data.gpass_settings,
            gpass_mint_auth: gpass_mint_auth,
            user_gpass_wallet: user_gpass_wallet,
            accumulative_fund: freezing_params_data.accumulative_fund,
            treasury: freezing_params_data.treasury,
            treasury_auth: treasury_auth,
            gpass_program: gpass_program_id,
            token_program: spl_token::id(),
        })
        .args(freezing::instruction::Unfreeze {})
        .send()?;

    Ok(())
}
