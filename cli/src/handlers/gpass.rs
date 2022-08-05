use crate::commands;
use anchor_client::{solana_sdk::pubkey::Pubkey, Client, Program};
use clap::{value_t, value_t_or_exit, values_t};
use clap::{ArgMatches, Error};

pub fn handle(cmd_matches: &ArgMatches, client: &Client, program_id: Pubkey) -> Result<(), Error> {
    let program = client.program(program_id);
    let (sub_command, arg_matches) = cmd_matches.subcommand();
    match (sub_command, arg_matches) {
        (commands::gpass::CMD_INITIALIZE, Some(arg_matches)) => {
            // TODO:
            cmd_initialize(&program);
            Ok(())
        }

        _ => {
            println!("{}", cmd_matches.usage());
            Ok(())
        }
    }
}

fn cmd_initialize(program: &Program) {}
