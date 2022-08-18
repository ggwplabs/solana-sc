use super::CMDS_STAKING;
use clap::{App, AppSettings, Arg, SubCommand};
use solana_clap_utils::input_validators::is_valid_pubkey;

pub const CMD_INITIALIZE: &str = "initialize";

pub fn get_staking_commands<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CMDS_STAKING)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("Staking smart contract commands.")
        .subcommand(
            SubCommand::with_name(CMD_INITIALIZE)
                .about("Initialize the staking contract")
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
                        .value_name("u64")
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
}
