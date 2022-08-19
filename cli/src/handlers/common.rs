use super::utils::get_or_create_token_account;
use crate::commands;
use crate::handlers::utils::get_token_mint_data;
use anchor_client::anchor_lang::system_program;
use anchor_client::solana_sdk::program_option::COption;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::ClientError;
use anchor_client::{solana_sdk::pubkey::Pubkey, Client, Program};
use clap::value_t_or_exit;
use clap::{ArgMatches, Error};
use freezing::state::{FreezingInfo, RewardTableRow, GPASS_MINT_AUTH_SEED, TREASURY_AUTH_SEED};
use gpass::state::GpassInfo;
use spl_token::amount_to_ui_amount;
use spl_token::state::Mint;

pub fn handle(
    cmd_matches: &ArgMatches,
    client: &Client,
    gpass_program_id: Pubkey,
    freezing_program_id: Pubkey,
    staking_program_id: Pubkey,
) -> Result<(), Error> {
    let gpass_program = client.program(gpass_program_id);
    let freezing_program = client.program(freezing_program_id);
    let staking_program = client.program(staking_program_id);

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
    update_auth: Pubkey,
    ggwp_token: Pubkey,
) -> Result<(), ClientError> {
    // Initial checks
    assert_eq!(gpass::id(), gpass_program.id());
    assert_eq!(freezing::id(), freezing_program.id());
    assert_eq!(staking::id(), staking_program.id());

    let admin_pk = gpass_program.payer();
    assert_eq!(admin_pk, freezing_program.payer());
    assert_eq!(admin_pk, staking_program.payer());

    println!("Admin PK: {}", admin_pk);
    println!("Update authority: {}", update_auth);

    let admin_balance_before = gpass_program.rpc().get_balance(&admin_pk)?;
    assert!(admin_balance_before > 10_000_000_000); // TODO: calc amount for full init

    let ggwp_token_data: Mint = get_token_mint_data(&gpass_program, ggwp_token)?;
    assert_eq!(ggwp_token_data.is_initialized, true);
    assert_eq!(ggwp_token_data.mint_authority, COption::Some(admin_pk));
    assert_eq!(ggwp_token_data.freeze_authority, COption::None);

    // Generate kps for infos
    let gpass_info = Keypair::new();
    let freezing_info = Keypair::new();
    let staking_info = Keypair::new();

    println!("New GPASS info PK: {}", gpass_info.pubkey());
    println!("New freezing info PK: {}", freezing_info.pubkey());
    println!("New staking info PK: {}", staking_info.pubkey());

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
            TREASURY_AUTH_SEED.as_bytes(),
            freezing_info.pubkey().as_ref(),
        ],
        &freezing_program.id(),
    );
    println!("Freezing treasury auth: {}", freezing_treasury_auth);

    // Init of get all token wallets (and funds)
    let freezing_treasury =
        get_or_create_token_account(&gpass_program, ggwp_token, freezing_treasury_auth)?;
    let accumulative_fund = get_or_create_token_account(&gpass_program, ggwp_token, admin_pk)?;

    println!("Accumulative fund: {}", accumulative_fund);

    // Init GPASS with lists of minters and burners
    gpass_program
        .request()
        .accounts(gpass::accounts::Initialize {
            admin: admin_pk,
            gpass_info: gpass_info.pubkey(),
            system_program: system_program::ID,
        })
        .args(gpass::instruction::Initialize {
            burn_period: 30 * 24 * 60 * 60, // 30 days
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
            reward_period: 24 * 60 * 60,
            royalty: 8,
            unfreeze_royalty: 15,
            unfreeze_lock_period: 15 * 24 * 60 * 60, // 15 days
            reward_table: reward_table,
        })
        .signer(&freezing_info)
        .send()?;
    let freezing_info_data: FreezingInfo = freezing_program.account(freezing_info.pubkey())?;
    println!("Freezing info initialized: {:?}", freezing_info_data);

    // Init Staking

    // Calc balance diff
    let admin_balance_after = gpass_program.rpc().get_balance(&admin_pk)?;
    println!(
        "Spent SOL: {}",
        amount_to_ui_amount(admin_balance_before - admin_balance_after, 0)
    );

    Ok(())
}
