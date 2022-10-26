use crate::commands;
use anchor_client::anchor_lang::system_program;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::ClientError;
use anchor_client::{solana_sdk::pubkey::Pubkey, Client, Program};
use clap::value_t_or_exit;
use clap::{ArgMatches, Error};
use fighting::state::{
    FightingSettings, GameInfo, UserFightingInfo, GAME_INFO_SEED, GPASS_BURN_AUTH_SEED,
    REWARD_TRANSFER_AUTH_SEED, USER_INFO_SEED,
};

pub fn handle(
    cmd_matches: &ArgMatches,
    client: &Client,
    fighting_program_id: Pubkey,
) -> Result<(), Error> {
    let program = client.program(fighting_program_id);
    let (sub_command, arg_matches) = cmd_matches.subcommand();
    match (sub_command, arg_matches) {
        (commands::fighting::CMD_INITIALIZE, Some(arg_matches)) => {
            println!("Commad initialize");
            let update_auth = value_t_or_exit!(arg_matches, "update_auth", Pubkey);
            let validator = value_t_or_exit!(arg_matches, "validator", Pubkey);
            let gpass_info = value_t_or_exit!(arg_matches, "gpass_info", Pubkey);
            let reward_distribution_info =
                value_t_or_exit!(arg_matches, "reward_distribution_info", Pubkey);
            let afk_timeout = value_t_or_exit!(arg_matches, "afk_timeout", i64);
            let royalty = value_t_or_exit!(arg_matches, "royalty", u8);
            let reward_coefficient = value_t_or_exit!(arg_matches, "reward_coefficient", u32);
            let gpass_daily_reward_coefficient =
                value_t_or_exit!(arg_matches, "gpass_daily_reward_coefficient", u32);

            cmd_initialize(
                &program,
                update_auth,
                validator,
                gpass_info,
                reward_distribution_info,
                afk_timeout,
                royalty,
                reward_coefficient,
                gpass_daily_reward_coefficient,
            )
            .expect("Initialize error");

            println!("Successful");
            Ok(())
        }

        (commands::fighting::CMD_UPDATE_ADMIN, Some(arg_matches)) => {
            let fighting_settings = value_t_or_exit!(arg_matches, "fighting_settings", Pubkey);
            let admin = value_t_or_exit!(arg_matches, "admin", Pubkey);
            cmd_update_admin(&program, fighting_settings, admin).expect("Update admin error");

            println!("Successful");
            Ok(())
        }

        (commands::fighting::CMD_SET_UPDATE_AUTHORITY, Some(arg_matches)) => {
            let fighting_settings = value_t_or_exit!(arg_matches, "fighting_settings", Pubkey);
            let update_aut = value_t_or_exit!(arg_matches, "update_auth", Pubkey);
            cmd_set_update_authority(&program, fighting_settings, update_aut)
                .expect("Set update authority error");

            println!("Successful");
            Ok(())
        }

        (commands::fighting::CMD_UPDATE_AFK_TIMEOUT, Some(arg_matches)) => {
            let fighting_settings = value_t_or_exit!(arg_matches, "fighting_settings", Pubkey);
            let afk_timeout = value_t_or_exit!(arg_matches, "afk_timeout", i64);

            cmd_update_afk_timeout(&program, fighting_settings, afk_timeout)
                .expect("Update afk timeout error");

            println!("Successful");
            Ok(())
        }

        (commands::fighting::CMD_UPDATE_VALIDATOR, Some(arg_matches)) => {
            let fighting_settings = value_t_or_exit!(arg_matches, "fighting_settings", Pubkey);
            let validator = value_t_or_exit!(arg_matches, "validator", Pubkey);

            cmd_update_validator(&program, fighting_settings, validator)
                .expect("Update validator error");

            println!("Successful");
            Ok(())
        }

        (commands::fighting::CMD_SHOW_SETTINGS, Some(arg_matches)) => {
            let fighting_settings = value_t_or_exit!(arg_matches, "fighting_settings", Pubkey);
            let data: FightingSettings = program
                .account(fighting_settings)
                .expect("Account fetch error");
            println!("Fighting Settings: {:?}", data);

            Ok(())
        }

        (commands::fighting::CMD_SHOW_GAME_INFO, Some(arg_matches)) => {
            let fighting_settings = value_t_or_exit!(arg_matches, "fighting_settings", Pubkey);
            let user = value_t_or_exit!(arg_matches, "user", Pubkey);
            let game_id = value_t_or_exit!(arg_matches, "game_id", u64);

            let (game_info, _) = Pubkey::find_program_address(
                &[
                    GAME_INFO_SEED.as_bytes(),
                    fighting_settings.as_ref(),
                    user.as_ref(),
                    game_id.to_le_bytes().as_ref(),
                ],
                &program.id(),
            );
            println!("Game info PK: {}", game_info);

            let data: GameInfo = program.account(game_info).expect("Account fetch error");
            println!("Game Info: {:?}", data);

            Ok(())
        }

        (commands::fighting::CMD_SHOW_USER_INFO, Some(arg_matches)) => {
            let fighting_settings = value_t_or_exit!(arg_matches, "fighting_settings", Pubkey);
            let user = value_t_or_exit!(arg_matches, "user", Pubkey);

            let (user_info, _) = Pubkey::find_program_address(
                &[
                    USER_INFO_SEED.as_bytes(),
                    fighting_settings.as_ref(),
                    user.as_ref(),
                ],
                &program.id(),
            );
            println!("User info PK: {}", user_info);

            let data: UserFightingInfo = program.account(user_info).expect("Account fetch error");
            println!("User Info: {:?}", data);

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
    validator: Pubkey,
    gpass_info: Pubkey,
    reward_distribution_info: Pubkey,
    afk_timeout: i64,
    royalty: u8,
    reward_coefficient: u32,
    gpass_daily_reward_coefficient: u32,
) -> Result<(), ClientError> {
    let fighting_settings = Keypair::new();
    println!(
        "New Fighting settings Pubkey: {}",
        fighting_settings.pubkey()
    );

    let (gpass_burn_auth, _) = Pubkey::find_program_address(
        &[
            GPASS_BURN_AUTH_SEED.as_bytes(),
            fighting_settings.pubkey().as_ref(),
            gpass_info.as_ref(),
        ],
        &program.id(),
    );
    println!("GPASS burn auth PK: {}", gpass_burn_auth);

    let (reward_transfer_auth, _) = Pubkey::find_program_address(
        &[
            REWARD_TRANSFER_AUTH_SEED.as_bytes(),
            fighting_settings.pubkey().as_ref(),
            reward_distribution_info.as_ref(),
        ],
        &program.id(),
    );
    println!("Reward transfer auth PK: {}", reward_transfer_auth);

    program
        .request()
        .accounts(fighting::accounts::Initialize {
            admin: program.payer(),
            fighting_settings: fighting_settings.pubkey(),
            reward_distribution_info: reward_distribution_info,
            reward_transfer_auth: reward_transfer_auth,
            gpass_info: gpass_info,
            gpass_burn_auth: gpass_burn_auth,
            system_program: system_program::ID,
        })
        .args(fighting::instruction::Initialize {
            validator: validator,
            update_auth: update_auth,
            afk_timeout: afk_timeout,
            royalty: royalty,
            reward_coefficient: reward_coefficient,
            gpass_daily_reward_coefficient: gpass_daily_reward_coefficient,
        })
        .signer(&fighting_settings)
        .send()?;

    Ok(())
}

fn cmd_update_admin(
    program: &Program,
    fighting_settings: Pubkey,
    admin: Pubkey,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(fighting::accounts::UpdateSetting {
            authority: program.payer(),
            fighting_settings: fighting_settings,
        })
        .args(fighting::instruction::UpdateAdmin { admin: admin })
        .send()?;

    Ok(())
}

fn cmd_set_update_authority(
    program: &Program,
    fighting_settings: Pubkey,
    update_auth: Pubkey,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(fighting::accounts::UpdateSetting {
            authority: program.payer(),
            fighting_settings: fighting_settings,
        })
        .args(fighting::instruction::SetUpdateAuthority {
            update_auth: update_auth,
        })
        .send()?;

    Ok(())
}

fn cmd_update_afk_timeout(
    program: &Program,
    fighting_settings: Pubkey,
    afk_timeout: i64,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(fighting::accounts::UpdateSetting {
            authority: program.payer(),
            fighting_settings: fighting_settings,
        })
        .args(fighting::instruction::UpdateAfkTimeout {
            afk_timeout: afk_timeout,
        })
        .send()?;

    Ok(())
}

fn cmd_update_validator(
    program: &Program,
    fighting_settings: Pubkey,
    validator: Pubkey,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(fighting::accounts::UpdateSetting {
            authority: program.payer(),
            fighting_settings: fighting_settings,
        })
        .args(fighting::instruction::UpdateValidator {
            validator: validator,
        })
        .send()?;

    Ok(())
}
