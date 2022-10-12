use super::CMDS_REWARD_DISTRIBUTION;
use clap::{App, AppSettings, Arg, SubCommand};
use solana_clap_utils::input_validators::is_valid_pubkey;

pub const CMD_INITIALIZE: &str = "initialize";
pub const CMD_UPDATE_ADMIN: &str = "update-admin";
pub const CMD_SET_UPDATE_AUTHORITY: &str = "set-update-authority";
pub const CMD_UPDATE_TRANSFER_AUTH_LIST: &str = "update-transfer-auth-list";
pub const CMD_SHOW_INFO: &str = "show-info";

pub fn get_reward_distribution_commands<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CMDS_REWARD_DISTRIBUTION)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("Reward distribution smart contract commands.")
        .subcommand(
            SubCommand::with_name(CMD_INITIALIZE)
                .about("Initialize the reward distribution contract")
                .arg(
                    Arg::with_name("update_auth")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The reward distribution info update authority pubkey."),
                )
                .arg(
                    Arg::with_name("ggwp_token")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The GGWP token pubkey."),
                )
                .arg(
                    Arg::with_name("play_to_earn_fund")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The play to earn fund pubkey."),
                )
                .arg(
                    Arg::with_name("transfer_auth")
                        .short("t")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .multiple(true)
                        .takes_value(true)
                        .help("The transfer auth list pubkeys."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_UPDATE_ADMIN)
                .about("Admin can set the new admin.")
                .arg(
                    Arg::with_name("reward_distribution_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The reward distribution info account address."),
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
                    Arg::with_name("reward_distribution_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The reward distribution info account address."),
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
            SubCommand::with_name(CMD_UPDATE_TRANSFER_AUTH_LIST)
                .about("Update authority can set the new transfer auth list.")
                .arg(
                    Arg::with_name("reward_distribution_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The reward distribution info account address."),
                )
                .arg(
                    Arg::with_name("transfer_auth")
                        .short("t")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .multiple(true)
                        .takes_value(true)
                        .help("The transfer auth list pubkeys."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_SHOW_INFO)
                .about("Show reward distribution account info.")
                .arg(
                    Arg::with_name("reward_distribution_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The reward distribution info account address."),
                ),
        )
}
