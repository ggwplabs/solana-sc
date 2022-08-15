use super::CMDS_FREEZING;
use clap::{App, AppSettings, Arg, SubCommand};
use solana_clap_utils::input_validators::is_valid_pubkey;

pub const CMD_INITIALIZE: &str = "initialize";
pub const CMD_UPDATE_ADMIN: &str = "update-admin";
pub const CMD_SET_UPDATE_AUTHORITY: &str = "set-update-authority";
pub const CMD_UPDATE_ROYALTY: &str = "update-royalty";
pub const CMD_UPDATE_UNFREEZE_ROYALTY: &str = "update-unfreeze-royalty";
pub const CMD_UPDATE_REWARD_TABLE: &str = "update-reward-table";
pub const CMD_UPDATE_REWARD_PERIOD: &str = "update-reward-period";
pub const CMD_UPDATE_UNFREEZE_LOCK_PERIOD: &str = "update-unfreeze-lock-period";
pub const CMD_FREEZE: &str = "freeze";
pub const CMD_WITHDRAW_GPASS: &str = "withdraw-gpass";
pub const CMD_UNFREEZE: &str = "unfreeze";
pub const CMD_SHOW_PARAMS: &str = "show-params";
pub const CMD_SHOW_USER_INFO: &str = "show-user-info";

pub fn get_freezing_commands<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CMDS_FREEZING)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("Freezing smart contract commands.")
        .subcommand(
            SubCommand::with_name(CMD_INITIALIZE)
                .about("Initialize the freezing contract")
                .arg(
                    Arg::with_name("update_auth")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The params update authority pubkey."),
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
                    Arg::with_name("gpass_settings")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The GPASS settings account pubkey."),
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
                    Arg::with_name("reward_period")
                        .value_name("u64")
                        .required(true)
                        .takes_value(true)
                        .help("The reward period value in seconds."),
                )
                .arg(
                    Arg::with_name("royalty")
                        .value_name("u8")
                        .required(true)
                        .takes_value(true)
                        .help("The royalty value in percent."),
                )
                .arg(
                    Arg::with_name("unfreeze_royalty")
                        .value_name("u8")
                        .required(true)
                        .takes_value(true)
                        .help("The unfreeze royalty in percent."),
                )
                .arg(
                    Arg::with_name("unfreeze_lock_period")
                        .value_name("u64")
                        .required(true)
                        .takes_value(true)
                        .help("The unfreeze lock period value in seconds."),
                )
                .arg(
                    Arg::with_name("reward_table_ggwp")
                        .long("ggwp")
                        .value_name("u64")
                        .multiple(true)
                        .required(true)
                        .takes_value(true)
                        .help("The part of reward table row: (GGWP AMOUNT, GPASS AMOUNT)."),
                )
                .arg(
                    Arg::with_name("reward_table_gpass")
                        .long("gpass")
                        .value_name("u64")
                        .multiple(true)
                        .required(true)
                        .takes_value(true)
                        .help("The part of reward table row: (GGWP AMOUNT, GPASS AMOUNT)."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_UPDATE_ADMIN)
                .about("Admin can set the new admin of freezing.")
                .arg(
                    Arg::with_name("params")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The freezing params account address."),
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
                .about("Admin can set the new update authority of freezing.")
                .arg(
                    Arg::with_name("params")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The freezing params account address."),
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
            SubCommand::with_name(CMD_UPDATE_ROYALTY)
                .about("Update authority can set the new royalty value in percent.")
                .arg(
                    Arg::with_name("params")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The freezing params account address."),
                )
                .arg(
                    Arg::with_name("royalty")
                        .value_name("u8")
                        .required(true)
                        .takes_value(true)
                        .help("The new royalty value."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_UPDATE_UNFREEZE_ROYALTY)
                .about("Update authority can set the new unfreeze royalty value in percent.")
                .arg(
                    Arg::with_name("params")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The freezing params account address."),
                )
                .arg(
                    Arg::with_name("unfreeze_royalty")
                        .value_name("u8")
                        .required(true)
                        .takes_value(true)
                        .help("The new unfreeze royalty value."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_UPDATE_UNFREEZE_LOCK_PERIOD)
                .about("Update authority can set the new unfreeze lock period value in seconds.")
                .arg(
                    Arg::with_name("params")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The freezing params account address."),
                )
                .arg(
                    Arg::with_name("unfreeze_lock_period")
                        .value_name("u64")
                        .required(true)
                        .takes_value(true)
                        .help("The new unfreeze lock period value in second."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_UPDATE_REWARD_TABLE)
                .about("Update authority can set the new reward table.")
                .arg(
                    Arg::with_name("params")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The freezing params account address."),
                )
                .arg(
                    Arg::with_name("reward_table_ggwp")
                        .long("ggwp")
                        .value_name("u64")
                        .multiple(true)
                        .required(true)
                        .takes_value(true)
                        .help("The part of reward table row: (GGWP AMOUNT, GPASS AMOUNT)."),
                )
                .arg(
                    Arg::with_name("reward_table_gpass")
                        .long("gpass")
                        .value_name("u64")
                        .multiple(true)
                        .required(true)
                        .takes_value(true)
                        .help("The part of reward table row: (GGWP AMOUNT, GPASS AMOUNT)."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_UPDATE_REWARD_PERIOD)
                .about("Update authority can set the new reward period value in seconds.")
                .arg(
                    Arg::with_name("params")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The freezing params account address."),
                )
                .arg(
                    Arg::with_name("reward_period")
                        .value_name("u64")
                        .required(true)
                        .takes_value(true)
                        .help("The new reward period value in second."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_FREEZE)
                .about("User can freeze the amount of GGWP for getting rewards in GPASS.")
                .arg(
                    Arg::with_name("params")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The freezing params account address."),
                )
                .arg(
                    Arg::with_name("amount")
                        .value_name("ui_amount (f64)")
                        .required(true)
                        .takes_value(true)
                        .help("The amount to freeze."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_WITHDRAW_GPASS)
                .about("User can withdraw earned GPASS in every time.")
                .arg(
                    Arg::with_name("params")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The freezing params account address."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_UNFREEZE)
                .about("User can unfreeze full amount of freezed GGWP.")
                .arg(
                    Arg::with_name("params")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The freezing params account address."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_SHOW_PARAMS)
                .about("Show freezing params.")
                .arg(
                    Arg::with_name("params")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The freezing params account address."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_SHOW_USER_INFO)
                .about("Show the user info account.")
                .arg(
                    Arg::with_name("params")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The freezing params account address."),
                )
                .arg(
                    Arg::with_name("user")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The user system account address."),
                ),
        )
}
