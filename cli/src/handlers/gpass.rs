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

        (commands::gpass::CMD_UPDATE_ADMIN, Some(arg_matches)) => {
            let settings = value_t_or_exit!(arg_matches, "settings", Pubkey);
            let admin = value_t_or_exit!(arg_matches, "admin", Pubkey);
            cmd_update_admin(&program, settings, admin).expect("Update admin error");

            println!("Successful");
            Ok(())
        }

        (commands::gpass::CMD_SET_UPDATE_AUTHORITY, Some(arg_matches)) => {
            let settings = value_t_or_exit!(arg_matches, "settings", Pubkey);
            let update_authority = value_t_or_exit!(arg_matches, "update_authority", Pubkey);
            cmd_set_update_authority(&program, settings, update_authority)
                .expect("Set update authority error");

            println!("Successful");
            Ok(())
        }

        (commands::gpass::CMD_UPDATE_BURN_PERIOD, Some(arg_matches)) => {
            let settings = value_t_or_exit!(arg_matches, "settings", Pubkey);
            let burn_period = value_t_or_exit!(arg_matches, "burn_period", u64);
            cmd_update_burn_period(&program, settings, burn_period)
                .expect("Update burn period error");

            println!("Successful");
            Ok(())
        }

        (commands::gpass::CMD_UPDATE_MINTERS, Some(arg_matches)) => {
            let settings = value_t_or_exit!(arg_matches, "settings", Pubkey);
            let minters = values_t!(arg_matches, "minter", Pubkey).unwrap_or_default();
            cmd_update_minters(&program, settings, minters).expect("Update minters error");

            println!("Successful");
            Ok(())
        }

        (commands::gpass::CMD_UPDATE_BURNERS, Some(arg_matches)) => {
            let settings = value_t_or_exit!(arg_matches, "settings", Pubkey);
            let burners = values_t!(arg_matches, "burners", Pubkey).unwrap_or_default();
            cmd_update_burners(&program, settings, burners).expect("Update burners error");

            println!("Successful");
            Ok(())
        }

        (commands::gpass::CMD_CREATE_WALLET, Some(arg_matches)) => {
            let settings = value_t_or_exit!(arg_matches, "settings", Pubkey);
            let user = value_t_or_exit!(arg_matches, "user", Pubkey);
            cmd_create_wallet(&program, settings, user).expect("Create wallet error");

            println!("Successful");
            Ok(())
        }

        (commands::gpass::CMD_MINT_TO, Some(arg_matches)) => {
            let settings = value_t_or_exit!(arg_matches, "settings", Pubkey);
            let to = value_t_or_exit!(arg_matches, "to", Pubkey);
            let amount = value_t_or_exit!(arg_matches, "amount", u64);
            cmd_mint_to(&program, settings, to, amount).expect("Mint to error");

            println!("Successful");
            Ok(())
        }

        (commands::gpass::CMD_BURN, Some(arg_matches)) => {
            let settings = value_t_or_exit!(arg_matches, "settings", Pubkey);
            let from = value_t_or_exit!(arg_matches, "from", Pubkey);
            let amount = value_t_or_exit!(arg_matches, "amount", u64);
            cmd_burn(&program, settings, from, amount).expect("Burn error");

            println!("Successful");
            Ok(())
        }

        (commands::gpass::CMD_TRY_BURN_IN_PERIOD, Some(arg_matches)) => {
            let settings = value_t_or_exit!(arg_matches, "settings", Pubkey);
            let wallet = value_t_or_exit!(arg_matches, "wallet", Pubkey);
            cmd_try_burn_in_period(&program, settings, wallet).expect("Burn in period error");

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

fn cmd_update_admin(program: &Program, settings: Pubkey, admin: Pubkey) -> Result<(), ClientError> {
    program
        .request()
        .accounts(gpass::accounts::UpdateParam {
            authority: program.payer(),
            settings: settings,
        })
        .args(gpass::instruction::UpdateAdmin { admin: admin })
        .send()?;

    Ok(())
}

fn cmd_set_update_authority(
    program: &Program,
    settings: Pubkey,
    update_auth: Pubkey,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(gpass::accounts::UpdateParam {
            authority: program.payer(),
            settings: settings,
        })
        .args(gpass::instruction::SetUpdateAuthority {
            update_auth: update_auth,
        })
        .send()?;

    Ok(())
}

fn cmd_update_burn_period(
    program: &Program,
    settings: Pubkey,
    burn_period: u64,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(gpass::accounts::UpdateParam {
            authority: program.payer(),
            settings: settings,
        })
        .args(gpass::instruction::UpdateBurnPeriod {
            burn_period: burn_period,
        })
        .send()?;

    Ok(())
}

fn cmd_update_minters(
    program: &Program,
    settings: Pubkey,
    minters: Vec<Pubkey>,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(gpass::accounts::UpdateParam {
            authority: program.payer(),
            settings: settings,
        })
        .args(gpass::instruction::UpdateMinters { minters: minters })
        .send()?;

    Ok(())
}

fn cmd_update_burners(
    program: &Program,
    settings: Pubkey,
    burners: Vec<Pubkey>,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(gpass::accounts::UpdateParam {
            authority: program.payer(),
            settings: settings,
        })
        .args(gpass::instruction::UpdateBurners { burners: burners })
        .send()?;

    Ok(())
}

fn cmd_create_wallet(program: &Program, settings: Pubkey, user: Pubkey) -> Result<(), ClientError> {
    let (wallet, _bump) = Pubkey::find_program_address(
        &[
            gpass::state::USER_WALLET_SEED.as_bytes(),
            program.id().as_ref(),
            settings.as_ref(),
            user.as_ref(),
        ],
        &program.id(),
    );

    program
        .request()
        .accounts(gpass::accounts::CreateWallet {
            payer: program.payer(),
            settings: settings,
            user: user,
            wallet: wallet,
            system_program: system_program::ID,
        })
        .args(gpass::instruction::CreateWallet {})
        .send()?;

    Ok(())
}

fn cmd_mint_to(
    program: &Program,
    settings: Pubkey,
    to: Pubkey,
    amount: u64,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(gpass::accounts::MintTo {
            authority: program.payer(),
            settings: settings,
            to: to,
        })
        .args(gpass::instruction::MintTo { amount: amount })
        .send()?;

    Ok(())
}

fn cmd_burn(
    program: &Program,
    settings: Pubkey,
    from: Pubkey,
    amount: u64,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(gpass::accounts::Burn {
            authority: program.payer(),
            settings: settings,
            from: from,
        })
        .args(gpass::instruction::MintTo { amount: amount })
        .send()?;

    Ok(())
}

fn cmd_try_burn_in_period(
    program: &Program,
    settings: Pubkey,
    wallet: Pubkey,
) -> Result<(), ClientError> {
    program
        .request()
        .accounts(gpass::accounts::BurnInPeriod {
            settings: settings,
            wallet: wallet,
        })
        .args(gpass::instruction::TryBurnInPeriod {})
        .send()?;

    Ok(())
}
