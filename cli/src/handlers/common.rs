use super::utils::get_or_create_token_account;
use crate::commands;
use crate::handlers::utils::get_token_mint_data;
use anchor_client::anchor_lang::system_program;
use anchor_client::solana_sdk::program_option::COption;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::{solana_sdk::pubkey::Pubkey, Client, Program};
use anchor_client::{ClientError, Cluster};
use clap::value_t_or_exit;
use clap::{ArgMatches, Error};
use distribution::state::DistributionInfo;
use freezing::state::{FreezingInfo, RewardTableRow, GPASS_MINT_AUTH_SEED};
use gpass::state::GpassInfo;
use spl_token::amount_to_ui_amount;
use spl_token::state::Mint;
use staking::state::{StakingInfo, STAKING_FUND_AUTH_SEED};

pub fn handle(
    cmd_matches: &ArgMatches,
    client: &Client,
    cluster: &Cluster,
    gpass_program_id: Pubkey,
    freezing_program_id: Pubkey,
    staking_program_id: Pubkey,
    distribution_program_id: Pubkey,
) -> Result<(), Error> {
    let gpass_program = client.program(gpass_program_id);
    let freezing_program = client.program(freezing_program_id);
    let staking_program = client.program(staking_program_id);
    let distribution_program = client.program(distribution_program_id);

    let params = ProgramsParams::get_by_cluster(cluster);
    println!("Cluster: {}", cluster.url());
    println!("Params: {:?}", params);

    let (sub_command, arg_matches) = cmd_matches.subcommand();
    match (sub_command, arg_matches) {
        (commands::common::CMD_INIT_ALL, Some(arg_matches)) => {
            println!("Start initialize infrastructure");
            let update_auth = value_t_or_exit!(arg_matches, "update_auth", Pubkey);
            let ggwp_token = value_t_or_exit!(arg_matches, "ggwp_token", Pubkey);

            cmd_init_all(
                gpass_program,
                freezing_program,
                staking_program,
                distribution_program,
                params,
                update_auth,
                ggwp_token,
            )
            .expect("Init all command error");

            Ok(())
        }

        _ => {
            println!("{}", cmd_matches.usage());
            Ok(())
        }
    }
}

pub fn cmd_init_all(
    gpass_program: Program,
    freezing_program: Program,
    staking_program: Program,
    distribution_program: Program,
    params: ProgramsParams,
    update_auth: Pubkey,
    ggwp_token: Pubkey,
) -> Result<(), ClientError> {
    // TODO: deploy command
    // Initial checks
    assert_eq!(gpass::id(), gpass_program.id());
    assert_eq!(freezing::id(), freezing_program.id());
    assert_eq!(staking::id(), staking_program.id());
    assert_eq!(distribution::id(), distribution_program.id());

    let admin_pk = gpass_program.payer();
    assert_eq!(admin_pk, freezing_program.payer());
    assert_eq!(admin_pk, staking_program.payer());
    assert_eq!(admin_pk, distribution_program.payer());

    println!("Admin PK: {}", admin_pk);
    println!("Update authority: {}", update_auth);
    println!();

    let admin_balance_before = gpass_program.rpc().get_balance(&admin_pk)?;
    assert!(admin_balance_before > 10_000_000_000);

    let ggwp_token_data: Mint = get_token_mint_data(&gpass_program, ggwp_token)?;
    assert_eq!(ggwp_token_data.is_initialized, true);
    assert_eq!(ggwp_token_data.mint_authority, COption::Some(admin_pk));
    assert_eq!(ggwp_token_data.freeze_authority, COption::None);

    // Generate kps for infos
    let gpass_info = Keypair::new();
    let freezing_info = Keypair::new();
    let staking_info = Keypair::new();
    let distribution_info = Keypair::new();

    println!("New GPASS info PK: {}", gpass_info.pubkey());
    println!("New freezing info PK: {}", freezing_info.pubkey());
    println!("New staking info PK: {}", staking_info.pubkey());
    println!("New distribution info PK: {}", distribution_info.pubkey());
    println!();

    // Generate pks for authorities
    let (freezing_gpass_mint_auth, _) = Pubkey::find_program_address(
        &[
            GPASS_MINT_AUTH_SEED.as_bytes(),
            freezing_info.pubkey().as_ref(),
            gpass_info.pubkey().as_ref(),
        ],
        &freezing_program.id(),
    );
    println!("Freezing GPASS mint auth: {}", freezing_gpass_mint_auth);

    let (freezing_treasury_auth, _) = Pubkey::find_program_address(
        &[
            freezing::state::TREASURY_AUTH_SEED.as_bytes(),
            freezing_info.pubkey().as_ref(),
        ],
        &freezing_program.id(),
    );
    println!("Freezing treasury auth: {}", freezing_treasury_auth);

    let (staking_fund_auth, _) = Pubkey::find_program_address(
        &[
            STAKING_FUND_AUTH_SEED.as_bytes(),
            staking_info.pubkey().as_ref(),
        ],
        &staking_program.id(),
    );
    println!("Staking fund wallet auth: {}", staking_fund_auth);

    let (staking_treasury_auth, _) = Pubkey::find_program_address(
        &[
            staking::state::TREASURY_AUTH_SEED.as_bytes(),
            staking_info.pubkey().as_ref(),
        ],
        &staking_program.id(),
    );
    println!("Staking treasury auth: {}", staking_treasury_auth);

    let (accumulative_fund_auth, _) = Pubkey::find_program_address(
        &[
            distribution::state::ACCUMULATIVE_FUND_AUTH_SEED.as_bytes(),
            distribution_info.pubkey().as_ref(),
        ],
        &distribution_program.id(),
    );
    println!("Accumulative fund auth: {}", staking_treasury_auth);

    println!();

    // Init of get all token wallets (and funds)
    let freezing_treasury =
        get_or_create_token_account(&gpass_program, ggwp_token, freezing_treasury_auth)?;
    let staking_treasury =
        get_or_create_token_account(&gpass_program, ggwp_token, staking_treasury_auth)?;
    let accumulative_fund =
        get_or_create_token_account(&gpass_program, ggwp_token, accumulative_fund_auth)?;
    let staking_fund = get_or_create_token_account(&gpass_program, ggwp_token, staking_fund_auth)?;
    // TODO: temporary owners for wallets
    let play_to_earn_fund = get_or_create_token_account(&gpass_program, ggwp_token, admin_pk)?;
    let company_fund = get_or_create_token_account(&gpass_program, ggwp_token, admin_pk)?;
    let team_fund = get_or_create_token_account(&gpass_program, ggwp_token, admin_pk)?;

    println!(
        "Accumulative fund (owner: {}): {}",
        accumulative_fund_auth, accumulative_fund
    );
    println!(
        "Staking fund (owner: {}): {}",
        staking_fund_auth, staking_fund
    );
    println!(
        "Play to earn fund (owner: {}): {}",
        admin_pk, play_to_earn_fund
    );
    println!("Company fund (owner: {}): {}", admin_pk, company_fund);
    println!("Team fund (owner: {}): {}", admin_pk, team_fund);
    println!();

    // Init distribution with funds info
    distribution_program
        .request()
        .accounts(distribution::accounts::Initialize {
            admin: admin_pk,
            distribution_info: distribution_info.pubkey(),
            ggwp_token: ggwp_token,
            accumulative_fund: accumulative_fund,
            accumulative_fund_auth: accumulative_fund_auth,
            play_to_earn_fund: play_to_earn_fund,
            staking_fund: staking_fund,
            company_fund: company_fund,
            team_fund: team_fund,
            system_program: system_program::ID,
        })
        .args(distribution::instruction::Initialize {
            update_auth: update_auth,
            play_to_earn_fund_share: params.distribution.play_to_earn_fund_share,
            staking_fund_share: params.distribution.staking_fund_share,
            company_fund_share: params.distribution.company_fund_share,
            team_fund_share: params.distribution.team_fund_share,
        })
        .signer(&distribution_info)
        .send()?;
    let distribution_info_data: DistributionInfo =
        distribution_program.account(distribution_info.pubkey())?;
    println!("Distribution Initalized: {:?}", distribution_info_data);

    // Init GPASS with lists of minters and burners
    gpass_program
        .request()
        .accounts(gpass::accounts::Initialize {
            admin: admin_pk,
            gpass_info: gpass_info.pubkey(),
            system_program: system_program::ID,
        })
        .args(gpass::instruction::Initialize {
            burn_period: params.gpass.burn_period,
            update_auth: update_auth,
            minters: vec![freezing_gpass_mint_auth],
            burners: vec![],
        })
        .signer(&gpass_info)
        .send()?;
    let gpass_info_data: GpassInfo = gpass_program.account(gpass_info.pubkey())?;
    println!("GPASS Initalized: {:?}", gpass_info_data);

    // Init Freezing with gpass settings
    let reward_table = vec![
        RewardTableRow {
            ggwp_amount: 1000_000_000_000,
            gpass_amount: 5,
        },
        RewardTableRow {
            ggwp_amount: 2000_000_000_000,
            gpass_amount: 10,
        },
        RewardTableRow {
            ggwp_amount: 3000_000_000_000,
            gpass_amount: 15,
        },
        RewardTableRow {
            ggwp_amount: 4000_000_000_000,
            gpass_amount: 20,
        },
        RewardTableRow {
            ggwp_amount: 4800_000_000_000,
            gpass_amount: 25,
        },
    ];

    freezing_program
        .request()
        .accounts(freezing::accounts::Initialize {
            admin: admin_pk,
            freezing_info: freezing_info.pubkey(),
            gpass_mint_auth: freezing_gpass_mint_auth,
            treasury_auth: freezing_treasury_auth,
            ggwp_token: ggwp_token,
            gpass_info: gpass_info.pubkey(),
            accumulative_fund: accumulative_fund,
            treasury: freezing_treasury,
            system_program: system_program::ID,
            token_program: spl_token::id(),
        })
        .args(freezing::instruction::Initialize {
            update_auth: update_auth,
            reward_period: params.freezing.reward_period,
            royalty: params.freezing.royalty,
            unfreeze_royalty: params.freezing.unfreeze_royalty,
            unfreeze_lock_period: params.freezing.unfreeze_lock_period,
            reward_table: reward_table,
        })
        .signer(&freezing_info)
        .send()?;
    let freezing_info_data: FreezingInfo = freezing_program.account(freezing_info.pubkey())?;
    println!("Freezing info initialized: {:?}", freezing_info_data);

    // Init Staking
    staking_program
        .request()
        .accounts(staking::accounts::Initialize {
            admin: admin_pk,
            staking_info: staking_info.pubkey(),
            ggwp_token: ggwp_token,
            accumulative_fund: accumulative_fund,
            treasury: staking_treasury,
            treasury_auth: staking_treasury_auth,
            staking_fund: staking_fund,
            staking_fund_auth: staking_fund_auth,
            system_program: system_program::ID,
        })
        .args(staking::instruction::Initialize {
            update_auth: update_auth,
            epoch_period_days: params.staking.epoch_period_days,
            min_stake_amount: params.staking.min_stake_amount,
            hold_period_days: params.staking.hold_period_days,
            hold_royalty: params.staking.hold_royalty,
            royalty: params.staking.royalty,
            apr_start: params.staking.apr_start,
            apr_step: params.staking.apr_step,
            apr_end: params.staking.apr_end,
        })
        .signer(&staking_info)
        .send()?;
    let staking_info_data: StakingInfo = staking_program.account(staking_info.pubkey())?;
    println!("Staking info initialized: {:?}", staking_info_data);

    println!();

    // Calc balance diff
    let admin_balance_after = gpass_program.rpc().get_balance(&admin_pk)?;
    println!(
        "Spent SOL: {}",
        amount_to_ui_amount(admin_balance_before - admin_balance_after, 9)
    );

    Ok(())
}

#[derive(Debug)]
pub struct ProgramsParams {
    pub distribution: DistributionParams,
    pub gpass: GPASSParams,
    pub freezing: FreezingParams,
    pub staking: StakingParams,
}

#[derive(Debug)]
pub struct DistributionParams {
    pub play_to_earn_fund_share: u8,
    pub staking_fund_share: u8,
    pub company_fund_share: u8,
    pub team_fund_share: u8,
}

#[derive(Debug)]
pub struct GPASSParams {
    pub burn_period: u64,
}

#[derive(Debug)]
pub struct FreezingParams {
    pub reward_period: i64,
    pub royalty: u8,
    pub unfreeze_royalty: u8,
    pub unfreeze_lock_period: i64,
}

#[derive(Debug)]
pub struct StakingParams {
    pub epoch_period_days: u16,
    pub min_stake_amount: u64,
    pub hold_period_days: u16,
    pub hold_royalty: u8,
    pub royalty: u8,
    pub apr_start: u8,
    pub apr_step: u8,
    pub apr_end: u8,
}

impl ProgramsParams {
    pub fn get_by_cluster(cluster: &Cluster) -> Self {
        match cluster {
            Cluster::Devnet => ProgramsParams {
                distribution: DistributionParams {
                    play_to_earn_fund_share: 45,
                    staking_fund_share: 40,
                    company_fund_share: 5,
                    team_fund_share: 10,
                },
                gpass: GPASSParams {
                    burn_period: 1 * 24 * 60 * 60,
                },
                freezing: FreezingParams {
                    reward_period: 6 * 60 * 60,
                    royalty: 8,
                    unfreeze_royalty: 15,
                    unfreeze_lock_period: 3 * 24 * 60 * 60,
                },
                staking: StakingParams {
                    epoch_period_days: 5,
                    min_stake_amount: 3000_000_000_000,
                    hold_period_days: 2,
                    hold_royalty: 15,
                    royalty: 8,
                    apr_start: 45,
                    apr_step: 1,
                    apr_end: 5,
                },
            },
            Cluster::Testnet => ProgramsParams {
                distribution: DistributionParams {
                    play_to_earn_fund_share: 45,
                    staking_fund_share: 40,
                    company_fund_share: 5,
                    team_fund_share: 10,
                },
                gpass: GPASSParams {
                    burn_period: 1 * 24 * 60 * 60,
                },
                freezing: FreezingParams {
                    reward_period: 6 * 60 * 60,
                    royalty: 8,
                    unfreeze_royalty: 15,
                    unfreeze_lock_period: 3 * 24 * 60 * 60,
                },
                staking: StakingParams {
                    epoch_period_days: 5,
                    min_stake_amount: 3000_000_000_000,
                    hold_period_days: 2,
                    hold_royalty: 15,
                    royalty: 8,
                    apr_start: 45,
                    apr_step: 1,
                    apr_end: 5,
                },
            },
            Cluster::Mainnet => ProgramsParams {
                distribution: DistributionParams {
                    play_to_earn_fund_share: 45,
                    staking_fund_share: 40,
                    company_fund_share: 5,
                    team_fund_share: 10,
                },
                gpass: GPASSParams {
                    burn_period: 30 * 24 * 60 * 60,
                },
                freezing: FreezingParams {
                    reward_period: 24 * 60 * 60,
                    royalty: 8,
                    unfreeze_royalty: 15,
                    unfreeze_lock_period: 15 * 24 * 60 * 60,
                },
                staking: StakingParams {
                    epoch_period_days: 45,
                    min_stake_amount: 3000_000_000_000,
                    hold_period_days: 30,
                    hold_royalty: 15,
                    royalty: 8,
                    apr_start: 45,
                    apr_step: 1,
                    apr_end: 5,
                },
            },
            _ => panic!("Bad cluster"),
        }
    }
}
