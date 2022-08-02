use anchor_lang::prelude::*;
use context::*;

mod context;
mod error;
mod state;

declare_id!("ABHUowgjyTkmbMRRuMYJ5ui4wAz6Z6HE4PQMHy9YqMrQ");

#[program]
pub mod freezing {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // TODO
        Ok(())
    }

    pub fn change_admin(ctx: Context<ChangeAdmin>) -> Result<()> {
        // TODO
        Ok(())
    }

    pub fn change_params(ctx: Context<ChangeParams>) -> Result<()> {
        // TODO
        Ok(())
    }

    pub fn freeze(ctx: Context<Freeze>, amount: u64) -> Result<()> {
        // TODO
        Ok(())
    }

    pub fn unfreeze(ctx: Context<Unfreeze>, amount: u64) -> Result<()> {
        // TODO
        Ok(())
    }

    // TODO: viewers
}
