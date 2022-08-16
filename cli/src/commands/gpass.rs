use super::CMDS_GPASS;
use clap::{App, AppSettings, Arg, SubCommand};
use solana_clap_utils::input_validators::is_valid_pubkey;

pub const CMD_INITIALIZE: &str = "initialize";
pub const CMD_UPDATE_ADMIN: &str = "update-admin";
pub const CMD_SET_UPDATE_AUTHORITY: &str = "set-update-authority";
pub const CMD_UPDATE_BURN_PERIOD: &str = "update-burn-period";
pub const CMD_UPDATE_MINTERS: &str = "update-minters";
pub const CMD_UPDATE_BURNERS: &str = "update-burners";
pub const CMD_CREATE_WALLET: &str = "create-wallet";
pub const CMD_MINT_TO: &str = "mint-to";
pub const CMD_BURN: &str = "burn";
pub const CMD_TRY_BURN_IN_PERIOD: &str = "try-burn-in-period";
pub const CMD_SHOW_INFO: &str = "show-info";
pub const CMD_SHOW_WALLET: &str = "show-wallet";

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
                        .help("The GPASS info update authority pubkey."),
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
        .subcommand(
            SubCommand::with_name(CMD_UPDATE_ADMIN)
                .about("Admin can set the new admin of GPASS.")
                .arg(
                    Arg::with_name("gpass_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The GPASS Info account address."),
                )
                .arg(
                    Arg::with_name("admin")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The address of new admin."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_SET_UPDATE_AUTHORITY)
                .about("Admin can set the new update authority of GPASS.")
                .arg(
                    Arg::with_name("gpass_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The GPASS Info account address."),
                )
                .arg(
                    Arg::with_name("update_authority")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The address of new update authority."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_UPDATE_BURN_PERIOD)
                .about("Update authority can set the new burn period value.")
                .arg(
                    Arg::with_name("gpass_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The GPASS Info account address."),
                )
                .arg(
                    Arg::with_name("burn_period")
                        .value_name("u64")
                        .required(true)
                        .takes_value(true)
                        .help("The new burn period value."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_UPDATE_MINTERS)
                .about("Update authority can set the new list of minters.")
                .arg(
                    Arg::with_name("gpass_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The GPASS Info account address."),
                )
                .arg(
                    Arg::with_name("minter")
                        .short("m")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .multiple(true)
                        .takes_value(true)
                        .help("The minters pubkeys."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_UPDATE_BURNERS)
                .about("Update authority can set the new list of burners.")
                .arg(
                    Arg::with_name("gpass_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The GPASS Info account address."),
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
        .subcommand(
            SubCommand::with_name(CMD_CREATE_WALLET)
                .about("Create the new GPASS wallet for user.")
                .arg(
                    Arg::with_name("gpass_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The GPASS Info account address."),
                )
                .arg(
                    Arg::with_name("user")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The user pubkey."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_MINT_TO)
                .about("Mint the amount of GPASS into user wallet. Only for mint authority.")
                .arg(
                    Arg::with_name("gpass_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The GPASS Info account address."),
                )
                .arg(
                    Arg::with_name("to")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The user GPASS wallet pubkey."),
                )
                .arg(
                    Arg::with_name("amount")
                        .value_name("u64")
                        .required(true)
                        .takes_value(true)
                        .help("The amount to mint."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_BURN)
                .about("Burn the amount of GPASS from user wallet. Only for burn authority.")
                .arg(
                    Arg::with_name("gpass_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The GPASS Info account address."),
                )
                .arg(
                    Arg::with_name("from")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The user GPASS wallet pubkey."),
                )
                .arg(
                    Arg::with_name("amount")
                        .value_name("u64")
                        .required(true)
                        .takes_value(true)
                        .help("The amount to burn."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_TRY_BURN_IN_PERIOD)
                .about("Try to burn the full amount of GPASS from user wallet in period.")
                .arg(
                    Arg::with_name("gpass_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The GPASS Info account address."),
                )
                .arg(
                    Arg::with_name("wallet")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The user GPASS wallet pubkey."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_SHOW_INFO)
                .about("Show the information about GPASS Info.")
                .arg(
                    Arg::with_name("gpass_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The GPASS Info account address."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_SHOW_WALLET)
                .about("Show the information about GPASS user wallet.")
                .arg(
                    Arg::with_name("gpass_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The GPASS Info account address."),
                )
                .arg(
                    Arg::with_name("user")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The user account address."),
                ),
        )
}
