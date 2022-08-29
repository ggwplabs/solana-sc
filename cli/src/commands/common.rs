use super::CMDS_COMMON;
use clap::{App, AppSettings, Arg, SubCommand};
use solana_clap_utils::input_validators::is_valid_pubkey;

pub const CMD_INIT_ALL: &str = "init-all";

pub fn get_common_commands<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CMDS_COMMON)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("Commom commands.")
        .subcommand(
            SubCommand::with_name(CMD_INIT_ALL)
                .about(
                    "Initialize all infrastructure of smart contracts after deploy. \
                    Need to be started by admin.",
                )
                .arg(
                    Arg::with_name("update_auth")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help(
                            "Update authority pubkey (system account or managing smart contract).",
                        ),
                )
                .arg(
                    Arg::with_name("ggwp_token")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("GGWP Token mint pubkey."),
                ),
        )
}
