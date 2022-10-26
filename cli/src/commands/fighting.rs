use super::CMDS_FIGHTING;
use clap::{App, AppSettings, Arg, SubCommand};
use solana_clap_utils::input_validators::is_valid_pubkey;

pub const CMD_INITIALIZE: &str = "initialize";
pub const CMD_UPDATE_ADMIN: &str = "update-admin";
pub const CMD_SET_UPDATE_AUTHORITY: &str = "set-update-authority";
pub const CMD_UPDATE_AFK_TIMEOUT: &str = "update-afk-timeout";
pub const CMD_SHOW_SETTINGS: &str = "show-settings";
pub const CMD_UPDATE_VALIDATOR: &str = "update-validator";
pub const CMD_SHOW_GAME_INFO: &str = "show-game-info";
pub const CMD_SHOW_USER_INFO: &str = "show-user-info";

pub fn get_fighting_commands<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CMDS_FIGHTING)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("Fighting smart contract commands.")
        .subcommand(
            SubCommand::with_name(CMD_INITIALIZE)
                .about("Initialize the fighting contract")
                .arg(
                    Arg::with_name("update_auth")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The fighting info update authority pubkey."),
                )
                .arg(
                    Arg::with_name("validator")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The validator pubkey."),
                )
                .arg(
                    Arg::with_name("gpass_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The GPASS info pubkey."),
                )
                .arg(
                    Arg::with_name("reward_distribution_info")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The Reward Distribution info pubkey."),
                )
                .arg(
                    Arg::with_name("afk_timeout")
                        .value_name("i64")
                        .required(true)
                        .takes_value(true)
                        .help("The AFK timeout in seconds."),
                )
                .arg(
                    Arg::with_name("royalty")
                        .value_name("u8")
                        .required(true)
                        .takes_value(true)
                        .help("The royalty in percent."),
                )
                .arg(
                    Arg::with_name("reward_coefficient")
                        .value_name("u32")
                        .required(true)
                        .takes_value(true)
                        .help("The reward coefficient value."),
                )
                .arg(
                    Arg::with_name("gpass_daily_reward_coefficient")
                        .value_name("u32")
                        .required(true)
                        .takes_value(true)
                        .help("The gpass daily reward coefficient value."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_UPDATE_ADMIN)
                .about("Admin can set the new admin.")
                .arg(
                    Arg::with_name("fighting_settings")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The fighting settings account address."),
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
                    Arg::with_name("fighting_settings")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The fightings settings account address."),
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
            SubCommand::with_name(CMD_UPDATE_AFK_TIMEOUT)
                .about("Update authority can set the new afk timeout in seconds.")
                .arg(
                    Arg::with_name("fighting_settings")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The fighting settings account address."),
                )
                .arg(
                    Arg::with_name("afk_timeout")
                        .value_name("i64")
                        .required(true)
                        .takes_value(true)
                        .help("The new AFK timeout in seconds."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_UPDATE_VALIDATOR)
                .about("Update authority can set the new validator pk.")
                .arg(
                    Arg::with_name("fighting_settings")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The fighting settings account address."),
                )
                .arg(
                    Arg::with_name("validator")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The validator PK."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_SHOW_SETTINGS)
                .about("Show fighting settings info.")
                .arg(
                    Arg::with_name("fighting_settings")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The fighting settings account address."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_SHOW_GAME_INFO)
                .about("Show game info.")
                .arg(
                    Arg::with_name("fighting_settings")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The fighting settings account address."),
                )
                .arg(
                    Arg::with_name("user")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The player address."),
                )
                .arg(
                    Arg::with_name("game_id")
                        .value_name("u64")
                        .required(true)
                        .takes_value(true)
                        .help("The game id."),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_SHOW_USER_INFO)
                .about("Show user info data.")
                .arg(
                    Arg::with_name("fighting_settings")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The fighting settings account address."),
                )
                .arg(
                    Arg::with_name("user")
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .takes_value(true)
                        .help("The player address."),
                ),
        )
}
