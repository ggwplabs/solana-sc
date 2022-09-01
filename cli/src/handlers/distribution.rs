use super::utils::get_or_create_token_account;
use crate::commands;
use crate::handlers::utils::get_token_account_data;
use anchor_client::anchor_lang::system_program;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::ClientError;
use anchor_client::{solana_sdk::pubkey::Pubkey, Client, Program};
use clap::value_t_or_exit;
use clap::{ArgMatches, Error};
use distribution::state::{DistributionInfo, ACCUMULATIVE_FUND_AUTH_SEED};

pub fn handle(
    cmd_matches: &ArgMatches,
    client: &Client,
    distribution_program_id: Pubkey,
) -> Result<(), Error> {
    let program = client.program(distribution_program_id);
    let (sub_command, arg_matches) = cmd_matches.subcommand();
    match (sub_command, arg_matches) {
        (commands::distribution::CMD_INITIALIZE, Some(arg_matches)) => {
            println!("Commad initialize");
            let update_auth = value_t_or_exit!(arg_matches, "update_auth", Pubkey);
            let ggwp_token = value_t_or_exit!(arg_matches, "ggwp_token", Pubkey);
            let play_to_earn_fund = value_t_or_exit!(arg_matches, "play_to_earn_fund", Pubkey);
            let play_to_earn_fund_share =
                value_t_or_exit!(arg_matches, "play_to_earn_fund_share", u8);
            let staking_fund = value_t_or_exit!(arg_matches, "staking_fund", Pubkey);
            let staking_fund_share = value_t_or_exit!(arg_matches, "staking_fund_share", u8);
            let company_fund = value_t_or_exit!(arg_matches, "company_fund", Pubkey);
            let company_fund_share = value_t_or_exit!(arg_matches, "company_fund_share", u8);
            let team_fund = value_t_or_exit!(arg_matches, "team_fund", Pubkey);
            let team_fund_share = value_t_or_exit!(arg_matches, "team_fund_share", u8);

            cmd_initialize(
                &program,
                update_auth,
                ggwp_token,
                play_to_earn_fund,
                play_to_earn_fund_share,
                staking_fund,
                staking_fund_share,
                company_fund,
                company_fund_share,
                team_fund,
                team_fund_share,
            )
            .expect("Initialize error");

            println!("Successful");
            Ok(())
        }

        (commands::distribution::CMD_UPDATE_ADMIN, Some(arg_matches)) => {
            let distribution_info = value_t_or_exit!(arg_matches, "distribution_info", Pubkey);
            let admin = value_t_or_exit!(arg_matches, "admin", Pubkey);
            cmd_update_admin(&program, distribution_info, admin).expect("Update admin error");

            println!("Successful");
            Ok(())
        }

        (commands::distribution::CMD_SET_UPDATE_AUTHORITY, Some(arg_matches)) => {
            let distribution_info = value_t_or_exit!(arg_matches, "distribution_info", Pubkey);
            let update_aut = value_t_or_exit!(arg_matches, "update_auth", Pubkey);
            cmd_set_update_authority(&program, distribution_info, update_aut)
                .expect("Set update authority error");

            println!("Successful");
            Ok(())
        }

        (commands::distribution::CMD_UPDATE_SHARES, Some(arg_matches)) => {
            let distribution_info = value_t_or_exit!(arg_matches, "distribution_info", Pubkey);
            let play_to_earn_fund_share =
                value_t_or_exit!(arg_matches, "play_to_earn_fund_share", u8);
            let staking_fund_share = value_t_or_exit!(arg_matches, "staking_fund_share", u8);
            let company_fund_share = value_t_or_exit!(arg_matches, "company_fund_share", u8);
            let team_fund_share = value_t_or_exit!(arg_matches, "team_fund_share", u8);

            cmd_update_shares(
                &program,
                distribution_info,
                play_to_earn_fund_share,
                staking_fund_share,
                company_fund_share,
                team_fund_share,
            )
            .expect("Update fund shares error");

            println!("Successful");
            Ok(())
        }

        (commands::distribution::CMD_SHOW_FUNDS_INFO, Some(arg_matches)) => {
            let distribution_info = value_t_or_exit!(arg_matches, "distribution_info", Pubkey);
            let data: DistributionInfo = program
                .account(distribution_info)
                .expect("Account fetch error");

            println!("Last distribution: {}", data.last_distribution);
            println!("Accumulative fund: {}", data.accumulative_fund);
            let amount = get_token_account_data(&program, data.accumulative_fund)
                .expect("Fund fetch error")
                .amount;
            println!("Amount: {}", amount);

            println!("Play to earn fund: {}", data.play_to_earn_fund);
            let amount = get_token_account_data(&program, data.play_to_earn_fund)
                .expect("Fund fetch error")
                .amount;
            println!("Amount: {}", amount);
            println!("Share: {}", data.play_to_earn_fund_share);

            println!("Staking fund: {}", data.staking_fund);
            let amount = get_token_account_data(&program, data.staking_fund)
                .expect("Fund fetch error")
                .amount;
            println!("Amount: {}", amount);
            println!("Share: {}", data.staking_fund_share);

            println!("Company fund: {}", data.company_fund);
            let amount = get_token_account_data(&program, data.company_fund)
                .expect("Fund fetch error")
                .amount;
            println!("Amount: {}", amount);
            println!("Share: {}", data.company_fund_share);

            println!("Team fund: {}", data.team_fund);
            let amount = get_token_account_data(&program, data.team_fund)
                .expect("Fund fetch error")
                .amount;
            println!("Amount: {}", amount);
            println!("Share: {}", data.team_fund_share);

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
    play_to_earn_fund_share: u8,
    staking_fund: Pubkey,
    staking_fund_share: u8,
    company_fund: Pubkey,
    company_fund_share: u8,
    team_fund: Pubkey,
    team_fund_share: u8,
) -> Result<(), ClientError> {
    let distribution_info = Keypair::new();
    println!(
        "New Distribution info Pubkey: {}",
        distribution_info.pubkey()
    );

    let (accumulative_fund_auth, _) = Pubkey::find_program_address(
        &[
            ACCUMULATIVE_FUND_AUTH_SEED.as_bytes(),
            distribution_info.pubkey().as_ref(),
        ],
        &program.id(),
    );

    let accumulative_fund =
        get_or_create_token_account(program, ggwp_token, accumulative_fund_auth)?;

    program
        .request()
        .accounts(distribution::accounts::Initialize {
            admin: program.payer(),
            distribution_info: distribution_info.pubkey(),
            ggwp_token: ggwp_token,
            accumulative_fund: accumulative_fund,
            accumulative_fund_auth: accumulative_fund_auth,
            play_to_earn_fund: play_to_earn_fund,
            staking_fund: staking_fund,
            company_fund: company_fund,
            team_fund: team_fund,
            system_program: system_program::ID,
        })
        .args(distribution::instruction::Initialize {
            update_auth: update_auth,
            play_to_earn_fund_share: play_to_earn_fund_share,
            staking_fund_share: staking_fund_share,
            company_fund_share: company_fund_share,
            team_fund_share: team_fund_share,
        })
        .signer(&distribution_info)
        .send()?;

    Ok(())
}

fn cmd_update_admin(
    program: &Program,
    distribution_info: Pubkey,
    admin: Pubkey,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(distribution::accounts::UpdateParam {
            authority: program.payer(),
            distribution_info: distribution_info,
        })
        .args(distribution::instruction::UpdateAdmin { admin: admin })
        .send()?;

    Ok(())
}

fn cmd_set_update_authority(
    program: &Program,
    distribution_info: Pubkey,
    update_auth: Pubkey,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(distribution::accounts::UpdateParam {
            authority: program.payer(),
            distribution_info: distribution_info,
        })
        .args(distribution::instruction::SetUpdateAuthority {
            update_auth: update_auth,
        })
        .send()?;

    Ok(())
}

fn cmd_update_shares(
    program: &Program,
    distribution_info: Pubkey,
    play_to_earn_fund_share: u8,
    staking_fund_share: u8,
    company_fund_share: u8,
    team_fund_share: u8,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(distribution::accounts::UpdateParam {
            authority: program.payer(),
            distribution_info: distribution_info,
        })
        .args(distribution::instruction::UpdateShares {
            play_to_earn_fund_share: play_to_earn_fund_share,
            staking_fund_share: staking_fund_share,
            company_fund_share: company_fund_share,
            team_fund_share: team_fund_share,
        })
        .send()?;

    Ok(())
}
