use crate::commands;
use anchor_client::anchor_lang::system_program;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::ClientError;
use anchor_client::{solana_sdk::pubkey::Pubkey, Client, Program};
use clap::{value_t_or_exit, values_t};
use clap::{ArgMatches, Error};

pub fn handle(cmd_matches: &ArgMatches, client: &Client, program_id: Pubkey) -> Result<(), Error> {
    let program = client.program(program_id);
    let (sub_command, arg_matches) = cmd_matches.subcommand();
    match (sub_command, arg_matches) {
        (commands::gpass::CMD_INITIALIZE, Some(arg_matches)) => {
            println!("Commad initialize");
            let burn_period = value_t_or_exit!(arg_matches, "burn_period", u64);
            let update_auth = value_t_or_exit!(arg_matches, "update_auth", Pubkey);
            let minters = values_t!(arg_matches, "minter", Pubkey).unwrap_or_default();
            let burners = values_t!(arg_matches, "burner", Pubkey).unwrap_or_default();
            cmd_initialize(&program, burn_period, update_auth, minters, burners)
                .expect("Initialize error");

            println!("Successful");
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
    burn_period: u64,
    update_auth: Pubkey,
    minters: Vec<Pubkey>,
    burners: Vec<Pubkey>,
) -> Result<(), ClientError> {
    let settings = Keypair::new();
    println!("New GPASS Pubkey: {}", settings.pubkey());

    program
        .request()
        .accounts(gpass::accounts::Initialize {
            admin: program.payer(),
            settings: settings.pubkey(),
            system_program: system_program::ID,
        })
        .args(gpass::instruction::Initialize {
            burn_period: burn_period,
            update_auth: update_auth,
            minters: minters,
            burners: burners,
        })
        .signer(&settings)
        .send()?;

    Ok(())
}
