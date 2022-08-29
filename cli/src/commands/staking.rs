use super::CMDS_STAKING;
use clap::{App, AppSettings, Arg, SubCommand};
use solana_clap_utils::input_validators::is_valid_pubkey;

pub const CMD_INITIALIZE: &str = "initialize";
pub const CMD_UPDATE_ADMIN: &str = "update-admin";
pub const CMD_SET_UPDATE_AUTHORITY: &str = "set-update-authority";
pub const CMD_UPDATE_EPOCH_PERIOD_DAYS: &str = "update-epoch-period-days";
pub const CMD_UPDATE_MIN_STAKE_AMOUNT: &str = "update-min-stake-amount";
pub const CMD_UPDATE_HOLD_PERIOD_DAYS: &str = "update-hold-period-days";
pub const CMD_UPDATE_HOLD_ROYALTY: &str = "update-hold-royalty";
pub const CMD_UPDATE_ROYALTY: &str = "update-royalty";
pub const CMD_STAKE: &str = "stake";
pub const CMD_WITHDRAW: &str = "withdraw";
pub const CMD_SHOW_STAKING_INFO: &str = "show-staking-info";
pub const CMD_SHOW_USER_INFO: &str = "show-user-info";

pub fn get_staking_commands<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CMDS_STAKING)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("Staking smart contract commands.")
        .subcommand(
            SubCommand::with_name(CMD_INITIALIZE)
                .about("Initialize the staking contract.")
                .arg(
                    Arg::with_name("update_auth")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The staking info update authority pubkey."),
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
                    Arg::with_name("staking_fund")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The staking fund wallet pubkey."),
                )
                .arg(
                    Arg::with_name("accumulative_fund")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The accumulative fund wallet pubkey."),
                )
                .arg(
                    Arg::with_name("epoch_period_days")
                        .value_name("u16")
                        .required(true)
                        .takes_value(true)
                        .help("The epoch period in days."),
                )
                .arg(
                    Arg::with_name("min_stake_amount")
                        .value_name("f64")
                        .required(true)
                        .takes_value(true)
                        .help("The minimum amount to stake."),
                )
                .arg(
                    Arg::with_name("hold_period_days")
                        .value_name("u16")
                        .required(true)
                        .takes_value(true)
                        .help("The hold period in days."),
                )
                .arg(
                    Arg::with_name("hold_royalty")
                        .value_name("u8")
                        .required(true)
                        .takes_value(true)
                        .help("The hold royalty in percent."),
                )
                .arg(
                    Arg::with_name("royalty")
                        .value_name("u8")
                        .required(true)
                        .takes_value(true)
                        .help("The royalty value in percent."),
                )
                .arg(
                    Arg::with_name("apr_start")
                        .value_name("u8")
                        .required(true)
                        .takes_value(true)
                        .help("The start APR percent value."),
                )
                .arg(
                    Arg::with_name("apr_step")
                        .value_name("u8")
                        .required(true)
                        .takes_value(true)
                        .help("The step APR percent value."),
                )
                .arg(
                    Arg::with_name("apr_end")
                        .value_name("u8")
                        .required(true)
                        .takes_value(true)
                        .help("The end APR percent value."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_UPDATE_ADMIN)
                .about("Admin can set the new admin pubkey.")
                .arg(
                    Arg::with_name("staking_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The staking info pubkey."),
                )
                .arg(
                    Arg::with_name("admin")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The new admin pubkey."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_SET_UPDATE_AUTHORITY)
                .about("Admin can set the new update authority pubkey.")
                .arg(
                    Arg::with_name("staking_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The staking info pubkey."),
                )
                .arg(
                    Arg::with_name("update_auth")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The new update authority pubkey."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_UPDATE_EPOCH_PERIOD_DAYS)
                .about("Update authority can set the new epoch period in days.")
                .arg(
                    Arg::with_name("staking_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The staking info pubkey."),
                )
                .arg(
                    Arg::with_name("epoch_period_days")
                        .value_name("u16")
                        .required(true)
                        .takes_value(true)
                        .help("The new epoch period in days."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_UPDATE_MIN_STAKE_AMOUNT)
                .about("Update authority can set the new minimum stake amount.")
                .arg(
                    Arg::with_name("staking_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The staking info pubkey."),
                )
                .arg(
                    Arg::with_name("amount")
                        .value_name("f64")
                        .required(true)
                        .takes_value(true)
                        .help("The new minimum GGWP amount."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_UPDATE_HOLD_PERIOD_DAYS)
                .about("Update authority can set the new hold period in days.")
                .arg(
                    Arg::with_name("staking_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The staking info pubkey."),
                )
                .arg(
                    Arg::with_name("hold_period_days")
                        .value_name("u16")
                        .required(true)
                        .takes_value(true)
                        .help("The new hold period in days."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_UPDATE_HOLD_ROYALTY)
                .about("Update authority can set the new hold royalty in percent.")
                .arg(
                    Arg::with_name("staking_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The staking info pubkey."),
                )
                .arg(
                    Arg::with_name("hold_royalty")
                        .value_name("u8")
                        .required(true)
                        .takes_value(true)
                        .help("The new hold royalty in percent."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_UPDATE_ROYALTY)
                .about("Update authority can set the new royalty in percent.")
                .arg(
                    Arg::with_name("staking_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The staking info pubkey."),
                )
                .arg(
                    Arg::with_name("royalty")
                        .value_name("u8")
                        .required(true)
                        .takes_value(true)
                        .help("The new royalty in percent."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_STAKE)
                .about("User can stake the amount of GGWP.")
                .arg(
                    Arg::with_name("staking_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The staking info pubkey."),
                )
                .arg(
                    Arg::with_name("amount")
                        .value_name("f64")
                        .required(true)
                        .takes_value(true)
                        .help("The GGWP amount to stake."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_WITHDRAW)
                .about("User can withdraw the full amount of GGWP with rewards.")
                .arg(
                    Arg::with_name("staking_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The staking info pubkey."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_SHOW_STAKING_INFO)
                .about("Show the staking info data.")
                .arg(
                    Arg::with_name("staking_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The staking info pubkey."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_SHOW_USER_INFO)
                .about("Show the user info data.")
                .arg(
                    Arg::with_name("staking_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The staking info pubkey."),
                ),
        )
}
