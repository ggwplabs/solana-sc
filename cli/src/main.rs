//! CLI Client for interacting with the smart contracts
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file,
    },
    Client, Cluster,
};
use clap::{crate_description, crate_name, crate_version};
use commands::gpass::get_gpass_commands;
use std::{rc::Rc, str::FromStr};

mod app;
mod commands;
mod config;
mod handlers;

fn main() {
    let app = app::get_clap_app(crate_name!(), crate_description!(), crate_version!());
    let app = app.subcommand(get_gpass_commands());
    let app_matches = app.get_matches();

    let config = if let Some(config_path) = app_matches.value_of("config") {
        config::CLIConfig::load(config_path).expect("Config loading error")
    } else {
        config::CLIConfig::default()
    };

    let cluster = Cluster::from_str(&config.network).expect("Cluster error");
    let payer = read_keypair_file(&config.fee_payer_path).expect("Reading payer keypair error");
    println!("RPC Client URL: {}", cluster.url());

    let client = Client::new_with_options(cluster, Rc::new(payer), CommitmentConfig::processed());
    let (sub_command, cmd_matches) = app_matches.subcommand();
    match (sub_command, cmd_matches) {
        (commands::CMDS_GPASS, Some(cmd_matches)) => {
            handlers::gpass::handle(
                cmd_matches,
                &client,
                Pubkey::from_str(&config.programs.gpass).expect("Error in parsing program id"),
            )
            .expect("GPASS handle error");
        }

        _ => {
            println!("{}", app_matches.usage());
        }
    }
}
