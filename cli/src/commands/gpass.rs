use super::CMDS_GPASS;
use clap::{App, Arg, SubCommand, AppSettings};
use solana_clap_utils::input_validators::is_valid_pubkey;

pub const CMD_INITIALIZE: &str = "initialize";

pub fn get_gpass_commands<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CMDS_GPASS)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("GPASS Smart contract commands.")
        .subcommand(
            SubCommand::with_name(CMD_INITIALIZE)
                .about("Initialize the GPASS contract")
                .arg(
                    Arg::with_name("burn_period")
                        .value_name("u64")
                        .required(true)
                        .takes_value(true)
                        .help("The burn period value in seconds."),
                )
                .arg(
                    Arg::with_name("update_auth")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The params update authority pubkey."),
                )
                .arg(
                    Arg::with_name("minter")
                        .short("m")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .multiple(true)
                        .takes_value(true)
                        .help("The minters pubkeys."),
                )
                .arg(
                    Arg::with_name("burner")
                        .short("b")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .multiple(true)
                        .takes_value(true)
                        .help("The burners pubkeys."),
                ),
        )
}
