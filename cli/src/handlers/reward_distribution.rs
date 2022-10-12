use crate::commands;
use anchor_client::anchor_lang::system_program;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::ClientError;
use anchor_client::{solana_sdk::pubkey::Pubkey, Client, Program};
use clap::{value_t_or_exit, values_t};
use clap::{ArgMatches, Error};
use reward_distribution::state::{RewardDistributionInfo, PLAY_TO_EARN_FUND_AUTH_SEED};

pub fn handle(
    cmd_matches: &ArgMatches,
    client: &Client,
    reward_distribution_program_id: Pubkey,
) -> Result<(), Error> {
    let program = client.program(reward_distribution_program_id);
    let (sub_command, arg_matches) = cmd_matches.subcommand();
    match (sub_command, arg_matches) {
        (commands::reward_distribution::CMD_INITIALIZE, Some(arg_matches)) => {
            println!("Commad initialize");
            let update_auth = value_t_or_exit!(arg_matches, "update_auth", Pubkey);
            let ggwp_token = value_t_or_exit!(arg_matches, "ggwp_token", Pubkey);
            let play_to_earn_fund = value_t_or_exit!(arg_matches, "play_to_earn_fund", Pubkey);
            let transfer_auth_list =
                values_t!(arg_matches, "transfer_auth", Pubkey).unwrap_or_default();

            cmd_initialize(
                &program,
                update_auth,
                ggwp_token,
                play_to_earn_fund,
                transfer_auth_list,
            )
            .expect("Initialize error");

            println!("Successful");
            Ok(())
        }

        (commands::reward_distribution::CMD_UPDATE_ADMIN, Some(arg_matches)) => {
            let reward_distribution_info =
                value_t_or_exit!(arg_matches, "reward_distribution_info", Pubkey);
            let admin = value_t_or_exit!(arg_matches, "admin", Pubkey);
            cmd_update_admin(&program, reward_distribution_info, admin)
                .expect("Update admin error");

            println!("Successful");
            Ok(())
        }

        (commands::fighting::CMD_SET_UPDATE_AUTHORITY, Some(arg_matches)) => {
            let reward_distribution_info =
                value_t_or_exit!(arg_matches, "reward_distribution_info", Pubkey);
            let update_aut = value_t_or_exit!(arg_matches, "update_auth", Pubkey);
            cmd_set_update_authority(&program, reward_distribution_info, update_aut)
                .expect("Set update authority error");

            println!("Successful");
            Ok(())
        }

        (commands::reward_distribution::CMD_UPDATE_TRANSFER_AUTH_LIST, Some(arg_matches)) => {
            let reward_distribution_info =
                value_t_or_exit!(arg_matches, "reward_distribution_info", Pubkey);
            let transfer_auth_list =
                values_t!(arg_matches, "transfer_auth", Pubkey).unwrap_or_default();

            cmd_update_transfer_auth_list(&program, reward_distribution_info, transfer_auth_list)
                .expect("Update transfer auth list error");

            println!("Successful");
            Ok(())
        }

        (commands::reward_distribution::CMD_SHOW_INFO, Some(arg_matches)) => {
            let reward_distribution_info =
                value_t_or_exit!(arg_matches, "reward_distribution_info", Pubkey);
            let data: RewardDistributionInfo = program
                .account(reward_distribution_info)
                .expect("Account fetch error");
            println!("Reward Distribution info: {:?}", data);

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
    play_to_earn_fund: Pubkey,
    transfer_auth_list: Vec<Pubkey>,
) -> Result<(), ClientError> {
    let reward_distribution_info = Keypair::new();
    println!(
        "New Reward Distribution info Pubkey: {}",
        reward_distribution_info.pubkey()
    );

    let (play_to_earn_fund_auth, _) = Pubkey::find_program_address(
        &[
            PLAY_TO_EARN_FUND_AUTH_SEED.as_bytes(),
            reward_distribution_info.pubkey().as_ref(),
        ],
        &program.id(),
    );
    println!("Play to earn fund auth: {}", play_to_earn_fund_auth);

    program
        .request()
        .accounts(reward_distribution::accounts::Initialize {
            admin: program.payer(),
            reward_distribution_info: reward_distribution_info.pubkey(),
            ggwp_token: ggwp_token,
            play_to_earn_fund: play_to_earn_fund,
            play_to_earn_fund_auth: play_to_earn_fund_auth,
            system_program: system_program::ID,
        })
        .args(reward_distribution::instruction::Initialize {
            update_auth: update_auth,
            transfer_auth_list: transfer_auth_list,
        })
        .signer(&reward_distribution_info)
        .send()?;

    Ok(())
}

fn cmd_update_admin(
    program: &Program,
    reward_distribution_info: Pubkey,
    admin: Pubkey,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(reward_distribution::accounts::UpdateParam {
            authority: program.payer(),
            reward_distribution_info: reward_distribution_info,
        })
        .args(reward_distribution::instruction::UpdateAdmin { admin: admin })
        .send()?;

    Ok(())
}

fn cmd_set_update_authority(
    program: &Program,
    reward_distribution_info: Pubkey,
    update_auth: Pubkey,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(reward_distribution::accounts::UpdateParam {
            authority: program.payer(),
            reward_distribution_info: reward_distribution_info,
        })
        .args(reward_distribution::instruction::SetUpdateAuthority {
            update_auth: update_auth,
        })
        .send()?;

    Ok(())
}

fn cmd_update_transfer_auth_list(
    program: &Program,
    reward_distribution_info: Pubkey,
    transfer_auth_list: Vec<Pubkey>,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(reward_distribution::accounts::UpdateParam {
            authority: program.payer(),
            reward_distribution_info: reward_distribution_info,
        })
        .args(
            reward_distribution::instruction::UpdateTransferAuthorityList {
                transfer_auth_list: transfer_auth_list,
            },
        )
        .send()?;

    Ok(())
}
