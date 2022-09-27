use super::CMDS_DISTRIBUTION;
use clap::{App, AppSettings, Arg, SubCommand};
use solana_clap_utils::input_validators::is_valid_pubkey;

pub const CMD_INITIALIZE: &str = "initialize";
pub const CMD_DISTRIBUTE: &str = "distribute";
pub const CMD_UPDATE_ADMIN: &str = "update-admin";
pub const CMD_SET_UPDATE_AUTHORITY: &str = "set-update-authority";
pub const CMD_UPDATE_SHARES: &str = "update-shares";
pub const CMD_SHOW_FUNDS_INFO: &str = "show-funds-info";

pub fn get_distribution_commands<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CMDS_DISTRIBUTION)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("Distribution smart contract commands.")
        .subcommand(
            SubCommand::with_name(CMD_INITIALIZE)
                .about("Initialize the distribution contract")
                .arg(
                    Arg::with_name("update_auth")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The distribution info update authority pubkey."),
                )
                .arg(
                    Arg::with_name("ggwp_token")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The GGWP Token (mint) pubkey."),
                )
                .arg(
                    Arg::with_name("play_to_earn_fund")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The play to earn fund wallet pubkey."),
                )
                .arg(
                    Arg::with_name("play_to_earn_fund_share")
                        .value_name("u8")
                        .required(true)
                        .takes_value(true)
                        .help("The play to earn fund share in percent."),
                )
                .arg(
                    Arg::with_name("staking_fund")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The staking fund wallet pubkey."),
                )
                .arg(
                    Arg::with_name("staking_fund_share")
                        .value_name("u8")
                        .required(true)
                        .takes_value(true)
                        .help("The staking fund share in percent."),
                )
                .arg(
                    Arg::with_name("company_fund")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The company fund wallet pubkey."),
                )
                .arg(
                    Arg::with_name("company_fund_share")
                        .value_name("u8")
                        .required(true)
                        .takes_value(true)
                        .help("The company fund share in percent."),
                )
                .arg(
                    Arg::with_name("team_fund")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The team fund wallet pubkey."),
                )
                .arg(
                    Arg::with_name("team_fund_share")
                        .value_name("u8")
                        .required(true)
                        .takes_value(true)
                        .help("The team fund share in percent."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_DISTRIBUTE)
                .about("Anyone can start the distribution of GGWP tokens.")
                .arg(
                    Arg::with_name("distribution_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The distribution info account address."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_UPDATE_ADMIN)
                .about("Admin can set the new admin.")
                .arg(
                    Arg::with_name("distribution_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The distribution info account address."),
                )
                .arg(
                    Arg::with_name("admin")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The new admin address."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_SET_UPDATE_AUTHORITY)
                .about("Admin can set the new update authority.")
                .arg(
                    Arg::with_name("distribution_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The distribution info account address."),
                )
                .arg(
                    Arg::with_name("update_auth")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The new update authority address."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_UPDATE_SHARES)
                .about("Update authority can set the new fund shares.")
                .arg(
                    Arg::with_name("distribution_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The distribution info account address."),
                )
                .arg(
                    Arg::with_name("play_to_earn_fund_share")
                        .value_name("u8")
                        .required(true)
                        .takes_value(true)
                        .help("The play to earn fund share in percent."),
                )
                .arg(
                    Arg::with_name("staking_fund_share")
                        .value_name("u8")
                        .required(true)
                        .takes_value(true)
                        .help("The staking fund share in percent."),
                )
                .arg(
                    Arg::with_name("company_fund_share")
                        .value_name("u8")
                        .required(true)
                        .takes_value(true)
                        .help("The company fund share in percent."),
                )
                .arg(
                    Arg::with_name("team_fund_share")
                        .value_name("u8")
                        .required(true)
                        .takes_value(true)
                        .help("The team fund share in percent."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_SHOW_FUNDS_INFO)
                .about("Show information about funds.")
                .arg(
                    Arg::with_name("distribution_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The distribution info account address."),
                ),
        )
}
