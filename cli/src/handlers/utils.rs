use anchor_client::{
    solana_sdk::{program_pack::Pack, pubkey::Pubkey},
    ClientError, Program,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token::state::Account as TokenAccount;

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
