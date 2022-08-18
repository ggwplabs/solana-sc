use anchor_client::{
    solana_sdk::{
        program_option::COption, program_pack::Pack, pubkey::Pubkey, signature::Keypair,
        signer::Signer,
    },
    ClientError, Program,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token::{
    instruction::initialize_mint,
    state::{Account as TokenAccount, Mint},
};

pub fn create_token_mint(
    program: &Program,
    mint_authority: Pubkey,
    decimals: u8,
) -> Result<Pubkey, ClientError> {
    let token_mint = Keypair::new();

    program
        .request()
        .instruction(initialize_mint(
            &spl_token::id(),
            &token_mint.pubkey(),
            &mint_authority,
            None,
            decimals,
        )?)
        .send()?;

    Ok(token_mint.pubkey())
}

pub fn get_token_mint_data(program: &Program, token_mint: Pubkey) -> Result<Mint, ClientError> {
    let account = program
        .rpc()
        .get_account_with_commitment(&token_mint, program.rpc().commitment())?
        .value
        .unwrap();

    let token_mint_data = Mint::unpack(&account.data).unwrap();
    return Ok(token_mint_data);
}

pub fn get_or_create_token_account(
    program: &Program,
    token_mint: Pubkey,
    owner: Pubkey,
) -> Result<Pubkey, ClientError> {
    let token_account = get_associated_token_address(&owner, &token_mint);

    if let Some(account) = program
        .rpc()
        .get_account_with_commitment(&token_account, program.rpc().commitment())?
        .value
    {
        let account_data = TokenAccount::unpack(&account.data).unwrap();
        if !(account.owner == spl_token::id()
            && account_data.owner == owner
            && account_data.mint == token_mint)
        {
            println!("TokenAccount {} was incorrectly initialized", token_account);
            return Err(ClientError::AccountNotFound);
        }
    } else {
        program
            .request()
            .instruction(create_associated_token_account(
                &program.payer(),
                &owner,
                &token_mint,
            ))
            .send()?;
    }

    Ok(token_account)
}
